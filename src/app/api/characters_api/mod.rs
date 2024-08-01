use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::characters::Character;
use crate::core::shared::Position;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterApi {
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub cooldown: i32,
    pub cooldown_expiration: DateTime<Utc>,
    pub task: String,
    pub task_type: String,
}

impl From<CharacterApi> for Character {
    fn from(value: CharacterApi) -> Self {
        Self {
            name: value.name,
            position: Position {
                x: value.x,
                y: value.y,
            },
            cooldown: value.cooldown,
            cooldown_expiration: value.cooldown_expiration,
            task: value.task,
            task_type: value.task_type,
        }
    }
}

impl From<Character> for CharacterApi {
    fn from(value: Character) -> Self {
        Self {
            name: value.name,
            x: value.position.x,
            y: value.position.y,
            cooldown: value.cooldown,
            cooldown_expiration: value.cooldown_expiration,
            task: value.task,
            task_type: value.task_type,
        }
    }
}