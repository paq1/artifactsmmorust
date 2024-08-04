use chrono::{DateTime, Utc};

use crate::core::shared::Position;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub position: Position,
    // pub cooldown: i32,
    pub cooldown_expiration: DateTime<Utc>,
    // pub task: String,
    // pub task_type: String,
    pub inventory_max_items: u32,
    pub inventory: Vec<Slot>,
}

impl Character {
    pub fn is_full_inventory(&self) -> bool {
        let current_quantity: u32 = self.inventory.iter().map(|x| x.quantity).sum();
        current_quantity >= self.inventory_max_items
    }

    pub fn get_first_item(&self) -> Option<Slot> {
        self.inventory.iter().find(|e| e.quantity > 0).map(|e| e.clone())
    }

    pub fn cooldown_sec(&self) -> i64 {
        let now = Utc::now();
        (self.cooldown_expiration - now).num_seconds()
    }
}

#[derive(Clone, Debug)]
pub struct Slot {
    // pub slot: i32,
    pub code: String,
    pub quantity: u32,
}
