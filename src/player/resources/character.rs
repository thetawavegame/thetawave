use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::spawnable::ProjectileType;

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
    /// Projectile type
    pub projectile_type: ProjectileType,
    /// Time until fired projectile despawns
    pub projectile_despawn_time: f32,
    /// Velocity of fired projectile
    pub projectile_velocity: Vec2,
    /// Position of projectile spawn relative to player
    pub projectile_offset_position: Vec2,
    /// Period of time between firing blasts
    pub fire_period: f32,
}

/// Manages all characters
#[derive(Deserialize)]
pub struct CharactersResource {
    /// Names mapped to characters for all characters
    pub characters: HashMap<String, Character>,
}
