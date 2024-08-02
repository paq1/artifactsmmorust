use std::sync::Arc;

use reqwest::Client;
use tokio::time;

use crate::app::characters::behaviors::scalaman::scalaman_logique;
use crate::app::characters::behaviors::ulquiche::ulquiche_logique;
use crate::app::characters::infos::fetch_characters;
use crate::app::map::infos::fetch_maps;
use crate::app::services::can_fight_impl::CanFightImpl;
use crate::app::services::can_gathering_impl::CanGatheringImpl;
use crate::app::services::can_move_impl::CanMoveImpl;
use crate::core::behaviors::infinit_fight::InfinitFight;
use crate::core::services::can_fight::CanFight;
use crate::core::services::can_gathering::CanGathering;
use crate::core::services::can_move::CanMove;

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

    // services
    let can_fight: Arc<Box<dyn CanFight>> = Arc::new(Box::new(CanFightImpl {
        url: url.clone(),
        token: token.clone(),
        http_client: http_client.clone(),
    }));
    let can_gathering: Arc<Box<dyn CanGathering>> = Arc::new(Box::new(CanGatheringImpl {
        url: url.clone(),
        token: token.clone(),
        http_client: http_client.clone(),
    }));
    let can_move: Arc<Box<dyn CanMove>> = Arc::new(Box::new(CanMoveImpl {
        url: url.clone(),
        token: token.clone(),
        http_client: http_client.clone(),
    }));

    println!("chargement des chars");
    let players_init = fetch_characters(&http_client, &token, &url).await?.data;
    println!("chargement de la gamemap");
    let gamemaps = fetch_maps(&http_client, &token, &url).await?;

    println!("info lancement du bot");
    println!("count of players : {}", players_init.len());
    println!("count of gamemaps : {}", gamemaps.pagination.map(|p| p.total).unwrap_or(-1));

    let mut rustboy_action = "nothing".to_string();
    let mut _scalaman_action = "nothing".to_string();
    let mut _ulquiche_action = "nothing".to_string();

    let rustboy_init = players_init.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();


    let mut rustboy_behavior = InfinitFight::new(
        rustboy_init.clone(),
        can_fight.clone(),
        can_move.clone(),
    );

    loop {
        let players_updated = fetch_characters(&http_client, &token, &url).await?.data;
        let rustboy = players_updated.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();
        let scalaman = players_updated.iter().find(|e| e.name == "ScalaMan".to_string()).unwrap();
        let ulquiche = players_updated.iter().find(|e| e.name == "Ulquiche".to_string()).unwrap();
        let now = chrono::Utc::now();

        let _delta_time_rustboy = rustboy.cooldown_expiration - now;
        let _delta_time_scalaman = scalaman.cooldown_expiration - now;
        let _delta_time_ulquiche = ulquiche.cooldown_expiration - now;

        let next_beavior_rustboy = rustboy_behavior.run().await?;
        rustboy_behavior = InfinitFight {
            character_info: rustboy.clone(),
            ..next_beavior_rustboy.clone()
        };

        // rustboy_logique(rustboy, &delta_time_rustboy, &mut rustboy_action, can_fight.clone()).await?;
        // scalaman_logique(scalaman, &delta_time_scalaman, can_gathering.clone()).await?;
        // ulquiche_logique(ulquiche, &delta_time_ulquiche, can_gathering.clone()).await?;

        tokio::time::sleep(time::Duration::from_secs(1)).await;
        // break; // todo voir les conditions de break :)
    }
}

