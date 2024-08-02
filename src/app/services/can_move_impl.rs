use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::services::can_move::CanMove;
use crate::core::shared::Position;

pub struct CanMoveImpl {
    pub url: String,
    pub token: String,
    pub http_client: Arc<Client>,
}

#[async_trait]
impl CanMove for CanMoveImpl {
    async fn r#move(&self, character: &Character, position: &Position) -> Result<(), Error> {
        let response = self.http_client
            .post(format!("{}/my/{}/action/move", self.url, character.name))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&BodyMove { x: position.x, y: position.y })
            .send()
            .await
            .map_err(|e| Error::Simple(e.to_string()))?;

        if response.status() != 200 {
            Err(
                Error::WithCode(
                    ErrorWithCode {
                        code: "00MVTERR".to_string(),
                        title: "Erreur lors du deplacement".to_string(),
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

#[derive(Serialize, Deserialize)]
struct BodyMove {
    x: i32,
    y: i32,
}