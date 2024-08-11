use reqwest::Client;

use crate::app::api::map::GameMapApi;
use crate::core::errors::{Error, ErrorWithCode};
use crate::core::shared::api_models::{Many, Single};
use crate::core::shared::Position;

pub async fn fetch_maps_from_position(
    http_client: &Client,
    token: &str,
    url: &str,
    position: &Position,
) -> Result<GameMapApi, Error> {
    let response = http_client
        .get(format!("{url}/maps/{}/{}", position.x, position.y))
        // .form(&HashMap::from([
        //     ("page", 1),
        //     ("size", 100),
        // ]))
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
                    code: "00MAPREC".to_string(),
                    title: "Erreur lors de la recuperation de la map.".to_string(),
                    description: Some(format!("http status : {}", response.status())),
                    status: Some(response.status().as_u16() as i32),
                }
            )
        )
    } else {
        response
            .json::<Single<GameMapApi>>()
            .await
            .map(|e| {
                e.data
            })
            .map_err(|err| {
                Error::WithCode(
                    ErrorWithCode {
                        code: "00PESINGMAP".to_string(),
                        title: "Erreur lors du parsing des gamemaps".to_string(),
                        description: Some(err.to_string()),
                        status: None,
                    }
                )
            })
    }
}

pub async fn fetch_maps(
    http_client: &Client,
    token: &str,
    url: &str,
    params: Option<Vec<(&str, &str)>>,
) -> Result<Many<GameMapApi>, Error> {

    let params_str = params
        .map(|params| {
            let p = params.into_iter().map(|(k, v)| format!("{k}={v}")).collect::<Vec<_>>().join("&");
            format!("?{p}")
        })
        .unwrap_or("".to_string());

    println!("params : {params_str}");

    let response = http_client
        .get(format!("{url}/maps{params_str}"))
        // .form(&HashMap::from([
        //     ("page", 1),
        //     ("size", 100),
        // ]))
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
                    code: "00MAPREC".to_string(),
                    title: "Erreur lors de la recuperation des maps".to_string(),
                    description: Some(format!("http status : {}", response.status())),
                    status: Some(response.status().as_u16() as i32),
                }
            )
        )
    } else {
        response
            .json::<Many<GameMapApi>>()
            .await
            .map_err(|err| {
                Error::WithCode(
                    ErrorWithCode {
                        code: "00PEMAP".to_string(),
                        title: "Erreur lors du parsing des gamemaps".to_string(),
                        description: Some(err.to_string()),
                        status: None,
                    }
                )
            })
    }
}