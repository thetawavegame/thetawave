use bevy_ecs::component::Component;
use bevy_math::Vec2;
use bevy_time::{Timer, TimerMode};
use serde::Deserialize;

use crate::spawnable::{ProjectileType, SpawnPosition};

use std::time::Duration;

#[derive(Deserialize, Clone)]
pub enum FireMode {
    Automatic,
    Manual,
}

/// Stores data about about a Weapon using minimal data
#[derive(Deserialize, Clone)]
pub struct WeaponData {
    /// Time between firing projectiles
    pub reload_time: f32,
    /// Initial delay before first projectile(s) can be spawned
    pub initial_time: f32,
    /// Determines how the weapon handles the reload timer
    pub fire_mode: FireMode,
    /// Maximum number of projectiles the weapon can have
    pub capacity: usize,
    /// Data about the projectiles fired from the weapon
    pub projectile_data: WeaponProjectileData,
}

#[derive(Deserialize, Clone)]
pub struct WeaponProjectileData {
    /// Projectile type that the weapon spawns
    pub ammunition: ProjectileType,
    /// Damage of each projectile spawned by the weapon
    pub damage: usize,
    /// Position to spawn projectiles, either relative to the source or global
    pub position: SpawnPosition,
    /// Base speed of spawned projectiles
    pub speed: f32,
    /// Angle in radians of spawned projectiles
    pub direction: f32,
    /// Time before spawned projectiles despawn
    pub despawn_time: f32,
    /// Number of projectiles spawned at once
    pub count: usize,
    /// Determines the shape of the arc using (x, y) velocity multipliers
    pub spread_weights: Vec2,
    /// Maximum spead angle of fired projectiles
    pub max_spread_arc: f32,
    /// Target gap between fired projectiles
    pub projectile_gap: f32,
}

impl WeaponProjectileData {
    pub fn get_spread_angle_segment(&self, max_projectiles: f32) -> f32 {
        let total_projectiles_percent = (self.count as f32 - 1.) / (max_projectiles - 1.);
        // indicates the angle between the first and last projectile
        let spread_arc = self
            .max_spread_arc
            .min(total_projectiles_percent * self.projectile_gap);
        // indicates the angle between each projectile
        spread_arc / (self.count as f32 - 1.).max(1.)
    }
}

/// Describes how projectiles are spawned
#[derive(Component, Clone)]
pub struct WeaponComponent {
    /// Tracks time until next projectile(s) can be spawned
    pub reload_timer: Timer,
    /// Initial delay before first projectile(s) can be spawned
    pub initial_timer: Timer,
    /// Determines how the weapon handles the reload timer
    pub fire_mode: FireMode,
    /// Maximum number of projectiles the weapon can have
    pub capacity: usize,
    /// Data about the projectiles fired from the weapon
    pub projectile_data: WeaponProjectileData,
}

impl From<WeaponData> for WeaponComponent {
    fn from(value: WeaponData) -> Self {
        WeaponComponent {
            reload_timer: Timer::from_seconds(value.reload_time, TimerMode::Once),
            initial_timer: Timer::from_seconds(value.initial_time, TimerMode::Once),
            fire_mode: value.fire_mode,
            capacity: value.capacity,
            projectile_data: value.projectile_data,
        }
    }
}

impl WeaponComponent {
    /// Updates the weapon's timers
    /// Returns true if the weapon can be fired
    pub fn update(&mut self, delta_time: Duration) -> Option<WeaponProjectileData> {
        // tick the initial timer if there is still time remaining
        // if the initial timer is finished then the reload timer is ticked
        if !self.initial_timer.finished() {
            self.initial_timer.tick(delta_time);
            None
        } else {
            self.reload_timer.tick(delta_time);

            // if the reload timer is finished return true and reset the timer fire_mode is Automatic
            // if ther timer is not finsished then return false

            match self.fire_mode {
                FireMode::Automatic => self.fire_weapon(),
                FireMode::Manual => None,
            }
        }
    }

    pub fn can_fire(&self) -> bool {
        self.reload_timer.finished()
    }

    pub fn fire_weapon(&mut self) -> Option<WeaponProjectileData> {
        if self.can_fire() {
            self.reload_timer.reset();
            Some(self.projectile_data.clone())
        } else {
            None
        }
    }
}
