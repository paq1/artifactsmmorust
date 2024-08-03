use crate::core::behaviors::fight::FightBehavior;
use crate::core::behaviors::go_deposit_bank::GoDepositBankBehavior;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct GoInfinitFight {
    pub current_state: String,
    pub fight_position: Position,
    pub fight_behavior: FightBehavior,
    pub deposit_bank: GoDepositBankBehavior,
    pub moving_behavior: MovingBehavior,
}

impl GoInfinitFight {
    pub fn new(
        fight_position: &Position,
        fight_behavior: FightBehavior,
        deposit_bank: GoDepositBankBehavior,
        moving_behavior: MovingBehavior,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            fight_position: fight_position.clone(),
            fight_behavior,
            deposit_bank,
            moving_behavior,
        }
    }

    pub fn is_in_workflow_deposit(&self) -> bool {
        let workflow_deposit = vec![
            "full_inventory",
        ];

        workflow_deposit.contains(&self.current_state.as_str())
    }

    pub async fn next_behavior(
        &self,
        character: &Character,
    ) -> Result<GoInfinitFight, Error> {

        let cooldown = character.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown >= 0 => {
                println!("[{}] in cooldown for {} secs", character.name, cooldown);
                Ok(self.clone())
            }
            _ if character.is_full_inventory() && !self.is_in_workflow_deposit() => {
                println!("[{}] inventory is full, need deposit !", character.name);
                Ok(
                    GoInfinitFight {
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
                let moving = self.moving_behavior.next_behavior(&character, &self.fight_position).await?;
                if moving.current_state.as_str() == "finish" {
                    Ok(
                        GoInfinitFight {
                            current_state: "in_fight_zone".to_string(),
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
            "in_fight_zone" => {
                let new_fight_behavior_ok = self.fight_behavior.next_behavior(character).await;
                match new_fight_behavior_ok {
                    Ok(new_fight_behavior) => {
                        if new_fight_behavior.current_state.as_str() == "finish" {
                            Ok(
                                GoInfinitFight {
                                    current_state: "empty".to_string(),
                                    fight_behavior: self.fight_behavior.reset(),
                                    ..self.clone()
                                }
                            )
                        } else {
                            Ok(
                                Self {
                                    fight_behavior: new_fight_behavior,
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
                                        GoInfinitFight {
                                            current_state: "full_inventory".to_string(),
                                            fight_behavior: self.fight_behavior.reset(),
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
