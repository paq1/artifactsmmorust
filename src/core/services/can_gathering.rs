use async_trait::async_trait;

use crate::core::characters::Character;
use crate::core::errors::Error;

#[async_trait]
pub trait CanGathering: Send + Sync {
    async fn gathering(&self, character: &Character) -> Result<(), Error>;
}
