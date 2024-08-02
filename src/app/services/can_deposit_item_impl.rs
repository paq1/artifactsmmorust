use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::services::can_deposit_item::CanDepositItem;

pub struct CanDepositItemImpl {
    pub url: String,
    pub token: String,
    pub http_client: Arc<Client>,
}

#[async_trait]
impl CanDepositItem for CanDepositItemImpl {
    async fn deposit(&self, character: &Character, code_item: &String, quantity: u32) -> Result<(), Error> {
        let response = self.http_client
            .post(format!("{}/my/{}/action/bank/deposit", self.url, character.name))
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.token))
            .json(&BodyDeposit { code: code_item.clone(), quantity })
            .send()
            .await
            .map_err(|e| Error::Simple(e.to_string()))?;

        if response.status() != 200 {
            Err(
                Error::WithCode(
                    ErrorWithCode {
                        code: "00DEPERR".to_string(),
                        title: "Erreur lors dépot".to_string(),
                        description: None,
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
struct BodyDeposit {
    code: String,
    quantity: u32,
}