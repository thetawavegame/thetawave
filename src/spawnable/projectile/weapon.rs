use std::time::Duration;

use bevy::{
    math::Vec2,
    time::{Timer, TimerMode},
};
use thetawave_interface::spawnable::ProjectileType;

use crate::spawnable::SpawnPosition;

enum FireMode {
    Automatic,
    Manual,
}

/// Stores data about about a Weapon using minimal data
pub struct WeaponData {
    /// Projectile type that the weapon spawns
    pub ammunition: ProjectileType,
    /// Damage of each projectile spawned by the weapon
    pub damage: f32,
    /// Position to spawn projectiles, either relative to the source or global
    pub position: SpawnPosition,
    /// Time between firing projectiles
    pub reload_time: f32,
    /// Initial delay before first projectile(s) can be spawned
    pub initial_time: f32,
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
    /// Determines how the weapon handles the reload timer
    pub fire_mode: FireMode,
}

/// Describes how projectiles are spawned
pub struct Weapon {
    /// Projectile type that the weapon spawns
    pub ammunition: ProjectileType,
    /// Damage of each projectile spawned by the weapon
    pub damage: f32,
    /// Position to spawn projectiles, either relative to the source or global
    pub position: SpawnPosition,
    /// Tracks time until next projectile(s) can be spawned
    pub reload_timer: Timer,
    /// Initial delay before first projectile(s) can be spawned
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
    /// Determines how the weapon handles the reload timer
    pub fire_mode: FireMode,
}

impl From<WeaponData> for Weapon {
    fn from(value: WeaponData) -> Self {
        Weapon {
            ammunition: value.ammunition,
            damage: value.damage,
            position: value.position,
            reload_timer: Timer::from_seconds(value.reload_time, TimerMode::Once),
            initial_timer: Timer::from_seconds(value.initial_time, TimerMode::Once),
            projectile_speed: value.projectile_speed,
            direction: value.direction,
            despawn_time: value.despawn_time,
            count: value.count,
            spread_weights: value.spread_weights,
            fire_mode: value.fire_mode,
        }
    }
}

impl Weapon {
    /// Updates the weapon's timers
    /// Returns true if the weapon can be fired
    pub fn update(&mut self, delta_time: Duration) -> bool {
        // tick the initial timer if there is still time remaining
        // if the initial timer is finished then the reload timer is ticked
        if !self.initial_timer.finished() {
            self.initial_timer.tick(delta_time);
            false
        } else {
            self.reload_timer.tick(delta_time);

            // if the reload timer is finished return true and reset the timer fire_mode is Automatic
            // if ther timer is not finsished then return false
            if self.reload_timer.finished() {
                if matches!(self.fire_mode, FireMode::Automatic) {
                    self.reload_timer.reset();
                }
                return true;
            }
            false
        }
    }
}
