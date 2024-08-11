use crate::app::api::bank::ItemApi;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::services::can_get_bank::CanGetBank;
use crate::core::shared::api_models::Many;
use async_trait::async_trait;
use reqwest::Client;
use std::sync::Arc;

pub struct CanGetBankImpl {
    pub url: String,
    pub token: String,
    pub http_client: Arc<Client>,
}

#[async_trait]
impl CanGetBank for CanGetBankImpl {


    // fixme prendre en compte la pagination
    async fn get_items(&self) -> Result<Vec<(String, u32)>, Error> {
        let response = self.http_client
            .get(format!("{}/my/bank/items?page=1&size=100", self.url))
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
                        code: "00ITBAN".to_string(),
                        title: "Erreur lors de la recuperation des items de la bank".to_string(),
                        description: Some(format!("http status : {}", response.status())),
                        status: Some(response.status().as_u16() as i32),
                    }
                )
            )
        } else {
            response
                .json::<Many<ItemApi>>()
                .await
                .map(|x| x.data.iter().map(|slot| (slot.code.clone(), slot.quantity)).collect())
                .map_err(|err| {
                    Error::WithCode(
                        ErrorWithCode {
                            code: "00PEMAP".to_string(),
                            title: "Erreur lors du des items".to_string(),
                            description: Some(err.to_string()),
                            status: None,
                        }
                    )
                })
        }
    }
}