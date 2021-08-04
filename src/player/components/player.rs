use crate::player::Character;
use bevy::prelude::*;
use serde::Deserialize;

/// Component for managing core attributes of the player
#[derive(Deserialize, Debug)]
pub struct PlayerComponent {
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
}

impl From<&Character> for PlayerComponent {
    fn from(character: &Character) -> Self {
        PlayerComponent {
            acceleration: character.acceleration,
            deceleration: character.deceleration,
            speed: character.speed,
        }
    }
}
