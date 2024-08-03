use async_trait::async_trait;

use crate::core::characters::Character;
use crate::core::errors::Error;

#[async_trait]
pub trait CanCraft {
    async fn _craft(&self, character: &Character, code_item: &String, quantity: u32) -> Result<(), Error>;
}
