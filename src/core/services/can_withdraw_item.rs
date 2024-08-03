use async_trait::async_trait;

use crate::core::characters::Character;
use crate::core::errors::Error;

#[async_trait]
pub trait CanWithdrawItem: Send + Sync {
    async fn withdraw(&self, character: &Character, code_item: &String, quantity: u32) -> Result<(), Error>;
}
