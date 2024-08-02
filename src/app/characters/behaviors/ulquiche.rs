use std::sync::Arc;

use chrono::TimeDelta;

use crate::core::characters::Character;
use crate::core::services::can_gathering::CanGathering;

pub async fn ulquiche_logique(
    ulquiche: &Character,
    cooldown: &TimeDelta,
    can_gathering: Arc<Box<dyn CanGathering>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // waiting ulquiche cooldown

    if cooldown.num_milliseconds() > 0 {
        println!("ulquiche in cooldown for {} sec", cooldown.num_seconds());
    }

    if ulquiche.cooldown_expiration <= chrono::offset::Local::now() {
        println!("go recolter :)");
        match can_gathering.gathering(ulquiche)
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