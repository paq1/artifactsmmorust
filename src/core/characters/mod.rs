use chrono::{DateTime, Utc};

use crate::core::shared::Position;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub position: Position,
    pub cooldown: i32,
    pub cooldown_expiration: DateTime<Utc>,
    pub task: String,
    pub task_type: String,
}