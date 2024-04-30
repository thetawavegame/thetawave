use crate::core::{get_db, OurDBError, OPTIONS_TABLE_NAME};
use bevy::log::error;
use rusqlite::Result;

use thetawave_interface::game::options::GameOptions;

fn _get_game_options(options_profile_id: usize) -> Result<Option<GameOptions>, OurDBError> {
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT bloomEnabled, bloomIntensity, tutorialsEnabled FROM {OPTIONS_TABLE_NAME}
    WHERE optionsProfileId=?1
        "
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([options_profile_id])?;
    match rows.next()? {
        Some(r) => {
            let bloom_enabled = r.get(0)?;
            let bloom_intensity = r.get(1)?;
            let tutorials_enabled = r.get(2)?;
            Ok(Some(GameOptions {
                bloom_enabled,
                bloom_intensity,
                tutorials_enabled,
            }))
        }

        None => Ok(None),
    }
}

/// Returns all of the options in the game.
pub fn get_game_options(options_profile_id: usize) -> Option<GameOptions> {
    _get_game_options(options_profile_id).unwrap_or_else(|err| {
        error!("Could not read game options. {}", &err);
        None
    })
}
