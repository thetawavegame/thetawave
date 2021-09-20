use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Contains data necessary to create a player entity.
/// A character is chosen at the beginning of the game.
/// The base stats of the player are provided from the character.
/// Other data such as sprite sheets are also included with the character.
#[derive(Deserialize)]
pub struct Character {
    /// Base acceleration
    pub acceleration: Vec2,
    /// Base deceleration
    pub deceleration: Vec2,
    /// Base speed
    pub speed: Vec2,
    /// Collider size (relative to the sprite size)
    pub collider_dimensions: Vec2,
    /// Density of the collider (mass of collider is proportional to its size)
    pub collider_density: f32,
    /// Sprite sheet path
    pub sprite_path: String,
}

/// Manages all characters
#[derive(Deserialize)]
pub struct CharactersResource {
    /// Names mapped to characters for all characters
    pub characters: HashMap<String, Character>,
}
