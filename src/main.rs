use std::sync::Arc;
use chrono::DateTime;
use reqwest::Client;
use tokio::time;

use app::characters::actions::fight::fight;

use crate::app::characters::infos::{fetch_characters, fetch_one_character};
use crate::app::map::infos::fetch_maps;

mod core;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();


    let http_client = Arc::new(Client::new());
    let url = std::env::var("API_URL_ARTIFACTSMMO")
        .unwrap_or("https://api.artifactsmmo.com".to_string());
    let token = std::env::var("TOKEN_API_ARTIFACTSMMO")
        .expect("env variable `TOKEN_API_ARTIFACTSMMO` should be set by in .env or in docker-compose env");

    let players = fetch_characters(&http_client, &token, &url).await?.data;
    let gamemaps = fetch_maps(&http_client, &token, &url).await?;

    println!("info lancement du bot");
    println!("count of players : {}", players.len());
    println!("count of gamemaps : {}", gamemaps.pagination.map(|p| p.total).unwrap_or(-1));

    loop {
        let rustboy = fetch_one_character(&http_client, token.as_str(), url.as_str(), "RustBoy").await?;
        let now = chrono::Utc::now();

        println!("{rustboy:?}");

        let delta_time_rustboy = rustboy.cooldown_expiration - now;

        // waiting rustboy cooldown
        if delta_time_rustboy.num_milliseconds() > 0 {
            let delta_time_rustboy_seconds =  delta_time_rustboy.num_seconds() as u64;
            let delta_time_rustboy_ms =  delta_time_rustboy.num_milliseconds() as u64;
            println!("waiting {delta_time_rustboy_seconds} sec");
            tokio::time::sleep(time::Duration::from_millis(delta_time_rustboy_ms)).await;
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

