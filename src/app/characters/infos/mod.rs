use reqwest::Client;

use crate::app::api::characters_api::CharacterApi;
use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::shared::api_models::Many;

pub async fn fetch_characters(
    http_client: &Client,
    token: &str,
    url: &str,
) -> Result<Many<Character>, Error> {
    let response = http_client
        .get(format!("{url}/my/characters"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| Error::Simple(e.to_string()))?;

    if response.status() != 200 {
        Err(
            Error::WithCode(
                ErrorWithCode {
                    code: "00PASDE".to_string(),
                    title: "Erreur lors de la recuperation des personnage".to_string(),
                    description: None,
                    status: Some(response.status().as_u16() as i32),
                }
            )
        )
    } else {
        response
            .json::<Many<CharacterApi>>()
            .await
            .map(|c| {
                c.dmap(|x| x.into())
            })
            .map_err(|err| {
                Error::WithCode(
                    ErrorWithCode {
                        code: "00FMCEER".to_string(),
                        title: "Erreur lors du parsing des characters".to_string(),
                        description: Some(err.to_string()),
                        status: None,
                    }
                )
            })
    }
}