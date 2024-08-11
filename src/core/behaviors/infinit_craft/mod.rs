use std::sync::Arc;
use crate::core::behaviors::crafting::CraftingBehavior;
use crate::core::behaviors::deposit_bank::DepositBankBehavior;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::behaviors::withdraw_bank::WithdrawBankBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_get_bank::CanGetBank;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct InfinitCraftBehavior {
    pub current_state: String,
    pub can_get_bank: Arc<Box<dyn CanGetBank>>,
    pub moving_behavior: MovingBehavior,
    pub deposit_bank_behavior: DepositBankBehavior,
    pub withdraw_bank_behavior: WithdrawBankBehavior,
    pub crafting_behavior: CraftingBehavior
}

impl InfinitCraftBehavior {
    pub fn new(
        can_get_bank: Arc<Box<dyn CanGetBank>>,
        moving_behavior: MovingBehavior,
        deposit_bank_behavior: DepositBankBehavior,
        withdraw_bank_behavior: WithdrawBankBehavior,
        crafting_behavior: CraftingBehavior
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_get_bank,
            moving_behavior,
            deposit_bank_behavior,
            withdraw_bank_behavior,
            crafting_behavior,
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
        player: &Character,
        bank_position: &Position,
        craft_position: &Position,
        ingredients: &Vec<(&str, u32)>,
        craft_result_code: &str,
    ) -> Result<InfinitCraftBehavior, Error> {
        let cooldown_sec = player.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown_sec >= 0 => {
                println!("[{}] in cooldown for {cooldown_sec} secs", player.name);
                Ok(self.clone())
            }
            "empty" => {
                let next_moving_behavior = self.moving_behavior.next_behavior(player, bank_position).await?;
                if next_moving_behavior.current_state.as_str() == "finish" {
                    Ok(
                        InfinitCraftBehavior {
                            current_state: "in_bank".to_string(),
                            moving_behavior: self.moving_behavior.reset(),
                            ..self.clone()
                        }
                    )
                }
                else {
                    Ok(
                        InfinitCraftBehavior {
                            moving_behavior: next_moving_behavior,
                            ..self.clone()
                        }
                    )
                }
            }
            "in_bank" => {
                let next_deposit_all = self.deposit_bank_behavior.next_behavior(player).await?;
                if next_deposit_all.current_state.as_str() == "finish" {
                    Ok(
                        InfinitCraftBehavior {
                            current_state: "deposit_all".to_string(),
                            deposit_bank_behavior: self.deposit_bank_behavior.reset(),
                            ..self.clone()
                        }
                    )
                }
                else {
                    Ok(
                        InfinitCraftBehavior {
                            deposit_bank_behavior: next_deposit_all,
                            ..self.clone()
                        }
                    )
                }
            }
            "deposit_all" => {

                let global_quantity_for_one_recipe: u32 = ingredients
                    .into_iter().map(|(_, quantity)| quantity)
                    .sum();

                let taille_max_inventaire = player.inventory_max_items;

                let mandatories = ingredients
                    .iter().map(|(code, quantity)| {
                    (*code, taille_max_inventaire * quantity / global_quantity_for_one_recipe)
                }).collect::<Vec<_>>();

                let inventory_codes_only: Vec<String> = player.inventory.iter().map(|s| s.code.clone()).collect::<Vec<_>>();

                let maybe_ingredient_missing = mandatories
                    .iter()
                    .find(|(code, _)| {
                        !inventory_codes_only.contains(&code.to_string())
                    })
                    .map(|(code, quantity)| (*code, *quantity));

                if !self.can_take_in_bank(&mandatories).await {
                    println!("[{}] - pas assez d'ingredients pour craft", player.name);
                    Ok(
                        InfinitCraftBehavior {
                            current_state: "finish".to_string(),
                            withdraw_bank_behavior: self.withdraw_bank_behavior.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    match maybe_ingredient_missing {
                        Some((code, quantity)) => {
                            let next_withdraw_item = self.withdraw_bank_behavior.next_behavior(player, code, Some(quantity)).await?;
                            if next_withdraw_item.current_state.as_str() == "finish" {
                                Ok(
                                    InfinitCraftBehavior {
                                        current_state: "deposit_all".to_string(),
                                        withdraw_bank_behavior: self.withdraw_bank_behavior.reset(),
                                        ..self.clone()
                                    }
                                )
                            }
                            else {
                                Ok(
                                    InfinitCraftBehavior {
                                        withdraw_bank_behavior: next_withdraw_item,
                                        ..self.clone()
                                    }
                                )
                            }
                        }
                        None => {
                            Ok(
                                InfinitCraftBehavior {
                                    current_state: "withdraw_all".to_string(),
                                    withdraw_bank_behavior: self.withdraw_bank_behavior.reset(),
                                    ..self.clone()
                                }
                            )
                        }
                    }
                }
            }
            "withdraw_all" => {
                let next_crafting_behavior = self.crafting_behavior.next_behavior(player, craft_position, ingredients, craft_result_code, None).await?;
                if next_crafting_behavior.current_state.as_str() == "finish" {
                    Ok(
                        InfinitCraftBehavior {
                            current_state: "empty".to_string(),
                            crafting_behavior: self.crafting_behavior.reset(),
                            ..self.clone()
                        }
                    )
                }
                else {
                    Ok(
                        InfinitCraftBehavior {
                            crafting_behavior: next_crafting_behavior,
                            ..self.clone()
                        }
                    )
                }
            }
            _ => {
                Err(
                    Error::Simple("invalid transition".to_string())
                )
            }
        }
    }


    pub async fn can_take_in_bank(&self, mandatorie: &Vec<(&str, u32)>) -> bool {
        match self.can_get_bank.get_items().await {
            Ok(table) => {
                let maybevalid = table.iter().map(|(bank_code, bank_quantity)| {
                    let maybe_item_found = mandatorie.into_iter().find(|(mandatory_code, mandatory_quantity)| {
                        *mandatory_code == bank_code.as_str() && mandatory_quantity <= bank_quantity
                    });
                    match maybe_item_found {
                        Some(_) => true,
                        None => false
                    }
                }).collect::<Vec<_>>();

                !maybevalid.contains(&false)
            }
            Err(_) => false
        }
    }
}
