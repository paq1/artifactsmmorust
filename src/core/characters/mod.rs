use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Character {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub cooldown: i32,
    pub cooldown_expiration: DateTime<Utc>,
    pub task: String,
    pub task_type: String,
}