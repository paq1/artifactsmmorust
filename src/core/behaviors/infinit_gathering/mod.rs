use std::sync::Arc;

use crate::core::behaviors::infinit_gathering::states::InfinitGateringStates;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::services::can_gathering::CanGathering;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

mod states;

#[derive(Clone)]
pub struct InfinitGathering {
    pub position: Position,
    pub current_state: InfinitGateringStates,
    pub character_info: Character,
    pub can_gathering: Arc<Box<dyn CanGathering>>,
    pub can_move: Arc<Box<dyn CanMove>>,
    pub can_deposit_item: Arc<Box<dyn CanDepositItem>>,
}

impl InfinitGathering {
    pub fn new(
        character_info: Character,
        can_gathering: Arc<Box<dyn CanGathering>>,
        can_move: Arc<Box<dyn CanMove>>,
        can_deposit_item: Arc<Box<dyn CanDepositItem>>,
        position: &Position,
    ) -> Self {
        InfinitGathering {
            current_state: InfinitGateringStates::Empty,
            character_info,
            can_gathering,
            can_move,
            can_deposit_item,
            position: position.clone(),
        }
    }

    pub async fn run(&self) -> Result<Self, Error> {
        let now = chrono::Utc::now();

        match self.current_state {
            InfinitGateringStates::Empty => {
                println!("Empty states for : {}", self.character_info.name);
                let position = self.position.clone();
                if self.character_info.position == position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitGathering {
                        current_state: InfinitGateringStates::GoingGathering,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &position).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", position, self.character_info.name);
                            Ok(InfinitGathering {
                                current_state: InfinitGateringStates::GoingGathering,
                                ..self.clone()
                            })
                        }
                        Err(e) => {
                            println!("can move in error : {e:?}");
                            Ok(self.clone())
                        } // on laisse le meme etat certainement un erreur cote serveur
                    }
                }
            }
            InfinitGateringStates::GoingGathering => {
                if self.character_info.cooldown_expiration <= now {
                    println!("go gathering for {}", self.character_info.name);
                    match self.can_gathering.gathering(&self.character_info)
                        .await {
                        Ok(()) => {
                            println!("end gathering for {}", self.character_info.name);
                            Ok(
                                InfinitGathering {
                                    current_state: InfinitGateringStates::EndGathering,
                                    ..self.clone()
                                }
                            )
                        }
                        Err(e) => {
                            println!("can gathering in error : {e:?}");
                            match e {
                                Error::WithCode(error_with_code) => {
                                    if error_with_code.status.unwrap_or(0) == 497 {
                                        Ok(
                                            InfinitGathering {
                                                current_state: InfinitGateringStates::FullInventory,
                                                ..self.clone()
                                            }
                                        )
                                    } else {
                                        Ok(self.clone()) // peut etre un pb serveur, on attend
                                    }
                                }
                                _ => Ok(self.clone()) // peut etre un pb serveur, on attend
                            }
                        }
                    }
                } else {
                    let cooldown = self.character_info.cooldown_expiration - now;
                    println!("{} in cooldown for {} sec", self.character_info.name, cooldown.num_seconds());
                    Ok(self.clone()) // cooldown de move pas terminer, on attend
                }
            }
            InfinitGateringStates::EndGathering => {
                println!("trigger state GoingGathering"); // on relance le combat
                Ok(
                    InfinitGathering {
                        current_state: InfinitGateringStates::GoingGathering,
                        ..self.clone()
                    }
                )
            }
            InfinitGateringStates::FullInventory => {
                println!("trigger state GoingBank"); // on relance le combat
                Ok(
                    InfinitGathering {
                        current_state: InfinitGateringStates::GoingBank,
                        ..self.clone()
                    }
                )
            }
            InfinitGateringStates::GoingBank => {
                let bank_position = Position { x: 4, y: 1 };
                if self.character_info.position == bank_position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitGathering {
                        current_state: InfinitGateringStates::Deposit,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", bank_position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &bank_position).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", bank_position, self.character_info.name);
                            Ok(InfinitGathering {
                                current_state: InfinitGateringStates::Deposit,
                                ..self.clone()
                            })
                        }
                        Err(e) => {
                            println!("can move in error : {e:?}");
                            Ok(self.clone())
                        } // on laisse le meme etat certainement un erreur cote serveur
                    }
                }
            }
            InfinitGateringStates::Deposit => {
                let item = self.character_info.get_first_item();
                match item {
                    Some(slot) => {
                        match self.can_deposit_item.deposit(&self.character_info, &slot.code, slot.quantity as u32).await {
                            Ok(_) => {
                                println!("deposit ok slot: {:?} for : {}", slot, self.character_info.name);
                                Ok(InfinitGathering {
                                    current_state: InfinitGateringStates::Deposit,
                                    ..self.clone()
                                })
                            }
                            Err(e) => {
                                println!("can move in error : {e:?}");
                                Ok(self.clone())
                            } // on laisse le meme etat certainement un erreur cote serveur
                        }
                    }
                    None => {
                        Ok(InfinitGathering {
                            current_state: InfinitGateringStates::Empty,
                            ..self.clone()
                        })
                    }
                }
            }
        }
    }
}