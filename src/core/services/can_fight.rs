use async_trait::async_trait;

use crate::core::characters::Character;
use crate::core::errors::Error;

#[async_trait]
pub trait CanFight {
    async fn fight(&self, character: &Character) -> Result<(), Error>;
}
