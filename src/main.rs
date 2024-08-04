use std::sync::Arc;

use chrono::Utc;
use reqwest::Client;
use tokio::time;

use crate::app::characters::infos::fetch_characters;
use crate::app::map::infos::{fetch_maps, fetch_maps_from_position};
use crate::app::services::can_deposit_item_impl::CanDepositItemImpl;
use crate::app::services::can_fight_impl::CanFightImpl;
use crate::app::services::can_gathering_impl::CanGatheringImpl;
use crate::app::services::can_move_impl::CanMoveImpl;
use crate::app::services::can_withdraw_item_impl::CanWithdrawItemImpl;
use crate::core::behaviors::fight::FightBehavior;
use crate::core::behaviors::gathering::GatheringBehavior;
use crate::core::behaviors::deposit_bank::DepositBankBehavior;
use crate::core::behaviors::infinit_gathering::InfinitGateringBehavior;
use crate::core::behaviors::inifinit_fight::InfinitFight;
use crate::core::behaviors::moving::MovingBehavior;
use crate::core::behaviors::withdraw_bank::WithdrawBankBehavior;
use crate::core::services::can_deposit_item::CanDepositItem;
use crate::core::services::can_fight::CanFight;
use crate::core::services::can_gathering::CanGathering;
use crate::core::services::can_move::CanMove;
use crate::core::services::can_withdraw_item::CanWithdrawItem;
use crate::core::shared::Position;

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
        .expect("env variable `TOKEN_API_ARTIFACTSMMO` should be set by in .env or in docker-compose.yml env");

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
    let can_deposit_item: Arc<Box<dyn CanDepositItem>> = Arc::new(Box::new(CanDepositItemImpl {
        url: url.clone(),
        token: token.clone(),
        http_client: http_client.clone(),
    }));
    let can_withdraw_item: Arc<Box<dyn CanWithdrawItem>> = Arc::new(Box::new(CanWithdrawItemImpl {
        url: url.clone(),
        token: token.clone(),
        http_client: http_client.clone(),
    }));

    println!("chargement des chars");
    let players_init = fetch_characters(&http_client, &token, &url).await?.data;
    println!("chargement de la gamemap");
    let gamemaps = fetch_maps(&http_client, &token, &url, None).await?;

    println!("info lancement du bot");
    println!("count of players : {}", players_init.len());
    println!("count of gamemaps : {}", gamemaps.pagination.map(|p| p.total).unwrap_or(-1));

    // let rustboy_init = players_init.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();

    let cooper_maps = fetch_maps(&http_client, &token, &url, Some(vec![("content_code", "copper_rocks")])).await?;
    println!("cooper_maps len : {}", cooper_maps.data.len());
    let first_coopermaps = cooper_maps.data.first().unwrap();
    let cooper_position = first_coopermaps.get_position();

    // behaviors
    let moving_behavior_template: MovingBehavior = MovingBehavior::new(can_move.clone());
    let deposit_bank_behavior_template = DepositBankBehavior::new(
        can_deposit_item.clone(),
        moving_behavior_template.clone(),
    );
    let gathering_behavior_template = GatheringBehavior::new(can_gathering.clone());
    let fight_behavior_template = FightBehavior::new(can_fight.clone());
    let _withdraw_bank_behavior_template = WithdrawBankBehavior::new(
        can_withdraw_item.clone(),
        moving_behavior_template.clone(),
    );

    let mut rustboy_behavior = InfinitFight::new(
        &Position { x: 0, y: -1 },
        fight_behavior_template.clone(),
        deposit_bank_behavior_template.clone(),
        moving_behavior_template.clone(),
    );

    let mut scalaman_behavior = InfinitGateringBehavior::new(
        &cooper_position,
        gathering_behavior_template.clone(),
        deposit_bank_behavior_template.clone(),
        moving_behavior_template.clone(),
    );

    let mut ulquiche_behavior = InfinitGateringBehavior::new(
        &Position::new(-1, 0),
        gathering_behavior_template.clone(),
        deposit_bank_behavior_template.clone(),
        moving_behavior_template.clone(),
    );

    let mut jeanne_behavior = InfinitFight::new(
        &Position { x: 4, y: -1 },
        fight_behavior_template.clone(),
        deposit_bank_behavior_template.clone(),
        moving_behavior_template.clone(),
    );
    // let mut ulquiche_behavior = withdraw_bank_behavior_template.clone();

    let mut cerise_behavior = InfinitFight::new(
        &Position { x: 0, y: 1 },
        fight_behavior_template.clone(),
        deposit_bank_behavior_template.clone(),
        moving_behavior_template.clone(),
    );

    let coopermap = fetch_maps_from_position(
        &http_client.clone(),
        token.as_str(),
        url.as_str(),
        &Position::new(2, 0),
    ).await;

    println!("gamemap cooper {coopermap:?}");


    loop {
        let fetch_characters_query = fetch_characters(&http_client, &token, &url).await;
        match fetch_characters_query {
            Ok(many) => {
                let players_updated = many.data;
                let rustboy = players_updated.iter().find(|e| e.name == "RustBoy".to_string()).unwrap();
                let scalaman = players_updated.iter().find(|e| e.name == "ScalaMan".to_string()).unwrap();
                let ulquiche = players_updated.iter().find(|e| e.name == "Ulquiche".to_string()).unwrap();
                let cerise = players_updated.iter().find(|e| e.name == "Cerise".to_string()).unwrap();
                let jeanne = players_updated.iter().find(|e| e.name == "Jeanne".to_string()).unwrap();
                let now = Utc::now();

                let lowest_cooldown = players_updated
                    .iter()
                    // .filter(|p| p.name != "Jeanne".to_string())
                    .map(|p| {
                        let cooldown = (p.cooldown_expiration - now).num_seconds();
                        println!("cooldown for {} is {}", p.name, cooldown);
                        cooldown
                    })
                    .min()
                    .unwrap_or(1);

                if lowest_cooldown >= 0 {
                    println!("waiting {lowest_cooldown}");
                    tokio::time::sleep(time::Duration::from_secs(lowest_cooldown as u64)).await;
                }


                let next_beavior_rustboy = rustboy_behavior.next_behavior(
                    &rustboy
                ).await?;
                rustboy_behavior = next_beavior_rustboy;

                let next_beavior_scalaman = scalaman_behavior.next_behavior(
                    &scalaman
                ).await?;
                scalaman_behavior = next_beavior_scalaman;

                let next_beavior_ulquiche = ulquiche_behavior.next_behavior(
                    &ulquiche
                ).await?;
                ulquiche_behavior = next_beavior_ulquiche;

                let next_beavior_cerise = cerise_behavior.next_behavior(
                    &cerise
                ).await?;
                cerise_behavior = next_beavior_cerise;

                let next_beavior_jeanne = jeanne_behavior.next_behavior(
                    &jeanne
                ).await?;
                jeanne_behavior = next_beavior_jeanne;
            }
            Err(e) => {
                println!("[SERVER] no fetch for characters, we wait 30 sec for next call");
                println!("[SERVER] details : {e:?}");
                tokio::time::sleep(time::Duration::from_secs(30)).await;
            }
        };

    }
}

