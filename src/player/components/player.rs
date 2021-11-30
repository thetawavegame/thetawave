use crate::{player::Character, spawnable::ProjectileType};
use bevy::prelude::*;

use crate::spawnable::Health;

/// Component for managing core attributes of the player
#[derive(Debug)]
pub struct PlayerComponent {
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
    /// Collider dimensions
    pub collider_dimensions: Vec2,
    /// Collider density
    pub collider_density: f32,
    /// Type of projectile fired
    pub projectile_type: ProjectileType,
    /// Time until fired projectile despawns
    pub projectile_despawn_time: f32,
    /// Velocity of fired projectile
    pub projectile_velocity: Vec2,
    /// Position of projectile spawn relative to player
    pub projectile_offset_position: Vec2,
    /// Tracks time between firing blasts
    pub fire_timer: Timer,
    pub health: Health,
}

impl From<&Character> for PlayerComponent {
    fn from(character: &Character) -> Self {
        PlayerComponent {
            acceleration: character.acceleration,
            deceleration: character.deceleration,
            speed: character.speed,
            collider_dimensions: character.collider_dimensions,
            collider_density: character.collider_density,
            projectile_type: character.projectile_type.clone(),
            projectile_despawn_time: character.projectile_despawn_time,
            projectile_velocity: character.projectile_velocity,
            projectile_offset_position: character.projectile_offset_position,
            fire_timer: Timer::from_seconds(character.fire_period, false),
            health: character.health.clone(),
        }
    }
}
