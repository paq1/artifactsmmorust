use std::sync::Arc;

use chrono::TimeDelta;

use crate::core::characters::Character;
use crate::core::services::can_gathering::CanGathering;

pub async fn scalaman_logique(
    scalaman: &Character,
    cooldown: &TimeDelta,
    can_gathering: Arc<Box<dyn CanGathering>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // waiting scalaman cooldown

    if cooldown.num_milliseconds() > 0 {
        println!("scalaman in cooldown for {} sec", cooldown.num_seconds());
    }

    if scalaman.cooldown_expiration <= chrono::offset::Local::now() {
        println!("go recolter :)");
        match can_gathering.gathering(scalaman)
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