use std::sync::Arc;

use reqwest::Client;
use tokio::time;

use crate::app::characters::infos::fetch_characters;
use crate::app::map::infos::fetch_maps;
use crate::app::services::can_fight_impl::CanFightImpl;
use crate::app::services::can_gathering_impl::CanGatheringImpl;
use crate::app::services::can_move_impl::CanMoveImpl;
use crate::core::behaviors::infinit_fight::InfinitFight;
use crate::core::behaviors::infinit_gathering_cooper::InfinitGatheringCooper;
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

    let rustboy_init = players_init.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();
    let scalaman_init = players_init.iter().find(|e| e.name == "ScalaMan".to_string()).unwrap();
    let ulquiche_init = players_init.iter().find(|e| e.name == "Ulquiche".to_string()).unwrap();


    let mut rustboy_behavior = InfinitFight::new(
        rustboy_init.clone(),
        can_fight.clone(),
        can_move.clone(),
    );

    let mut scalaman_behavior = InfinitGatheringCooper::new(
        scalaman_init.clone(),
        can_gathering.clone(),
        can_move.clone(),
    );

    let mut ulquiche_behavior = InfinitGatheringCooper::new(
        ulquiche_init.clone(),
        can_gathering.clone(),
        can_move.clone(),
    );

    loop {
        let players_updated = fetch_characters(&http_client, &token, &url).await?.data;
        let rustboy = players_updated.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();
        let scalaman = players_updated.iter().find(|e| e.name == "ScalaMan".to_string()).unwrap();
        let ulquiche = players_updated.iter().find(|e| e.name == "Ulquiche".to_string()).unwrap();

        let next_beavior_rustboy = rustboy_behavior.run().await?;
        rustboy_behavior = InfinitFight {
            character_info: rustboy.clone(),
            ..next_beavior_rustboy.clone()
        };

        let next_beavior_scalaman = scalaman_behavior.run().await?;
        scalaman_behavior = InfinitGatheringCooper {
            character_info: scalaman.clone(),
            ..next_beavior_scalaman.clone()
        };

        let next_beavior_ulquiche = ulquiche_behavior.run().await?;
        ulquiche_behavior = InfinitGatheringCooper {
            character_info: ulquiche.clone(),
            ..next_beavior_ulquiche.clone()
        };


        tokio::time::sleep(time::Duration::from_secs(1)).await;
        // break; // todo voir les conditions de break :)
    }
}

