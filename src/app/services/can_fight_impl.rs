use std::sync::Arc;
use async_trait::async_trait;
use reqwest::Client;
use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::services::can_fight::CanFight;

pub struct CanFightImpl {
    pub url: String,
    pub token: String,
    pub http_client: Arc<Client>,
}

#[async_trait]
impl CanFight for CanFightImpl {
    async fn fight(&self, character: &Character) -> Result<(), Error> {
        let response = self.http_client
            .post(format!("{}/my/{}/action/fight", self.url, character.name ))
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
                        code: "00CBTERR".to_string(),
                        title: "Erreur lors du combat".to_string(),
                        description: Some(format!("http status : {}", response.status())),
                    }
                )
            )
        } else {
            Ok(())
        }
    }
}