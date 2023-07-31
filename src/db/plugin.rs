use crate::game::CurrentGameMetrics;
/// Exposes a single Plugin that links the game and our persistence layer.
use crate::spawnable::{MobDestroyedEvent, MobType};
use crate::states;
use bevy::prelude::*;

use super::core::{get_db, setup_db, DEFAULT_USER_ID};
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
fn count_enemies_destroyed_system(mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>) {
    for event in mob_destroyed_event_reader.iter() {
        if let MobType::Enemy(enemy_type) = &event.mob_type {
            inc_mob_killed_count_for_user(DEFAULT_USER_ID, &enemy_type)
                .unwrap_or_else(|e| error!("Error incrementing mob kill count: {e}"));
        }
    }
}

fn flush_to_db_and_reset_current_game_metrics_system(
    mut current_game_metrics: ResMut<CurrentGameMetrics>,
) {
    inc_n_shots_fired_for_user_id(DEFAULT_USER_ID, current_game_metrics.n_shots_fired)
        .unwrap_or_else(|e| {
            error!(
                "Failed to flush per-run/game metrics to the database. Skipping. {}",
                e
            )
        });
    *current_game_metrics = CurrentGameMetrics::default();
}
impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::AppStates::LoadingAssets), db_setup_system);
        app.add_systems(
            OnEnter(states::AppStates::GameOver),
            inc_games_played_stat_system,
        );
        app.add_systems(Update, count_enemies_destroyed_system);
        app.add_systems(
            OnExit(states::AppStates::GameOver),
            flush_to_db_and_reset_current_game_metrics_system,
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
