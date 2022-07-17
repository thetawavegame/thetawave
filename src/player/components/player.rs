use bevy::prelude::*;

use crate::{misc::Health, player::Character, spawnable::ProjectileType};

/// Component for managing core attributes of the player
#[derive(Component, Debug)]
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
    pub attack_damage: f32,
    pub collision_damage: f32,
    pub attraction_distance: f32,
    pub attraction_acceleration: f32,
    pub money: usize,
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
            attack_damage: character.attack_damage,
            collision_damage: character.collision_damage,
            attraction_distance: character.attraction_distance,
            attraction_acceleration: character.attraction_acceleration,
            money: character.money,
        }
    }
}
