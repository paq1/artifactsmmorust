use async_trait::async_trait;

use crate::core::errors::Error;

#[async_trait]
pub trait CanGetBank: Send + Sync {
    async fn get_items(&self) -> Result<Vec<(String, u32)>, Error>;
}
