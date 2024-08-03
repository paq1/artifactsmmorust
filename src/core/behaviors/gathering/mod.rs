use std::sync::Arc;

use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_gathering::CanGathering;

#[derive(Clone)]
pub struct GatheringBehavior {
    pub current_state: String,
    pub can_gathering: Arc<Box<dyn CanGathering>>,
}

impl GatheringBehavior {
    pub fn new(
        can_gathering: Arc<Box<dyn CanGathering>>,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_gathering,
        }
    }

    pub fn reset(&self) -> Self {
        Self {
            current_state: "empty".to_string(),
            ..self.clone()
        }
    }
    pub async fn next_behavior(
        &self,
        player: &Character,
    ) -> Result<GatheringBehavior, Error> {
        let cooldown_sec = player.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown_sec >= 0 => {
                println!("[{}] in cooldown for {cooldown_sec} secs", player.name);
                Ok(self.clone())
            }
            "empty" => {
                match self.can_gathering.gathering(player).await {
                    Ok(_) => {
                        println!("[{}] - succeed gathering.", player.name);
                        Ok(
                            GatheringBehavior {
                                current_state: "finish".to_string(),
                                ..self.clone()
                            }
                        )
                    }
                    Err(e) => {
                        println!("[{}] error gathering {e}", player.name);
                        match e.clone() {
                            Error::WithCode(error_with_code) => {
                                if error_with_code.status.unwrap_or(0) == 497 {
                                    println!("[{}] - failed gathering because inventory is full", player.name);
                                    Err(e)
                                } else {
                                    Ok(
                                        self.clone(),
                                    )
                                }
                            }
                            _ => {
                                println!("retry at the next iteration");
                                Ok(self.clone())
                            } // peut etre un pb serveur, on attend
                        }
                    }
                }
            }
            _ => {
                Err(
                    Error::Simple("invalid transition".to_string())
                )
            }
        }
    }
}
