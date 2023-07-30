use crate::spawnable::{EnemyMobType, MobDestroyedEvent};
use crate::states;
use bevy::prelude::*;
use dirs::data_dir;
use rusqlite::{params, Connection, Error, Result};
use std::env::var_os;
use std::ffi::OsStr;
use std::path::PathBuf;
use thiserror::Error;

const THETAWAVE_DB_PATH_ENVVAR: &'static str = "THETAWAVE_DB_PATH";
const THETAWAVE_DB_FILE: &'static str = "thetawave.sqlite";
const USERSTAT: &'static str = "UserStat";
const ENEMY_KILL_HISTORY_TABLE_NAME: &'static str = "EnemiesKilled";

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
        app.add_systems(Update, count_enemies_destroyed_system);
    }
}

fn db_setup_system() {
    match get_db() {
        Ok(conn) => setup_db(conn).unwrap_or_else(|e| {
            error!("{e}");
        }),
        Err(e) => {
            error!("{e}");
        }
    };
}

fn inc_mob_killed_count_for_user(
    user_id: isize,
    mob_type: &EnemyMobType,
) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {ENEMY_KILL_HISTORY_TABLE_NAME} (userId, enemyMobType, nKilled)
    VALUES (?1,  ?2, 1)
    ON CONFLICT DO UPDATE SET nKilled=nKilled+1"
    );
    let conn = get_db()?;
    conn.prepare(&stmt_raw)?
        .execute(params![DEFAULT_USER_ID, mob_type.to_string()])?;
    Ok(())
}

fn get_mob_killed_counts_for_user(user_id: isize) -> Result<Vec<(String, isize)>, OurDBError> {
    let stmt_raw = format!(
        "
    SELECT enemyMobType, nKilled FROM  {ENEMY_KILL_HISTORY_TABLE_NAME} 
    WHERE userId=?1
    ORDER BY enemyMobType LIMIT 50"
    );
    let conn = get_db()?;
    let mut stmt = conn.prepare(&stmt_raw)?;
    let rows = stmt.query([user_id])?;
    rows.mapped(|r| {
        let a = r.get::<usize, String>(0)?;
        let b = r.get::<usize, isize>(1)?;
        Ok((a, b))
    })
    .collect::<Result<Vec<(String, isize)>, Error>>()
    .map_err(OurDBError::from)
}

pub fn print_mob_kills(user_id: isize) -> String {
    match get_mob_killed_counts_for_user(user_id) {
        Ok(mob_kills) => mob_kills
            .into_iter()
            .map(|(mobtype, n)| format!("{mobtype}: {n}"))
            .collect::<Vec<String>>()
            .join("\n"),
        Err(e) => {
            error!("{e}");
            String::default()
        }
    }
}

fn count_enemies_destroyed_system(mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>) {
    for event in mob_destroyed_event_reader.iter() {
        if let crate::spawnable::MobType::Enemy(enemy_type) = &event.mob_type {
            inc_mob_killed_count_for_user(DEFAULT_USER_ID, &enemy_type)
                .unwrap_or_else(|e| error!("Error incrementing mob kill count: {e}"));
        }
    }
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
            println!("Error updating game stats: {e}");
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
