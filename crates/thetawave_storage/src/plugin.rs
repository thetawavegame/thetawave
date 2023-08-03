/// Exposes a single Plugin that links the game and our persistence layer.
use bevy::prelude::*;
use std::collections::BTreeMap;

use crate::user_stats::{
    get_mob_killed_counts_for_user, get_user_stats, set_user_stats_for_user_id,
};
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
};
use thetawave_interface::states;

use super::core::{get_db, setup_db};
use super::user_stats::{set_mob_killed_count_for_user, set_n_shots_fired_for_user_id};

/// Persist some user-specific stats and game state to a local SQLite database.
pub struct DBPlugin;

fn flush_user_stats_for_completed_games_to_db(
    shot_counters_for_current_game: Res<UserStatsByPlayerForCompletedGamesCache>,
) {
    if let Some(user_stats) = (*shot_counters_for_current_game).0.get(&DEFAULT_USER_ID) {
        set_user_stats_for_user_id(DEFAULT_USER_ID, user_stats).unwrap_or_else(|e| {
            error!(
                "Failed to flush per-run/game metrics to the database. Skipping. {}",
                e
            )
        });
    }
}
fn flush_mobs_killed_for_completed_games_counters_to_db(
    mobs_killed_for_current_game: Res<MobKillsByPlayerForCompletedGames>,
) {
    if let Some(mob_kills) = (**mobs_killed_for_current_game).get(&DEFAULT_USER_ID) {
        for (mob_type, n_killed) in mob_kills {
            set_mob_killed_count_for_user(DEFAULT_USER_ID, &mob_type, n_killed.clone())
                .unwrap_or_else(|e| error!("Error incrementing mob kill count: {e}"));
        }
    }
}
impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::AppStates::LoadingAssets), db_setup_system);
        app.add_systems(
            OnExit(states::AppStates::LoadingAssets),
            (load_user_stats_cache_from_db, load_mob_kills_cache_from_db),
        );
        app.add_systems(
            OnExit(states::AppStates::GameOver),
            (
                flush_user_stats_for_completed_games_to_db,
                flush_mobs_killed_for_completed_games_counters_to_db,
            ),
        );
    }
}
fn load_user_stats_cache_from_db(
    mut user_stats_cache: ResMut<UserStatsByPlayerForCompletedGamesCache>,
) {
    if !user_stats_cache.0.is_empty() {
        warn!(
            "evicting data from the in-memory cache. Is this right? {:?}",
            user_stats_cache
        );
    }
    user_stats_cache.0 = BTreeMap::from([(0, get_user_stats(DEFAULT_USER_ID).unwrap_or_default())]);
}
fn load_mob_kills_cache_from_db(mut mob_kills_cache: ResMut<MobKillsByPlayerForCompletedGames>) {
    if !mob_kills_cache.0.is_empty() {
        warn!(
            "evicting data from the in-memory mob kills cache. Is this right? {:?}",
            mob_kills_cache
        );
    }
    mob_kills_cache.0 = BTreeMap::from([(0, get_mob_killed_counts_for_user(DEFAULT_USER_ID))]);
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
