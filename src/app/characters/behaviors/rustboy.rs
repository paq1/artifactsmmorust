use chrono::TimeDelta;
use reqwest::Client;
use crate::app::characters::actions::fight::fight;
use crate::core::characters::Character;

pub async fn rustboy_logique(
    rustboy: &Character,
    http_client: &Client,
    url: &String,
    token: &String,
    cooldown: &TimeDelta,
) -> Result<(), Box<dyn std::error::Error>> {
    // waiting rustboy cooldown

    if cooldown.num_milliseconds() > 0 {
        println!("rustboy in cooldown for {} sec", cooldown.num_seconds());
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

    Ok(())
}