use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{misc::Health, player::components::AbilityType, spawnable::ProjectileType};

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
    /// Character type
    pub character_type: CharacterType,
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
    /// Health of the player
    pub health: Health,
    /// Amount of damage dealt per attack
    pub attack_damage: f32,
    /// Amount of damage dealt on contact
    pub collision_damage: f32,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and conumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    pub ability_period: f32,
    pub ability_type: AbilityType,
}

#[derive(Deserialize)]
pub enum CharacterType {
    Captain,
    Juggernaut,
}

/// Manages all characters
#[derive(Resource, Deserialize)]
pub struct CharactersResource {
    /// Names mapped to characters for all characters
    pub characters: HashMap<String, Character>,
}
