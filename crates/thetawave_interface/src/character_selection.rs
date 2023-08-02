use bevy_ecs_macros::Event;

/// Stores the index (likely 0 or 1) of the player that joined an n-player game.
#[derive(Event)]
pub struct PlayerJoinEvent(pub usize);
