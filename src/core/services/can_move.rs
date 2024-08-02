use async_trait::async_trait;

use crate::core::characters::Character;
use crate::core::errors::Error;
use crate::core::shared::Position;

#[async_trait]
pub trait CanMove: Send + Sync {
    async fn r#move(&self, character: &Character, position: &Position) -> Result<(), Error>;
}
