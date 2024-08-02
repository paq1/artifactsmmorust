use std::sync::Arc;

use crate::core::behaviors::Behavior;
use crate::core::behaviors::go_deposit_bank::GoDepositBankBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_gathering::CanGathering;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct GoInfinitGateringBehavior {
    pub character_info: Character,
    pub current_state: String,
    pub gathering_position: Position,
    pub can_gathering: Arc<Box<dyn CanGathering>>,
    pub can_move: Arc<Box<dyn CanMove>>,
    pub deposit_bank: GoDepositBankBehavior,
}

impl GoInfinitGateringBehavior {
    pub fn new(
        character_info: Character,
        gathering_position: &Position,
        can_gathering: Arc<Box<dyn CanGathering>>,
        can_move: Arc<Box<dyn CanMove>>,
        deposit_bank: GoDepositBankBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            character_info,
            gathering_position: gathering_position.clone(),
            can_gathering,
            can_move,
            deposit_bank,
        }
    }
}

impl GoInfinitGateringBehavior {
    pub async fn next_behavior(&self) -> Result<Option<Behavior>, Error> {
        let now = chrono::Utc::now();

        match self.current_state.as_str() {
            _ if self.character_info.is_full_inventory() => {
                println!("[{}] inventory is full", self.character_info.name);
                let deposit_bank = self.deposit_bank.next_behavior().await?;
                match deposit_bank {
                    Some(behavior) => {
                        match behavior {
                            Behavior::GoDepositBankBehavior(b) => {
                                Ok(
                                    Some(
                                        Behavior::GoInfinitGateringBehavior(
                                            Self {
                                                current_state: "empty".to_string(),
                                                deposit_bank: b,
                                                ..self.clone()
                                            }
                                        )
                                    )
                                )
                            }
                            _ => {
                                Err(Error::Simple("unexpected behavior type".to_string()))
                            }
                        }

                    }
                    None => {
                        Ok(
                            Some(
                                Behavior::GoInfinitGateringBehavior(
                                    self.clone()
                                )
                            )
                        )
                    }
                }
            }
            "empty" => {
                if self.character_info.position != self.gathering_position {
                    println!("[{}] trying to move in gathering zone at {:?}", self.character_info.name, self.gathering_position);
                    match self.can_move.r#move(&self.character_info, &self.gathering_position).await {
                        Ok(_) => {
                            println!("[{}] - moved to {:?}", self.character_info.name, self.gathering_position);
                            Ok(
                                Some(
                                    Behavior::GoInfinitGateringBehavior(
                                        GoInfinitGateringBehavior {
                                            current_state: "in_gathering_zone".to_string(),
                                            ..self.clone()
                                        }
                                    )
                                )
                            )
                        }
                        Err(e) => {
                            println!("{e}");
                            Ok(Some(Behavior::GoInfinitGateringBehavior(self.clone())))
                        }
                    }
                } else {
                    println!("[{}] - already in gathering zone at {:?}", self.character_info.name, self.gathering_position);
                    Ok(
                        Some(
                            Behavior::GoInfinitGateringBehavior(
                                GoInfinitGateringBehavior {
                                    current_state: "in_gathering_zone".to_string(),
                                    ..self.clone()
                                }
                            )
                        )
                    )
                }
            }
            "in_gathering_zone" => {
                if self.character_info.cooldown_expiration <= now {
                    println!("[{}] - trying gathering for ", self.character_info.name);
                    match self.can_gathering.gathering(&self.character_info)
                        .await {
                        Ok(()) => {
                            println!("[{}] - succeed gathering.", self.character_info.name);
                            Ok(
                                Some(
                                    Behavior::GoInfinitGateringBehavior(
                                        GoInfinitGateringBehavior {
                                            current_state: "empty".to_string(),
                                            ..self.clone()
                                        }
                                    )
                                )
                            )
                        }
                        Err(e) => {
                            println!("[{}] - failed gathering. error: {e:?}", self.character_info.name);
                            match e {
                                Error::WithCode(error_with_code) => {
                                    if error_with_code.status.unwrap_or(0) == 497 {
                                        println!("[{}] - failed gathering because inventory is full", self.character_info.name);
                                        Ok(
                                            Some(
                                                Behavior::GoInfinitGateringBehavior(
                                                    GoInfinitGateringBehavior {
                                                        current_state: "empty".to_string(),
                                                        ..self.clone()
                                                    }
                                                )
                                            )
                                        )
                                    } else {
                                        Ok(
                                            Some(
                                                Behavior::GoInfinitGateringBehavior(
                                                    self.clone(),
                                                )
                                            )
                                        )
                                    }
                                }
                                _ => {
                                    Ok(
                                        Some(
                                            Behavior::GoInfinitGateringBehavior(
                                                self.clone(),
                                            )
                                        )
                                    )
                                } // peut etre un pb serveur, on attend
                            }
                        }
                    }
                } else {
                    let cooldown = self.character_info.cooldown_expiration - now;
                    println!("[{}] in cooldown for {} sec", self.character_info.name, cooldown.num_seconds());
                    Ok(
                        Some(
                            Behavior::GoInfinitGateringBehavior(
                                self.clone(),
                            )
                        )
                    ) // cooldown de move pas terminer, on attend
                }
            }
            _ => {
                Err(
                    Error::Simple(format!("invalid transition from {}", self.current_state))
                )
            }
        }
    }
}
