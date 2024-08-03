use std::sync::Arc;

use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_fight::CanFight;

#[derive(Clone)]
pub struct FightBehavior {
    pub current_state: String,
    pub can_fight: Arc<Box<dyn CanFight>>,
}

impl FightBehavior {
    pub fn new(
        can_fight: Arc<Box<dyn CanFight>>,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_fight,
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
    ) -> Result<FightBehavior, Error> {
        let cooldown_sec = player.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown_sec >= 0 => {
                println!("[{}] in cooldown for {cooldown_sec} secs", player.name);
                Ok(self.clone())
            }
            "empty" => {
                match self.can_fight.fight(player).await {
                    Ok(_) => {
                        println!("[{}] fight succeed.", player.name);
                        Ok(
                            FightBehavior {
                                current_state: "finish".to_string(),
                                ..self.clone()
                            }
                        )
                    }
                    Err(e) => {
                        println!("[{}] fight failed due to {e:?}.", player.name);
                        Ok(
                            self.clone()
                        )
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
