use crate::core::{get_db, OurDBError, OPTIONS_TABLE_NAME};
use bevy::log::error;
use rusqlite::Result;

use thetawave_interface::game::options::GameOptions;

fn _get_game_options() -> Result<Option<GameOptions>, OurDBError> {
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT bloom FROM {OPTIONS_TABLE_NAME}
        "
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([])?;
    match rows.next()? {
        Some(r) => {
            let bloom = r.get(0)?;
            Ok(Some(GameOptions { bloom }))
        }

        None => Ok(None),
    }
}

/// Returns all of the options in the game.
pub fn get_game_options() -> Option<GameOptions> {
    _get_game_options().unwrap_or_else(|err| {
        error!("Could not read game options. {}", &err);
        None
    })
}
