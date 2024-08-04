use std::sync::Arc;

use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_withdraw_item::CanWithdrawItem;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct WithdrawBankBehavior {
    pub current_state: String,
    pub can_withdraw_item: Arc<Box<dyn CanWithdrawItem>>,
    pub moving_behavior: MovingBehavior,
}

impl WithdrawBankBehavior {
    pub fn new(
        can_withdraw_item: Arc<Box<dyn CanWithdrawItem>>,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_withdraw_item,
            moving_behavior,
        }
    }

    pub fn reset(&self) -> Self {
        Self {
            current_state: "empty".to_string(),
            moving_behavior: self.moving_behavior.reset(),
            ..self.clone()
        }
    }

    pub async fn next_behavior(
        &self,
        character_info: &Character, // fixme passer le code et la quantite
        item_code: &str,
        quantity: Option<u32>, // if None, take all
    ) -> Result<WithdrawBankBehavior, Error> {
        let cooldown = character_info.cooldown_sec();

        let bank_position = Position::new(4, 1);

        match self.current_state.as_str() {
            _ if cooldown >= 0 => {
                println!("[{}] in cooldown for {} secs", character_info.name, cooldown);
                Ok(self.clone())
            }
            "empty" => {
                let new_moving_behavior = self.moving_behavior.next_behavior(&character_info, &bank_position).await?;
                if new_moving_behavior.current_state == "finish" {
                    Ok(
                        WithdrawBankBehavior {
                            current_state: "in_bank".to_string(),
                            moving_behavior: self.moving_behavior.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    // comportement non terminer
                    Ok(
                        WithdrawBankBehavior {
                            moving_behavior: new_moving_behavior,
                            ..self.clone()
                        }
                    )

                }
            }
            "in_bank" => {
                match self.can_withdraw_item.withdraw(&character_info, &item_code.to_string(), quantity.unwrap_or(character_info.get_quantity_available())).await {
                    Ok(_) => {
                        println!("[{}] - withdraw ok ", character_info.name);
                        Ok(
                            WithdrawBankBehavior {
                                current_state: "finish".to_string(),
                                ..self.clone()
                            }
                        )
                    }
                    Err(e) => {
                        println!("[{}] - can withdraw item in error : {e:?}", character_info.name);
                        Ok(self.clone())
                    } // on laisse le meme etat certainement un erreur cote serveur
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
