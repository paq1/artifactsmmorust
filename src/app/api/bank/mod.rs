use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ItemApi {
    pub code: String,
    pub quantity: u32,
}