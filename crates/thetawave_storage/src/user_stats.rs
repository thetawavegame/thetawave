use crate::core::{get_db, OurDBError, ENEMY_KILL_HISTORY_TABLE_NAME, USERSTAT};
use bevy::prelude::{error, info};
use rusqlite::{params, Error, Result};
use std::collections::HashMap;
use thetawave_interface::spawnable::EnemyMobType;

use thetawave_interface::historical_metrics::UserStat;
pub(super) fn inc_games_played_stat(user_id: usize) -> Result<(), OurDBError> {
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

pub(super) fn inc_n_shots_fired_for_user_id(
    user_id: usize,
    n_shots: usize,
) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {USERSTAT} (userId, totalShotsFired)
    VALUES (?1,  ?2)
    ON CONFLICT DO UPDATE SET totalShotsFired=totalShotsFired+?2"
    );
    let conn = get_db()?;
    info!(
        "Preparing db upsert {} with param n_shots={}",
        &stmt_raw, n_shots
    );
    conn.prepare(&stmt_raw)?
        .execute(params![user_id, n_shots])?;
    Ok(())
}
fn _get_games_lost_count_by_id(user_id: usize) -> Result<usize, OurDBError> {
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

fn _get_user_stats(user_id: usize) -> Result<UserStat, OurDBError> {
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT userId, totalGamesLost, totalShotsFired FROM  {USERSTAT} 
    WHERE userId=?1"
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([user_id])?;
    match rows.next()? {
        Some(r) => {
            let user_id: usize = r.get(0)?;
            let total_games_lost = r.get(1)?;
            let total_shots_fired = r.get(2)?;
            Ok(UserStat {
                user_id,
                total_games_lost,
                total_shots_fired,
            })
        }

        None => Err(OurDBError::InternalError(String::from("User not found"))),
    }
}

/// Returns the user stats for games that have already been completed and flushed to the db.
pub fn get_user_stats(user_id: usize) -> Option<UserStat> {
    match _get_user_stats(user_id) {
        Err(err) => {
            error!(
                "Could not read user stats. Falling back to zeroed out defaults. {}",
                err
            );
            None
        }
        Ok(user_stat) => Some(user_stat),
    }
}

pub fn get_games_lost_count_by_id(user_id: usize) -> usize {
    _get_games_lost_count_by_id(user_id).unwrap_or(0)
}

pub(super) fn inc_mob_killed_count_for_user(
    user_id: usize,
    mob_type: &EnemyMobType,
    amount: usize,
) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {ENEMY_KILL_HISTORY_TABLE_NAME} (userId, enemyMobType, nKilled)
    VALUES (?1,  ?2, ?3)
    ON CONFLICT DO UPDATE SET nKilled=nKilled+?3"
    );
    let conn = get_db()?;
    conn.prepare(&stmt_raw)?
        .execute(params![user_id, mob_type.to_string(), amount])?;
    Ok(())
}

fn _get_mob_killed_counts_for_user(user_id: usize) -> Result<HashMap<String, usize>, OurDBError> {
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
        let b = r.get::<usize, usize>(1)?;
        Ok((a, b))
    })
    .collect::<Result<HashMap<String, usize>, Error>>()
    .map_err(OurDBError::from)
}

pub fn get_mob_killed_counts_for_user(user_id: usize) -> HashMap<String, usize> {
    _get_mob_killed_counts_for_user(user_id).unwrap_or_else(|e| {
        error!(
            "Failed to get mob kill counts from db. Empty result fallback. {}",
            e
        );
        HashMap::default()
    })
}
