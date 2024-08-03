use std::sync::Arc;

use crate::core::behaviors::go_deposit_bank::GoDepositBankBehavior;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_gathering::CanGathering;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct GoInfinitGateringBehavior {
    pub current_state: String,
    pub gathering_position: Position,
    pub can_gathering: Arc<Box<dyn CanGathering>>,
    pub deposit_bank: GoDepositBankBehavior,
    pub moving_behavior: MovingBehavior,
}

impl GoInfinitGateringBehavior {

    pub fn new(
        gathering_position: &Position,
        can_gathering: Arc<Box<dyn CanGathering>>,
        deposit_bank: GoDepositBankBehavior,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            gathering_position: gathering_position.clone(),
            can_gathering,
            deposit_bank,
            moving_behavior,
        }
    }

    pub async fn next_behavior(
        &self,
        character: &Character
    ) -> Result<GoInfinitGateringBehavior, Error> {
        let now = chrono::Utc::now();
        let cooldown = character.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown >= 0 => {
                println!("[{}] in cooldown for {} secs", character.name, cooldown);
                Ok(self.clone())
            }
            _ if character.is_full_inventory() => {
                println!("[{}] inventory is full", character.name);
                let deposit_bank = self.deposit_bank.next_behavior(
                    &character
                ).await?;

                if deposit_bank.current_state.as_str() == "finish" {
                    Ok(
                        Self {
                            current_state: "empty".to_string(),
                            deposit_bank: deposit_bank.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    Ok(
                        Self {
                            deposit_bank: deposit_bank.clone(),
                            ..self.clone()
                        }
                    )
                }
            }
            "empty" => {
                let moving = self.moving_behavior.next_behavior(&character, &self.gathering_position).await?;
                if moving.current_state.as_str() == "finish" {
                    Ok(
                        GoInfinitGateringBehavior {
                            current_state: "in_gathering_zone".to_string(),
                            moving_behavior: self.moving_behavior.reset(),
                            ..self.clone()
                        }
                    )
                } else {
                    Ok(
                        Self {
                            moving_behavior: moving.clone(),
                            ..self.clone()
                        }
                    )
                }
            }
            "in_gathering_zone" => {
                if character.cooldown_expiration <= now {
                    println!("[{}] - trying gathering for ", character.name);
                    match self.can_gathering.gathering(&character)
                        .await {
                        Ok(()) => {
                            println!("[{}] - succeed gathering.", character.name);
                            Ok(
                                GoInfinitGateringBehavior {
                                    current_state: "empty".to_string(),
                                    ..self.clone()
                                }
                            )
                        }
                        Err(e) => {
                            println!("[{}] - failed gathering. error: {e:?}", character.name);
                            match e {
                                Error::WithCode(error_with_code) => {
                                    if error_with_code.status.unwrap_or(0) == 497 {
                                        println!("[{}] - failed gathering because inventory is full", character.name);
                                        Ok(
                                            GoInfinitGateringBehavior {
                                                current_state: "empty".to_string(),
                                                ..self.clone()
                                            }
                                        )
                                    } else {
                                        Ok(
                                            self.clone(),
                                        )
                                    }
                                }
                                _ => {
                                    Ok(
                                        self.clone(),
                                    )
                                } // peut etre un pb serveur, on attend
                            }
                        }
                    }
                } else {
                    let cooldown = character.cooldown_expiration - now;
                    println!("[{}] in cooldown for {} sec", character.name, cooldown.num_seconds());
                    Ok(
                        self.clone(),
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
