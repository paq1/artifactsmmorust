use std::sync::Arc;

use reqwest::Client;
use tokio::time;

use crate::app::characters::behaviors::rustboy::rustboy_logique;
use crate::app::characters::behaviors::scalaman::scalaman_logique;
use crate::app::characters::behaviors::ulquiche::ulquiche_logique;
use crate::app::characters::infos::fetch_characters;
use crate::app::map::infos::fetch_maps;

mod core;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    println!("debut du code");

    let http_client = Arc::new(Client::new());
    let url = std::env::var("API_URL_ARTIFACTSMMO")
        .unwrap_or("https://api.artifactsmmo.com".to_string());
    let token = std::env::var("TOKEN_API_ARTIFACTSMMO")
        .expect("env variable `TOKEN_API_ARTIFACTSMMO` should be set by in .env or in docker-compose env");

    println!("chargement des chars");
    let players_init = fetch_characters(&http_client, &token, &url).await?.data;
    println!("chargement de la gamemap");
    let gamemaps = fetch_maps(&http_client, &token, &url).await?;

    println!("info lancement du bot");
    println!("count of players : {}", players_init.len());
    println!("count of gamemaps : {}", gamemaps.pagination.map(|p| p.total).unwrap_or(-1));

    let mut rustboy_action = "nothing".to_string();
    let mut scalaman_action = "nothing".to_string();
    let mut ulquiche_action = "nothing".to_string();

    loop {
        let players_updated = fetch_characters(&http_client, &token, &url).await?.data;
        let rustboy = players_updated.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();
        let scalaman = players_updated.iter().find(|e| e.name == "ScalaMan".to_string()).unwrap();
        let ulquiche = players_updated.iter().find(|e| e.name == "Ulquiche".to_string()).unwrap();
        let now = chrono::Utc::now();

        let delta_time_rustboy = rustboy.cooldown_expiration - now;
        let delta_time_scalaman = scalaman.cooldown_expiration - now;
        let delta_time_ulquiche = ulquiche.cooldown_expiration - now;

        rustboy_logique(rustboy, &http_client, &url, &token, &delta_time_rustboy, &mut rustboy_action).await?;
        // scalaman_logique(scalaman, &http_client, &url, &token, &delta_time_scalaman).await?;
        // ulquiche_logique(ulquiche, &http_client, &url, &token, &delta_time_ulquiche).await?;

        tokio::time::sleep(time::Duration::from_secs(1)).await;
        // break; // todo voir les conditions de break :)
    }
}

