use crate::states;
use bevy::prelude::*;
use dirs::data_dir;
use rusqlite::{Connection, Result};
use std::env::var_os;
use std::ffi::OsStr;
use std::path::PathBuf;

const THETAWAVE_DB_PATH_ENVVAR: &'static str = "THETAWAVE_DB_PATH";
const THETAWAVE_DB_FILE: &'static str = "thetawave.sqlite";
const USERSTAT: &'static str = "UserStat";
pub const DEFAULT_USER_ID: isize = 0;

fn default_db_path() -> PathBuf {
    let data_dir = data_dir().unwrap();
    let game_data_dir = data_dir.join("thetawave");
    std::fs::create_dir_all(&game_data_dir).unwrap();
    game_data_dir.join(THETAWAVE_DB_FILE)
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

pub fn get_db() -> Result<Connection> {
    let db_path = match var_os(OsStr::new(THETAWAVE_DB_PATH_ENVVAR)) {
        Some(osstr) => PathBuf::from(osstr),
        None => default_db_path(),
    };
    Connection::open(db_path)
}

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

/// Manages scanning of entities using the cursor
pub fn db_setup_system() {
    setup_db(get_db().unwrap()).unwrap();
}

pub fn inc_games_played_stat_system() {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {USERSTAT} (userId, totalGamesLost)
    VALUES (?1,  ?2)
    ON CONFLICT DO UPDATE SET totalGamesLost=totalGamesLost+?2"
    );
    let conn = get_db().unwrap();
    conn.prepare(&stmt_raw).unwrap().execute([DEFAULT_USER_ID, 1]).unwrap();
}
fn _get_games_lost_count_by_id(user_id: isize) -> Result<isize>{
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT totalGamesLost FROM  {USERSTAT} 
    WHERE userId=?1"
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([user_id])?;
    let maybe_row = rows.next()?;
    maybe_row.unwrap().get(0)
}

pub fn get_games_lost_count_by_id(user_id: isize) -> isize{
    _get_games_lost_count_by_id(user_id).unwrap_or(0)
}
