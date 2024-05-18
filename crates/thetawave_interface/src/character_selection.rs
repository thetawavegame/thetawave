use bevy_ecs_macros::Event;

use crate::player::PlayerInput;

/// Stores the index (likely 0 or 1) of the player that joined an n-player game.
#[derive(Event)]
pub struct PlayerJoinEvent {
    pub player_idx: u8,
    pub input: PlayerInput,
}
