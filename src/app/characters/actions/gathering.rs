use reqwest::Client;
use crate::core::errors::{Error, ErrorWithCode};

pub async fn gathering(
    http_client: &Client,
    token: &str,
    url: &str,
    player_name: &str,
) -> Result<(), Error> {
    let response = http_client
        .post(format!("{url}/my/{player_name}/action/gathering"))
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
                    code: "00CBTERR".to_string(),
                    title: "Erreur lors de la récolte".to_string(),
                    description: Some(format!("{player_name} - http status : {}", response.status())),
                }
            )
        )
    } else {
        Ok(())
    }
}