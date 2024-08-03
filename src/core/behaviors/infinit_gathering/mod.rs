use crate::core::behaviors::gathering::GatheringBehavior;
use crate::core::behaviors::deposit_bank::DepositBankBehavior;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct InfinitGateringBehavior {
    pub current_state: String,
    pub gathering_position: Position,
    pub gathering_behavior: GatheringBehavior,
    pub deposit_bank: DepositBankBehavior,
    pub moving_behavior: MovingBehavior,
}

impl InfinitGateringBehavior {
    pub fn new(
        gathering_position: &Position,
        gathering_behavior: GatheringBehavior,
        deposit_bank: DepositBankBehavior,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            gathering_position: gathering_position.clone(),
            gathering_behavior,
            deposit_bank,
            moving_behavior,
        }
    }

    pub async fn next_behavior(
        &self,
        character: &Character,
    ) -> Result<InfinitGateringBehavior, Error> {
        let cooldown = character.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown >= 0 => {
                println!("[{}] in cooldown for {} secs", character.name, cooldown);
                Ok(self.clone())
            }
            _ if character.is_full_inventory() && self.current_state.as_str() == "empty" => {
                println!("[{}] inventory is full, need deposit !", character.name);
                Ok(
                    InfinitGateringBehavior {
                        current_state: "full_inventory".to_string(),
                        ..self.clone()
                    }
                )
            }
            "full_inventory" => {
                println!("[{}] inventory is full, trying deposit", character.name);
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
                        InfinitGateringBehavior {
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
                let new_gathering_behavior_ok = self.gathering_behavior.next_behavior(character).await;
                match new_gathering_behavior_ok {
                    Ok(new_gathering_behavior) => {
                        if new_gathering_behavior.current_state.as_str() == "finish" {
                            Ok(
                                InfinitGateringBehavior {
                                    current_state: "empty".to_string(),
                                    gathering_behavior: self.gathering_behavior.reset(),
                                    ..self.clone()
                                }
                            )
                        } else {
                            Ok(
                                Self {
                                    gathering_behavior: new_gathering_behavior,
                                    ..self.clone()
                                }
                            )
                        }
                    }
                    Err(e) => {
                        match e.clone() {
                            Error::WithCode(error_code) => {
                                if error_code.status.unwrap_or(0) == 497 {
                                    Ok(
                                        InfinitGateringBehavior {
                                            current_state: "full_inventory".to_string(),
                                            gathering_behavior: self.gathering_behavior.reset(),
                                            ..self.clone()
                                        }
                                    )
                                } else {
                                    Ok(self.clone())
                                }
                            }
                            _ => {
                                Ok(self.clone())
                            }
                        }
                    }
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
