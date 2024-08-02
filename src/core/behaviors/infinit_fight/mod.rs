use std::sync::Arc;

use crate::core::behaviors::infinit_fight::states::InfinitFightStates;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::services::can_fight::CanFight;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

pub mod states;

#[derive(Clone)]
pub struct InfinitFight {
    pub current_state: InfinitFightStates,
    pub character_info: Character,
    pub can_fight: Arc<Box<dyn CanFight>>,
    pub can_move: Arc<Box<dyn CanMove>>,
    pub can_deposit_item: Arc<Box<dyn CanDepositItem>>,
}

impl InfinitFight {
    pub fn new(
        character_info: Character,
        can_fight: Arc<Box<dyn CanFight>>,
        can_move: Arc<Box<dyn CanMove>>,
        can_deposit_item: Arc<Box<dyn CanDepositItem>>,
    ) -> Self {
        InfinitFight {
            current_state: InfinitFightStates::Empty,
            character_info,
            can_fight,
            can_move,
            can_deposit_item,
        }
    }

    pub async fn run(&self) -> Result<Self, Error> {
        let now = chrono::Utc::now();

        match self.current_state {
            InfinitFightStates::Empty if self.character_info.is_full_inventory() => {
                Ok(
                    InfinitFight {
                        current_state: InfinitFightStates::FullInventory,
                        ..self.clone()
                    }
                )
            }
            InfinitFightStates::Empty => {
                println!("Empty states for : {}", self.character_info.name);
                let fight_position = Position { x: 0, y: 1 };
                if self.character_info.position == fight_position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitFight {
                        current_state: InfinitFightStates::GoingFight,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", fight_position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &Position { x: 0, y: 1 }).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", fight_position, self.character_info.name);
                            Ok(InfinitFight {
                                current_state: InfinitFightStates::GoingFight,
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
            InfinitFightStates::GoingFight => {
                if self.character_info.cooldown_expiration <= now {
                    println!("go fight {}", self.character_info.name);
                    match self.can_fight.fight(&self.character_info)
                        .await {
                        Ok(()) => {
                            println!("end fight for {}", self.character_info.name);
                            Ok(
                                InfinitFight {
                                    current_state: InfinitFightStates::EndFight,
                                    ..self.clone()
                                }
                            )
                        }
                        Err(e) => {
                            println!("can fight in error : {e:?}");
                            Ok(self.clone()) // peut etre un pb serveur, on attend
                        }
                    }
                } else {
                    let cooldown = self.character_info.cooldown_expiration - now;
                    println!("{} in cooldown for {} sec", self.character_info.name, cooldown.num_seconds());
                    Ok(self.clone()) // cooldown de move pas terminer, on attend
                }
            }
            InfinitFightStates::EndFight => {
                println!("trigger state GoingFight"); // on relance le combat
                if self.character_info.is_full_inventory() {
                    Ok(
                        InfinitFight {
                            current_state: InfinitFightStates::FullInventory,
                            ..self.clone()
                        }
                    )
                } else {
                    Ok(
                        InfinitFight {
                            current_state: InfinitFightStates::GoingFight,
                            ..self.clone()
                        }
                    )
                }
            }
            InfinitFightStates::FullInventory => {
                Ok(
                    InfinitFight {
                        current_state: InfinitFightStates::GoingBank,
                        ..self.clone()
                    }
                )
            }
            InfinitFightStates::GoingBank => {
                let bank_position = Position { x: 4, y: 1 };
                if self.character_info.position == bank_position {
                    println!("same position for : {}", self.character_info.name);
                    Ok(InfinitFight {
                        current_state: InfinitFightStates::Deposit,
                        ..self.clone()
                    })
                } else {
                    println!("move at {:?} for : {}", bank_position, self.character_info.name);
                    match self.can_move.r#move(&self.character_info, &bank_position).await {
                        Ok(_) => {
                            println!("move at {:?} for : {}", bank_position, self.character_info.name);
                            Ok(InfinitFight {
                                current_state: InfinitFightStates::Deposit,
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
            InfinitFightStates::Deposit => {
                let item = self.character_info.get_first_item();
                match item {
                    Some(slot) => {
                        match self.can_deposit_item.deposit(&self.character_info, &slot.code, slot.quantity as u32).await {
                            Ok(_) => {
                                println!("deposit ok slot: {:?} for : {}", slot, self.character_info.name);
                                Ok(InfinitFight {
                                    current_state: InfinitFightStates::Deposit,
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
                        Ok(InfinitFight {
                            current_state: InfinitFightStates::Empty, // on recommence :)
                            ..self.clone()
                        })
                    }
                }
            }
        }
    }
}