//! Systems for managing players

mod attacks;
mod movement;

use crate::states::AppStates;
use bevy::prelude::*;

pub use self::attacks::player_fire_weapon_system;
pub use self::movement::player_movement_system;

use super::PlayerComponent;

pub fn player_death_system(
    player_query: Query<&PlayerComponent>,
    mut app_state: ResMut<State<AppStates>>,
) {
    for player in player_query.iter() {
        if player.health.is_dead() {
            app_state.set(AppStates::GameOver).unwrap();
        }
    }
}
