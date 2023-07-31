/// Exposes a single Plugin that links the game and our persistence layer.
use crate::spawnable::{MobDestroyedEvent, MobType};
use crate::states;
use bevy::prelude::*;

use super::core::{get_db, setup_db, DEFAULT_USER_ID};
use super::user_stats::{inc_games_played_stat, inc_mob_killed_count_for_user};

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
