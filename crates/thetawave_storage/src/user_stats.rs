use crate::core::{get_db, OurDBError, ENEMY_KILL_HISTORY_TABLE_NAME, USERSTAT};
use bevy::prelude::{error, info};
use rusqlite::{params, Result};
use thetawave_interface::spawnable::EnemyMobType;

use thetawave_interface::game::historical_metrics::{MobsKilledBy1PlayerCacheT, UserStat};

pub(super) fn set_user_stats_for_user_id(
    user_id: usize,
    user_stats: &UserStat,
) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {USERSTAT} (userId, totalShotsFired, totalGamesLost)
    VALUES (?1,  ?2, ?3)
    ON CONFLICT DO UPDATE SET totalShotsFired=?2, totalGamesLost=?3"
    );
    let conn = get_db()?;
    info!(
        "Preparing db upsert {} with param n_shots={}",
        &stmt_raw, user_stats.total_shots_fired
    );
    conn.prepare(&stmt_raw)?.execute(params![
        user_id,
        user_stats.total_shots_fired,
        user_stats.total_games_lost
    ])?;
    Ok(())
}
fn _get_user_stats(user_id: usize) -> Result<Option<UserStat>, OurDBError> {
    let conn = get_db()?;
    let stmt_raw = format!(
        "
    SELECT totalGamesLost, totalShotsFired, totalShotsHit FROM  {USERSTAT}
    WHERE userId=?1"
    );
    let mut stmt = conn.prepare(&stmt_raw)?;
    let mut rows = stmt.query([user_id])?;
    match rows.next()? {
        Some(r) => {
            let total_games_lost = r.get(0)?;
            let total_shots_fired = r.get(1)?;
            let total_shots_hit = r.get(2)?;
            Ok(Some(UserStat {
                total_games_lost,
                total_shots_fired,
                total_shots_hit,
            }))
        }

        None => Ok(None),
    }
}

/// Returns the user stats for games that have already been completed and flushed to the db.
pub fn get_user_stats(user_id: usize) -> Option<UserStat> {
    _get_user_stats(user_id).unwrap_or_else(|err| {
        error!(
            "Could not read user stats. Falling back to zeroed out defaults. {}",
            &err
        );
        None
    })
}

pub(super) fn set_mob_killed_count_for_user(
    user_id: usize,
    mob_type: &EnemyMobType,
    amount: usize,
) -> Result<(), OurDBError> {
    let stmt_raw = format!(
        "
    INSERT OR REPLACE INTO {ENEMY_KILL_HISTORY_TABLE_NAME} (userId, enemyMobType, nKilled)
    VALUES (?1,  ?2, ?3)
    ON CONFLICT DO UPDATE SET nKilled=?3"
    );
    let conn = get_db()?;
    conn.prepare(&stmt_raw)?
        .execute(params![user_id, mob_type.to_string(), amount])?;
    Ok(())
}

fn _get_mob_killed_counts_for_user(
    user_id: usize,
) -> Result<MobsKilledBy1PlayerCacheT, OurDBError> {
    let stmt_raw = format!(
        "
    SELECT enemyMobType, nKilled FROM  {ENEMY_KILL_HISTORY_TABLE_NAME} 
    WHERE userId=?1
    ORDER BY enemyMobType LIMIT 50"
    );
    let conn = get_db()?;
    let mut stmt = conn.prepare(&stmt_raw)?;
    let rows = stmt.query([user_id])?;
    Ok(rows
        .mapped(|r| {
            let a = r.get::<usize, String>(0)?;
            let b = r.get::<usize, usize>(1)?;
            Ok((a, b))
        })
        .collect::<Result<Vec<(String, usize)>, rusqlite::Error>>()?
        .into_iter()
        .map(|(mob, n)| {
            Ok((
                mob.parse::<EnemyMobType>().map_err(|e| {
                    OurDBError::InternalError(format!("Failed to read mob data from db {}", e))
                })?,
                n,
            ))
        })
        .collect::<Result<Vec<(EnemyMobType, usize)>, OurDBError>>()?
        .into_iter()
        // The DB primary key guarantees that translating from a vec -> hashmap doesnt lose elements
        .collect())
}

pub fn get_mob_killed_counts_for_user(user_id: usize) -> MobsKilledBy1PlayerCacheT {
    _get_mob_killed_counts_for_user(user_id).unwrap_or_else(|e| {
        error!(
            "Failed to get mob kill counts from db. Empty result fallback. {}",
            e
        );
        Default::default()
    })
}
