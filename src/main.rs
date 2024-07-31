use std::sync::Arc;
use chrono::DateTime;
use reqwest::Client;
use tokio::time;

use app::characters::actions::fight::fight;

use crate::app::characters::infos::fetch_one_character;

mod core;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let now = chrono::Utc::now();

    let http_client = Arc::new(Client::new());
    let url = std::env::var("API_URL_ARTIFACTSMMO")
        .unwrap_or("https://api.artifactsmmo.com".to_string());
    let token = std::env::var("TOKEN_API_ARTIFACTSMMO")
        .expect("env variable `TOKEN_API_ARTIFACTSMMO` should be set by in .env or in docker-compose env");

    loop {
        let rustboy = fetch_one_character(&http_client, token.as_str(), url.as_str(), "RustBoy").await?;
        println!("{rustboy:?}");

        let delta_time_rustboy = rustboy.cooldown_expiration - now;

        // waiting rustboy cooldown
        if delta_time_rustboy.num_seconds() > 0 {
            let delta_time_rustboy_seconds =  delta_time_rustboy.num_seconds() as u64;
            println!("waiting {delta_time_rustboy_seconds} sec");
            tokio::time::sleep(time::Duration::from_secs(delta_time_rustboy_seconds)).await;
            println!("end waiting");
        }


        // run rustboy fight
        if rustboy.cooldown_expiration <= chrono::offset::Local::now() {
            println!("go fight :)");
            match fight(&http_client, token.as_str(), url.as_str(), "RustBoy")
                .await {
                Ok(()) => (),
                Err(e) => {
                    println!("err : {e}");
                }
            }
        } else {
            println!("pas de combat, je suis occupe")
        }

        // break; // todo voir les conditions de break :)
    }



    fight(&http_client, token.as_str(), url.as_str(), "RustBoy")
        .await?;

    Ok(())
}

