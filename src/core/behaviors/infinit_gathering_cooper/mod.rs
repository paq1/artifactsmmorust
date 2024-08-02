use std::sync::Arc;

use crate::core::behaviors::infinit_gathering_cooper::states::InfinitGateringCooperStates;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::services::can_gathering::CanGathering;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

mod states;

#[derive(Clone)]
pub struct InfinitGatheringCooper {
    pub current_state: InfinitGateringCooperStates,
    pub character_info: Character,
    pub can_gathering: Arc<Box<dyn CanGathering>>,
    pub can_move: Arc<Box<dyn CanMove>>,
    pub can_deposit_item: Arc<Box<dyn CanDepositItem>>,
}

impl InfinitGatheringCooper {
    pub fn new(
        character_info: Character,
        can_gathering: Arc<Box<dyn CanGathering>>,
        can_move: Arc<Box<dyn CanMove>>,
        can_deposit_item: Arc<Box<dyn CanDepositItem>>,
    ) -> Self {
        InfinitGatheringCooper {
            current_state: InfinitGateringCooperStates::Empty,
            character_info,
            can_gathering,
            can_move,
            can_deposit_item,
        }
    }

    pub async fn run(&self) -> Result<Self, Error> {
        let now = chrono::Utc::now();

        match self.current_state {
            InfinitGateringCooperStates::Empty => {
                println!("Empty states for : {}", self.character_info.name);
                let cooper_position = Position { x: 2, y: 0 };
                if self.character_info.position == cooper_position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitGatheringCooper {
                        current_state: InfinitGateringCooperStates::GoingGathering,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", cooper_position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &cooper_position).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", cooper_position, self.character_info.name);
                            Ok(InfinitGatheringCooper {
                                current_state: InfinitGateringCooperStates::GoingGathering,
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
            InfinitGateringCooperStates::GoingGathering => {
                if self.character_info.cooldown_expiration <= now {
                    println!("go gathering for {}", self.character_info.name);
                    match self.can_gathering.gathering(&self.character_info)
                        .await {
                        Ok(()) => {
                            println!("end gathering for {}", self.character_info.name);
                            Ok(
                                InfinitGatheringCooper {
                                    current_state: InfinitGateringCooperStates::EndGathering,
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
                                            InfinitGatheringCooper {
                                                current_state: InfinitGateringCooperStates::FullInventory,
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
            InfinitGateringCooperStates::EndGathering => {
                println!("trigger state GoingGathering"); // on relance le combat
                Ok(
                    InfinitGatheringCooper {
                        current_state: InfinitGateringCooperStates::GoingGathering,
                        ..self.clone()
                    }
                )
            }
            InfinitGateringCooperStates::FullInventory => {
                println!("trigger state GoingBank"); // on relance le combat
                Ok(
                    InfinitGatheringCooper {
                        current_state: InfinitGateringCooperStates::GoingBank,
                        ..self.clone()
                    }
                )
            }
            InfinitGateringCooperStates::GoingBank => {
                let bank_position = Position { x: 4, y: 1 };
                if self.character_info.position == bank_position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitGatheringCooper {
                        current_state: InfinitGateringCooperStates::Deposit,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", bank_position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &bank_position).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", bank_position, self.character_info.name);
                            Ok(InfinitGatheringCooper {
                                current_state: InfinitGateringCooperStates::Deposit,
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
            InfinitGateringCooperStates::Deposit => {
                let item = self.character_info.get_first_item();
                match item {
                    Some(slot) => {
                        match self.can_deposit_item.deposit(&self.character_info, &slot.code, slot.quantity as u32).await {
                            Ok(_) => {
                                println!("deposit ok slot: {:?} for : {}", slot, self.character_info.name);
                                Ok(InfinitGatheringCooper {
                                    current_state: InfinitGateringCooperStates::Deposit,
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
                        Ok(InfinitGatheringCooper {
                            current_state: InfinitGateringCooperStates::Empty,
                            ..self.clone()
                        })
                    }
                }
            }
        }
    }
}