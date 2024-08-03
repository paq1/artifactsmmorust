use std::sync::Arc;

use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct DepositBankBehavior {
    pub current_state: String,
    pub can_deposit_item: Arc<Box<dyn CanDepositItem>>,
    pub moving_behavior: MovingBehavior,
}

impl DepositBankBehavior {
    pub fn new(
        can_deposit_item: Arc<Box<dyn CanDepositItem>>,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_deposit_item,
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
    ) -> Result<DepositBankBehavior, Error> {
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
                        DepositBankBehavior {
                            current_state: "in_bank".to_string(),
                            moving_behavior: self.moving_behavior.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    // comportement non terminer
                    Ok(
                        DepositBankBehavior {
                            moving_behavior: new_moving_behavior,
                            ..self.clone()
                        }
                    )

                }
            }
            "in_bank" => {
                let item = character_info.get_first_item();
                match item {
                    Some(slot) => {
                        match self.can_deposit_item.deposit(&character_info, &slot.code, slot.quantity as u32).await {
                            Ok(_) => {
                                println!("[{}] - deposit ok slot: {:?}", character_info.name, slot);
                                Ok(
                                    DepositBankBehavior {
                                        current_state: "finish".to_string(),
                                        ..self.clone()
                                    }
                                )
                            }
                            Err(e) => {
                                println!("[{}] - can move in error : {e:?}", character_info.name);
                                Ok(self.clone())
                            } // on laisse le meme etat certainement un erreur cote serveur
                        }
                    }
                    None => {
                        println!("[{}] - no deposit because inventory is empty.", character_info.name);
                        Ok(
                            DepositBankBehavior {
                                current_state: "finish".to_string(),
                                ..self.clone()
                            }
                        )
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
}
