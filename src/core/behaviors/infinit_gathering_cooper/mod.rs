use std::sync::Arc;

use crate::core::behaviors::infinit_gathering_cooper::states::InfinitGateringCooperStates;
use crate::core::characters::Character;
use crate::core::errors::Error;
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
}

impl InfinitGatheringCooper {
    pub fn new(
        character_info: Character,
        can_gathering: Arc<Box<dyn CanGathering>>,
        can_move: Arc<Box<dyn CanMove>>,
    ) -> Self {
        InfinitGatheringCooper {
            current_state: InfinitGateringCooperStates::Empty,
            character_info,
            can_gathering,
            can_move,
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
                            Ok(self.clone()) // peut etre un pb serveur, on attend
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
        }
    }
}