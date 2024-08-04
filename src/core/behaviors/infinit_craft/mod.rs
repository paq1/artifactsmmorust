use crate::core::behaviors::crafting::CraftingBehavior;
use crate::core::behaviors::deposit_bank::DepositBankBehavior;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::behaviors::withdraw_bank::WithdrawBankBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct InfinitCraftBehavior {
    pub current_state: String,
    pub moving_behavior: MovingBehavior,
    pub deposit_bank_behavior: DepositBankBehavior,
    pub withdraw_bank_behavior: WithdrawBankBehavior,
    pub crafting_behavior: CraftingBehavior
}

impl InfinitCraftBehavior {
    pub fn new(
        moving_behavior: MovingBehavior,
        deposit_bank_behavior: DepositBankBehavior,
        withdraw_bank_behavior: WithdrawBankBehavior,
        crafting_behavior: CraftingBehavior
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
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
        ingredient: (&str, u32),
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
                let (code, _) = ingredient;
                let next_withdraw_item = self.withdraw_bank_behavior.next_behavior(player, code, None).await?;
                if next_withdraw_item.current_state.as_str() == "finish" {
                    Ok(
                        InfinitCraftBehavior {
                            current_state: "withdraw_all".to_string(),
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
            // "withdraw_all" => {
            //     let next_moving_behavior = self.moving_behavior.next_behavior(player, craft_position).await?;
            //     if next_moving_behavior.current_state.as_str() == "finish" {
            //         Ok(
            //             InfinitCraftBehavior {
            //                 current_state: "in_craft_zone".to_string(),
            //                 moving_behavior: self.moving_behavior.reset(),
            //                 ..self.clone()
            //             }
            //         )
            //     }
            //     else {
            //         Ok(
            //             InfinitCraftBehavior {
            //                 moving_behavior: next_moving_behavior,
            //                 ..self.clone()
            //             }
            //         )
            //     }
            // }
            "withdraw_all" => {
                let next_crafting_behavior = self.crafting_behavior.next_behavior(player, craft_position, &vec![ingredient], craft_result_code, None).await?;
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
}
