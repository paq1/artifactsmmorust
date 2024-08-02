use reqwest::Client;

use crate::app::api::map::GameMap;
use crate::app::api::models::Many;
use crate::core::errors::{Error, ErrorWithCode};

// pub async fn fetch_one_map(
//     http_client: &Client,
//     token: &str,
//     url: &str,
//     name: &str,
// ) -> Result<GameMap, Error> {
//     let maybe_gamemap = fetch_maps(http_client, token, url)
//         .await
//         .map(|maps| {
//             maps.data.into_iter().find(|currentmap| currentmap.name == name.to_string())
//         })?;
//
//     maybe_gamemap
//         .map(|c| c.clone())
//         .ok_or(Error::WithCode(
//             ErrorWithCode {
//                 code: "00MAPNF".to_string(),
//                 title: format!("gamemap {name} not found"),
//                 description: None,
//                 status: None,
//             }
//         ))
// }

pub async fn fetch_maps(
    http_client: &Client,
    token: &str,
    url: &str,
) -> Result<Many<GameMap>, Error> {
    let response = http_client
        .get(format!("{url}/maps?page=1&size=100"))
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
            .json::<Many<GameMap>>()
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