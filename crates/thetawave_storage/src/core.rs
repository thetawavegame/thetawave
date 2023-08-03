use bevy::prelude::info;
use dirs::data_dir;
use rusqlite;
use rusqlite::Connection;
use std::env::var_os;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;
const THETAWAVE_DB_PATH_ENVVAR: &'static str = "THETAWAVE_DB_PATH";
const THETAWAVE_DB_FILE: &'static str = "thetawave.sqlite";
pub(super) const USERSTAT: &'static str = "UserStat";
pub(super) const ENEMY_KILL_HISTORY_TABLE_NAME: &'static str = "EnemiesKilled";

#[derive(Error, Debug)]
pub(super) enum OurDBError {
    #[error(
        "No suitable location found for the user stats database. Is this a supported platform?"
    )]
    NoDBPathFound,
    #[error("Sqlite Error: {0}")]
    SqliteError(rusqlite::Error),
    #[error("Failed to access sqlite file: {0}")]
    LocalFilesystemError(std::io::Error),
    #[error("Internal database error. Please report as a bug. {0}")]
    InternalError(String),
}
impl From<rusqlite::Error> for OurDBError {
    fn from(value: rusqlite::Error) -> Self {
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
    conn.execute(&create_user_stats_sql, []).map(|_| ())?;
    conn.execute(&create_enemies_killed_table_sql, [])
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
