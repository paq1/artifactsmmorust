use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;

use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::services::can_gathering::CanGathering;

pub struct CanGatheringImpl {
    pub url: String,
    pub token: String,
    pub http_client: Arc<Client>,
}

#[async_trait]
impl CanGathering for CanGatheringImpl {
    async fn gathering(&self, character: &Character) -> Result<(), Error> {
        let response = self.http_client
            .post(format!("{}/my/{}/action/gathering", self.url, character.name))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await
            .map_err(|e| Error::Simple(e.to_string()))?;

        if response.status() != 200 {
            Err(
                Error::WithCode(
                    ErrorWithCode {
                        code: "00GATERR".to_string(),
                        title: "Erreur lors du gathering".to_string(),
                        description: Some(format!("http status : {}", response.status())),
                        status: Some(response.status().as_u16() as i32),
                    }
                )
            )
        } else {
            Ok(())
        }
    }
}