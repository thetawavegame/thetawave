use crate::states;
use bevy::prelude::*;
use dirs::data_dir;
use rusqlite::{Connection, Error, Result};
use std::env::var_os;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

const THETAWAVE_DB_PATH_ENVVAR: &'static str = "THETAWAVE_DB_PATH";
const THETAWAVE_DB_FILE: &'static str = "thetawave.sqlite";
const USERSTAT: &'static str = "UserStat";
pub const DEFAULT_USER_ID: isize = 0;
#[derive(Error, Debug)]
enum OurDBError {
    #[error(
        "No suitable location found for the user stats database. Is this a supported platform?"
    )]
    NoDBPathFound,
    #[error("Sqlite Error: {0}")]
    SqliteError(Error),
    #[error("Failed to access sqlite file: {0}")]
    LocalFilesystemError(std::io::Error),
}
impl From<Error> for OurDBError {
    fn from(value: Error) -> Self {
        OurDBError::SqliteError(value)
    }
}
impl From<std::io::Error> for OurDBError {
    fn from(value: std::io::Error) -> Self {
        OurDBError::LocalFilesystemError(value)
    }
}
fn default_db_path() -> Result<PathBuf, OurDBError> {
    let data_dir = data_dir().ok_or(OurDBError::NoDBPathFound)?;
    let game_data_dir = data_dir.join("thetawave");
    std::fs::create_dir_all(&game_data_dir)?;
    Ok(game_data_dir.join(THETAWAVE_DB_FILE))
}
pub fn setup_db(conn: Connection) -> Result<()> {
    let create_user_stats_sql = format!(
        "CREATE TABLE IF NOT EXISTS {USERSTAT} (
        userId INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
        totalShotsFired  INTEGER NOT NULL DEFAULT 0,
        totalGamesLost INTEGER NOT NULL DEFAULT 0
    )"
    );
    let res = conn.execute(&create_user_stats_sql, []).map(|_| ());
    println!("Created sqlite db");
    res
}

fn get_db() -> Result<Connection, OurDBError> {
    let db_path = match var_os(OsStr::new(THETAWAVE_DB_PATH_ENVVAR)) {
        Some(osstr) => Ok(PathBuf::from(osstr)),
        None => default_db_path(),
    }?;
    Connection::open(db_path).map_err(OurDBError::from)
}

/// Persist some user-specific stats and game state to a local SQLite database.
pub struct DBPlugin;

impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::AppStates::LoadingAssets), db_setup_system);
        app.add_systems(
            OnEnter(states::AppStates::GameOver),
            inc_games_played_stat_system,
        );
    }
}

fn db_setup_system() {
    match get_db() {
        Ok(conn) => setup_db(conn).unwrap_or_else(|e| {
            println!("{e}");
        }),
        Err(e) => {
            println!("{e}");
        }
    };
}

fn _inc_games_played_stat_impl() -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {USERSTAT} (userId, totalGamesLost)
    VALUES (?1,  ?2)
    ON CONFLICT DO UPDATE SET totalGamesLost=totalGamesLost+?2"
    );
    let conn = get_db()?;
    conn.prepare(&stmt_raw)?.execute([DEFAULT_USER_ID, 1])?;
    Ok(())
}
fn inc_games_played_stat_system() {
    match _inc_games_played_stat_impl() {
        Ok(_) => {}
        Err(e) => {
            println!("{e}");
        }
    };
}
fn _get_games_lost_count_by_id(user_id: isize) -> Result<isize, OurDBError> {
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT totalGamesLost FROM  {USERSTAT} 
    WHERE userId=?1"
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([user_id])?;
    match rows.next()? {
        Some(r) => r.get(0).map_err(OurDBError::from),
        None => Ok(0),
    }
}

pub fn get_games_lost_count_by_id(user_id: isize) -> isize {
    _get_games_lost_count_by_id(user_id).unwrap_or(0)
}
