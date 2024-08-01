use chrono::TimeDelta;
use reqwest::Client;

use crate::app::characters::actions::gathering::gathering;
use crate::core::characters::Character;

pub async fn scalaman_logique(
    scalaman: &Character,
    http_client: &Client,
    url: &String,
    token: &String,
    cooldown: &TimeDelta,
) -> Result<(), Box<dyn std::error::Error>> {
    // waiting scalaman cooldown

    if cooldown.num_milliseconds() > 0 {
        println!("scalaman in cooldown for {} sec", cooldown.num_seconds());
    }

    if scalaman.cooldown_expiration <= chrono::offset::Local::now() {
        println!("go recolter :)");
        match gathering(&http_client, token.as_str(), url.as_str(), "ScalaMan")
            .await {
            Ok(()) => (),
            Err(e) => {
                println!("err : {e}");
            }
        }
    } else {
        println!("pas de recolte, je suis occupe")
    }

    Ok(())
}