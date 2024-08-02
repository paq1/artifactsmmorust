use std::sync::Arc;

use crate::core::behaviors::Behavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct GoDepositBankBehavior {
    pub character_info: Character,
    pub current_state: String,
    pub bank_position: Position,
    pub can_move: Arc<Box<dyn CanMove>>,
    pub can_deposit_item: Arc<Box<dyn CanDepositItem>>,
}

impl GoDepositBankBehavior {
    pub fn new(
        character_info: Character,
        bank_position: Position,
        can_move: Arc<Box<dyn CanMove>>,
        can_deposit_item: Arc<Box<dyn CanDepositItem>>,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            character_info,
            bank_position,
            can_move,
            can_deposit_item,
        }
    }
}

impl GoDepositBankBehavior {
    pub async fn next_behavior(&self) -> Result<Option<Behavior>, Error> {
        match self.current_state.as_str() {
            "empty" => {
                if self.bank_position != self.character_info.position {
                    println!("[{}] trying to move in bank at {:?}", self.character_info.name, self.bank_position);
                    match self.can_move.r#move(&self.character_info, &self.bank_position).await {
                        Ok(_) => {
                            println!("[{}] - moved to {:?}", self.character_info.name, self.bank_position);
                            Ok(
                                Some(
                                    Behavior::GoDepositBankBehavior(
                                        GoDepositBankBehavior {
                                            current_state: "in_bank".to_string(),
                                            ..self.clone()
                                        }
                                    )
                                )
                            )
                        }
                        Err(e) => {
                            println!("{e}");
                            Ok(Some(Behavior::GoDepositBankBehavior(self.clone())))
                        }
                    }
                } else {
                    println!("[{}] - already in bank at {:?}", self.character_info.name, self.bank_position);
                    Ok(
                        Some(
                            Behavior::GoDepositBankBehavior(
                                GoDepositBankBehavior {
                                    current_state: "in_bank".to_string(),
                                    ..self.clone()
                                }
                            )
                        )
                    )
                }
            }
            "in_bank" => {
                let item = self.character_info.get_first_item();
                match item {
                    Some(slot) => {
                        match self.can_deposit_item.deposit(&self.character_info, &slot.code, slot.quantity as u32).await {
                            Ok(_) => {
                                println!("[{}] - deposit ok slot: {:?}", self.character_info.name, slot);
                                Ok(
                                    Some(
                                        Behavior::GoDepositBankBehavior(
                                            GoDepositBankBehavior {
                                                current_state: "finish".to_string(),
                                                ..self.clone()
                                            }
                                        )
                                    )
                                )
                            }
                            Err(e) => {
                                println!("[{}] - can move in error : {e:?}", self.character_info.name);
                                Ok(Some(Behavior::GoDepositBankBehavior(self.clone())))
                            } // on laisse le meme etat certainement un erreur cote serveur
                        }
                    }
                    None => {
                        println!("[{}] - no deposit because inventory is empty.", self.character_info.name);
                        Ok(
                            Some(
                                Behavior::GoDepositBankBehavior(
                                    GoDepositBankBehavior {
                                        current_state: "finish".to_string(),
                                        ..self.clone()
                                    }
                                )
                            )
                        )
                    }
                }
            }
            "finish" => {
                // on reinitialise.
                Ok(
                    Some(
                        Behavior::GoDepositBankBehavior(
                            GoDepositBankBehavior {
                                current_state: "empty".to_string(),
                                ..self.clone()
                            }
                        )
                    )
                )
            }
            _ => {
                Err(
                    Error::Simple("invalid transition".to_string())
                )
            }
        }
    }
}
