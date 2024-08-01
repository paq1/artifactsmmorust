use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameMap {
    pub name: String,
    pub skin: String,
    pub x: i32,
    pub y: i32,
    // pub content: Content,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Content {
    pub r#type: String,
    pub code: String,
}

