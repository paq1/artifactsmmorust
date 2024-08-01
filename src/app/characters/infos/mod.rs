use reqwest::Client;
use crate::app::api::models::Many;
use crate::core::characters::Character;
use crate::core::errors::{Error, ErrorWithCode};

pub async fn fetch_one_character(
    http_client: &Client,
    token: &str,
    url: &str,
    name: &str,
) -> Result<Character, Error> {
    let maybecharacter = fetch_characters(http_client, token, url)
        .await
        .map(|caracters| {
            caracters.data.into_iter().find(|char| char.name == name.to_string())
        })?;

    maybecharacter
        .map(|c| c.clone())
        .ok_or(Error::WithCode(
            ErrorWithCode {
                code: "00CNOTF".to_string(),
                title: format!("character {name} not found"),
                description: None,
            }
        ))
}

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
                }
            )
        )
    } else {
        response
            .json::<Many<Character>>()
            .await
            .map_err(|err| {
                Error::WithCode(
                    ErrorWithCode {
                        code: "00FMCEER".to_string(),
                        title: "Erreur lors du parsing des characters".to_string(),
                        description: Some(err.to_string()),
                    }
                )
            })
    }
}