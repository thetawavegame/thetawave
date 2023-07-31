use crate::db::core::{get_db, OurDBError, ENEMY_KILL_HISTORY_TABLE_NAME, USERSTAT};
use crate::spawnable::EnemyMobType;
use bevy::prelude::error;
use rusqlite::{params, Error, Result};
pub(super) fn inc_games_played_stat(user_id: isize) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {USERSTAT} (userId, totalGamesLost)
    VALUES (?1,  ?2)
    ON CONFLICT DO UPDATE SET totalGamesLost=totalGamesLost+?2"
    );
    let conn = get_db()?;
    conn.prepare(&stmt_raw)?.execute([user_id, 1])?;
    Ok(())
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

pub(super) fn inc_mob_killed_count_for_user(
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
        .execute(params![user_id, mob_type.to_string()])?;
    Ok(())
}

fn _get_mob_killed_counts_for_user(user_id: isize) -> Result<Vec<(String, isize)>, OurDBError> {
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

pub fn get_mob_killed_counts_for_user(user_id: isize) -> Vec<(String, isize)> {
    _get_mob_killed_counts_for_user(user_id).unwrap_or_else(|e| {
        error!(
            "Failed to get mob kill counts from db. Empty result fallback. {}",
            e
        );
        Vec::default()
    })
}
