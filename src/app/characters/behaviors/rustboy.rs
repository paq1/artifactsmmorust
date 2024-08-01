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
    mut current_action: &mut String
) -> Result<(), Box<dyn std::error::Error>> {
    // waiting rustboy cooldown

    // run rustboy fight
    if rustboy.cooldown_expiration <= chrono::offset::Local::now() {
        println!("go fight :)");
        if *current_action == "nothing".to_string() {
            match fight(&http_client, token.as_str(), url.as_str(), "RustBoy")
                .await {
                Ok(()) => {
                    current_action = &mut "fight".to_string();
                    ()
                },
                Err(e) => {
                    println!("err : {e}");
                }
            }
        }
    } else {
        current_action = &mut "nothing".to_string();
        println!("rustboy in cooldown for {} sec", cooldown.num_seconds());
    }

    Ok(())
}