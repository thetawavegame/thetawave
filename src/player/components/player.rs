use bevy::prelude::*;
use serde::Deserialize;
use thetawave_interface::spawnable::ProjectileType;

use crate::player::Character;

/// Component for managing core attributes of the player
#[derive(Component, Debug, Clone)]
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
    /// Time between firing projectiles
    pub fire_period: f32,
    /// Amount of damage dealt per attack
    pub attack_damage: usize,
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and conumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    /// Timer for ability cooldown
    pub ability_cooldown_timer: Timer,
    /// Timer for ability action
    pub ability_action_timer: Option<Timer>,
    /// Type of ability
    pub ability_type: AbilityType,
    /// Whether the player responds to move inputs
    pub movement_enabled: bool,
    /// Multiplier for incoming damage
    pub incoming_damage_multiplier: f32,
    /// Index of the player
    pub player_index: usize,
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
            fire_timer: Timer::from_seconds(character.fire_period, TimerMode::Once),
            fire_period: character.fire_period,
            attack_damage: character.attack_damage,
            collision_damage: character.collision_damage,
            attraction_distance: character.attraction_distance,
            attraction_acceleration: character.attraction_acceleration,
            money: character.money,
            ability_cooldown_timer: Timer::from_seconds(character.ability_period, TimerMode::Once),
            ability_action_timer: None,
            ability_type: character.ability_type.clone(),
            movement_enabled: true,
            incoming_damage_multiplier: 1.0,
            player_index: 0,
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum AbilityType {
    Charge(f32),    // ability duration
    MegaBlast(f32), // scale and damage multiplier
}
