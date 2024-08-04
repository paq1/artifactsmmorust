use std::sync::Arc;

use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_craft::CanCraft;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct CraftingBehavior {
    pub current_state: String,
    pub can_craft: Arc<Box<dyn CanCraft>>,
    pub moving_behavior: MovingBehavior,
}

impl CraftingBehavior {
    pub fn new(
        can_craft: Arc<Box<dyn CanCraft>>,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_craft,
            moving_behavior,
        }
    }

    pub fn reset(&self) -> Self {
        Self {
            current_state: "empty".to_string(),
            ..self.clone()
        }
    }

    pub async fn next_behavior(
        &self,
        character_info: &Character,
        craft_position: &Position,
        craft_costs: &Vec<(&str, u32)>, // vec![("ash_wood", 6)]
        code_item: &str,
        maybe_quantity: Option<u32>, // if None craft max
    ) -> Result<CraftingBehavior, Error> {
        let cooldown = character_info.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown >= 0 => {
                println!("[{}] in cooldown for {} secs", character_info.name, cooldown);
                Ok(self.clone())
            }
            "empty" => {
                let new_moving_behavior = self.moving_behavior.next_behavior(&character_info, &craft_position).await?;
                if new_moving_behavior.current_state == "finish" {
                    Ok(
                        CraftingBehavior {
                            current_state: "in_craft_position".to_string(),
                            moving_behavior: self.moving_behavior.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    // comportement non terminer
                    Ok(
                        CraftingBehavior {
                            moving_behavior: new_moving_behavior,
                            ..self.clone()
                        }
                    )
                }
            }
            "in_craft_position" => {
                match maybe_quantity {
                    Some(quantity) => {
                        self.crafting_from_quantity(character_info, code_item, quantity).await
                    }
                    None => {
                        println!("[{}] try crafting {} but dont give quantity. So, quantity will be calculate.", character_info.name, code_item);
                        let ingredient_names = craft_costs.iter()
                            .map(|(name, _)| {
                                name.to_string()
                            })
                            .collect::<Vec<_>>();

                        let items_required_in_inventory = character_info.inventory
                            .iter()
                            .filter(|slot| {
                                ingredient_names.contains(&slot.code)
                            })
                            .map(|slot| (slot.clone().code, slot.clone().quantity))
                            .collect::<Vec<_>>();

                        if craft_costs.len() != items_required_in_inventory.len() {
                            println!("[{}] - craft failed because ingredients are missing in inventory, craft is finish", character_info.name);
                            Ok(
                                CraftingBehavior {
                                    current_state: "finish".to_string(),
                                    ..self.clone()
                                }
                            )
                        } else {
                            let quantity_computed = craft_costs
                                .into_iter()
                                .map(|(code, quantity)| {
                                    let quantity_max = items_required_in_inventory
                                        .iter()
                                        .find(|(code_inventory, _)| code_inventory.clone() == code.to_string())
                                        .map(|(_, q)| q / quantity.clone())
                                        .unwrap_or(0);
                                    quantity_max
                                })
                                .min()
                                .unwrap_or(0);

                            self.crafting_from_quantity(character_info, code_item, quantity_computed).await
                        }
                    }
                }
            }
            _ => {
                Err(
                    Error::Simple("invalid transition".to_string())
                )
            }
        }
    }

    async fn crafting_from_quantity(
        &self,
        character_info: &Character,
        code_item: &str,
        quantity: u32, // if None craft max
    ) -> Result<CraftingBehavior, Error> {
        match self.can_craft.craft(character_info, &code_item.to_string(), quantity).await {
            Ok(_) => {
                println!("[{}] - craft succeed.", character_info.name);
                Ok(
                    CraftingBehavior {
                        current_state: "finish".to_string(),
                        ..self.clone()
                    }
                )
            }
            Err(e) => {
                match e {
                    Error::WithCode(error_code) => {
                        if error_code.status.unwrap_or(0) == 478 {
                            println!("[{}] - craft failed because inventory is full. craft is finish", character_info.name);
                            Ok(
                                CraftingBehavior {
                                    current_state: "finish".to_string(),
                                    ..self.clone()
                                }
                            )
                        } else {
                            println!("[{}] - craft failed due to a not catch error. error code is {}", character_info.name, error_code.status.unwrap_or(0));
                            Ok(
                                self.clone()
                            )
                        }
                    }
                    Error::Simple(error_title) => {
                        println!("[{}] - craft failed due to a not catch error. {error_title}", character_info.name);
                        Ok(
                            self.clone()
                        )
                    }
                }
            }
        }
    }
}
