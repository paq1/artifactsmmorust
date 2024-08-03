use serde::{Deserialize, Serialize};

use crate::core::shared::Position;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMapApi {
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
    pub content: Option<Content>,
}

impl GameMapApi {
    pub fn get_position(&self) -> Position {
        Position::new(self.x, self.y)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    #[serde(rename = "type")]
    pub r#type: String,
    pub code: String,
}

