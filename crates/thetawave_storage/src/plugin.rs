/// Exposes a single Plugin that links the game and our persistence layer.
use bevy::prelude::*;

use thetawave_interface::game::counters::{EnemiesKilledCounter, ShotCounters};
use thetawave_interface::historical_metrics::DEFAULT_USER_ID;
use thetawave_interface::states;

use super::core::{get_db, setup_db};
use super::user_stats::{
    inc_games_played_stat, inc_mob_killed_count_for_user, inc_n_shots_fired_for_user_id,
};

/// Persist some user-specific stats and game state to a local SQLite database.
pub struct DBPlugin;

fn inc_games_played_stat_system() {
    match inc_games_played_stat(DEFAULT_USER_ID) {
        Ok(_) => {}
        Err(e) => {
            println!("Error updating game stats: {e}");
        }
    };
}

fn flush_to_db_and_reset_current_game_metrics_system(
    mut shot_counters_for_current_game: ResMut<ShotCounters>,
) {
    inc_n_shots_fired_for_user_id(
        DEFAULT_USER_ID,
        shot_counters_for_current_game.n_shots_fired,
    )
    .unwrap_or_else(|e| {
        error!(
            "Failed to flush per-run/game metrics to the database. Skipping. {}",
            e
        )
    });
    *shot_counters_for_current_game = ShotCounters::default();
}
fn flush_mobs_killed_for_current_game_to_db_and_reset_counters(
    mut mobs_killed_for_current_game: ResMut<EnemiesKilledCounter>,
) {
    for (mob_type, n_killed) in mobs_killed_for_current_game.0.iter() {
        inc_mob_killed_count_for_user(DEFAULT_USER_ID, &mob_type, n_killed.clone())
            .unwrap_or_else(|e| error!("Error incrementing mob kill count: {e}"));
    }
    *mobs_killed_for_current_game = EnemiesKilledCounter::default();
}
impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::AppStates::LoadingAssets), db_setup_system);
        app.add_systems(
            OnEnter(states::AppStates::GameOver),
            inc_games_played_stat_system,
        );
        app.add_systems(
            OnExit(states::AppStates::GameOver),
            (
                flush_to_db_and_reset_current_game_metrics_system,
                flush_mobs_killed_for_current_game_to_db_and_reset_counters,
            ),
        );
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
