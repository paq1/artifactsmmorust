use std::sync::Arc;

use crate::core::behaviors::infinit_fight::events::InfinitFightEvents;
use crate::core::behaviors::infinit_fight::states::InfinitFightStates;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_fight::CanFight;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

pub mod states;
pub mod events;

#[derive(Clone)]
pub struct InfinitFight {
    pub current_state: InfinitFightStates,
    pub events: Vec<InfinitFightEvents>,
    pub character_info: Character,
    pub can_fight: Arc<Box<dyn CanFight>>,
    pub can_move: Arc<Box<dyn CanMove>>,
}

impl InfinitFight {
    pub fn new(
        character_info: Character,
        can_fight: Arc<Box<dyn CanFight>>,
        can_move: Arc<Box<dyn CanMove>>,
    ) -> Self {
        InfinitFight {
            current_state: InfinitFightStates::Empty,
            events: vec![],
            character_info,
            can_fight,
            can_move,
        }
    }

    pub async fn run(&self) -> Result<Self, Error> {
        let now = chrono::Utc::now();

        match self.current_state {
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
                Ok(
                    InfinitFight {
                        current_state: InfinitFightStates::GoingFight,
                        ..self.clone()
                    }
                )
            }
            _ => Err(
                Error::Simple(format!("Erreur de transion d'etat pour : {:?}", self.current_state))
            )
        }
    }
}