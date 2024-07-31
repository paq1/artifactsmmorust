use reqwest::Client;
use crate::core::errors::{Error, ErrorWithCode};

pub async fn fight(
    http_client: &Client,
    token: &str,
    url: &str,
    player_name: &str,
) -> Result<(), Error> {
    let response = http_client
        .post(format!("{url}/my/{player_name}/action/fight"))
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
                    title: "Erreur lors du combat".to_string(),
                    description: None,
                }
            )
        )
    } else {
        Ok(())
    }
}