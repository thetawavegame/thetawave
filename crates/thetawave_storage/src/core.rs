use bevy::log::info;
use directories::ProjectDirs;
use rusqlite::{self, Connection};
use std::env::var_os;
use std::ffi::OsStr;
use std::path::PathBuf;
use thetawave_interface::game::options::DEFAULT_OPTIONS_PROFILE_ID;
use thiserror::Error;
pub(super) const THETAWAVE_DB_PATH_ENVVAR: &'static str = "THETAWAVE_DB_PATH";
const THETAWAVE_DB_FILE: &'static str = "thetawave.sqlite";
pub(super) const USERSTAT: &'static str = "UserStat";
pub(super) const ENEMY_KILL_HISTORY_TABLE_NAME: &'static str = "EnemiesKilled";
pub(super) const OPTIONS_TABLE_NAME: &'static str = "Options";

#[derive(Error, Debug, derive_more::From)]
pub(super) enum OurDBError {
    #[error(
        "No suitable location found for the user stats database. Is this a supported platform?"
    )]
    #[from(ignore)]
    NoDBPathFound,
    #[error("Sqlite Error: {0}")]
    SqliteError(rusqlite::Error),
    #[error("Failed to access sqlite file: {0}")]
    LocalFilesystemError(std::io::Error),
    #[error("Internal database error. Please report as a bug. {0}")]
    #[from(ignore)]
    InternalError(String),
}

fn default_db_path() -> Result<PathBuf, OurDBError> {
    match ProjectDirs::from("org", "thetawave-game", "thetawave") {
        Some(pdirs) => {
            let game_data_dir = pdirs.data_local_dir();
            std::fs::create_dir_all(&game_data_dir)?;
            Ok(game_data_dir.join(THETAWAVE_DB_FILE))
        }
        None => Err(OurDBError::NoDBPathFound),
    }
}

pub(super) fn setup_db(conn: Connection) -> rusqlite::Result<()> {
    let create_user_stats_sql = format!(
        "CREATE TABLE IF NOT EXISTS {USERSTAT} (
        userId INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
        totalShotsFired  INTEGER NOT NULL DEFAULT 0,
        totalShotsHit  INTEGER NOT NULL DEFAULT 0,
        totalGamesLost INTEGER NOT NULL DEFAULT 0
    )"
    );

    let create_enemies_killed_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS {ENEMY_KILL_HISTORY_TABLE_NAME} (
        userId INTEGER NOT NULL,
        enemyMobType VARCHAR(255) NOT NULL,
        nKilled INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (userId, enemyMobType)
    )"
    );

    let create_options_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS {OPTIONS_TABLE_NAME} (
        optionsProfileId INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
        bloomEnabled BOOLEAN NOT NULL DEFAULT TRUE,
        bloomIntensity REAL NOT NULL DEFAULT 1.0
    )"
    );

    conn.execute(&create_user_stats_sql, []).map(|_| ())?;
    conn.execute(&create_enemies_killed_table_sql, [])
        .map(|_| ())?;
    conn.execute(&create_options_table_sql, []).map(|_| ())?;

    // insert a default options row if it is not in the db
    let upsert_default_gameops_sql =
        format!("INSERT OR REPLACE INTO {OPTIONS_TABLE_NAME} (optionsProfileId) VALUES (?1)");
    conn.execute(&upsert_default_gameops_sql, [DEFAULT_OPTIONS_PROFILE_ID])
        .map(|_| ())?;

    info!("Created sqlite db");
    Ok(())
}

pub(super) fn get_db() -> Result<Connection, OurDBError> {
    let db_path = match var_os(OsStr::new(THETAWAVE_DB_PATH_ENVVAR)) {
        Some(osstr) => Ok(PathBuf::from(osstr)),
        None => default_db_path(),
    }?;
    Connection::open(db_path).map_err(OurDBError::from)
}
