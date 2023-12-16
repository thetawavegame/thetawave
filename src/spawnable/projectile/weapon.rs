use bevy::{math::Vec2, time::Timer};
use thetawave_interface::spawnable::ProjectileType;

use crate::spawnable::SpawnPosition;

/// Weapons spawn projectiles
pub struct Weapon {
    /// Projectile type that the weapon spawns
    pub ammunition: ProjectileType,
    /// Damage of each projectile spawned by the weapon
    pub damage: f32,
    /// Position to spawn projectiles, either relative to the source or global
    pub position: SpawnPosition,
    /// Tracks time until next projectile(s) can be spawned
    pub reload_timer: Timer,
    /// Initial delay before first projeectile(s) can be spawned
    pub initial_timer: Timer,
    /// Base speed of spawned projectiles
    pub projectile_speed: f32,
    /// Angle in radians of spawned projectiles
    pub direction: f32,
    /// Time before spawned projectiles despawn
    pub despawn_time: f32,
    /// Number of projectiles spawned at once
    pub count: usize,
    /// Determines the shape of the arc using (x, y) velocity multipliers
    pub spread_weights: Vec2,
}
