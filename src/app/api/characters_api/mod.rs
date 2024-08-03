use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::core::characters::{Character, Slot};
use crate::core::shared::Position;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CharacterApi {
    pub name: String,
    pub x: i32,
    pub y: i32,
    // pub cooldown: i32,
    pub cooldown_expiration: DateTime<Utc>,
    // pub task: String,
    // pub task_type: String,
    pub inventory_max_items: i32,
    pub inventory: Vec<SlotApi>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SlotApi {
    // pub slot: i32,
    pub code: String,
    pub quantity: i32,
}

impl From<SlotApi> for Slot {
    fn from(value: SlotApi) -> Self {
        Self {
            // slot: value.slot,
            code: value.code,
            quantity: value.quantity,
        }
    }
}


impl From<CharacterApi> for Character {
    fn from(value: CharacterApi) -> Self {
        Self {
            name: value.name,
            position: Position {
                x: value.x,
                y: value.y,
            },
            // cooldown: value.cooldown,
            cooldown_expiration: value.cooldown_expiration,
            // task: value.task,
            // task_type: value.task_type,
            inventory_max_items: value.inventory_max_items,
            inventory: value.inventory.into_iter().map(|item| item.into()).collect(),
        }
    }
}

// impl From<Character> for CharacterApi {
//     fn from(value: Character) -> Self {
//         Self {
//             name: value.name,
//             x: value.position.x,
//             y: value.position.y,
//             cooldown: value.cooldown,
//             cooldown_expiration: value.cooldown_expiration,
//             task: value.task,
//             task_type: value.task_type,
//         }
//     }
// }