use std::sync::Arc;

use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

#[derive(Clone)]
pub struct MovingBehavior {
    pub current_state: String,
    pub can_move: Arc<Box<dyn CanMove>>,
}

impl MovingBehavior {
    pub fn new(
        can_move: Arc<Box<dyn CanMove>>,
    ) -> Self {
        Self {
            current_state: "empty".to_string(),
            can_move,
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
        destination: &Position,
    ) -> Result<MovingBehavior, Error> {
        let cooldown_sec = player.cooldown_sec();

        match self.current_state.as_str() {
            _ if cooldown_sec >= 0 => {
                println!("[{}] in cooldown for {cooldown_sec} secs", player.name);
                Ok(self.clone())
            }
            "empty" => {
                if destination != &player.position {
                    println!("[{}] trying to move at {:?}", player.name, destination);
                    match self.can_move.r#move(player, &destination).await {
                        Ok(_) => {
                            println!("[{}] - moved to {:?}", player.name, destination);
                            Ok(
                                MovingBehavior {
                                    current_state: "finish".to_string(),
                                    ..self.clone()
                                }
                            )
                        }
                        Err(e) => {
                            println!("{e}");
                            Ok(self.clone())
                        }
                    }
                } else {
                    println!("[{}] - already at {:?}", player.name, destination);
                    Ok(
                        MovingBehavior {
                            current_state: "finish".to_string(),
                            ..self.clone()
                        }
                    )
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
