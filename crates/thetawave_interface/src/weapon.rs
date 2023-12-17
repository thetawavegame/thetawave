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
    /// Base reload time of the weapon
    pub base_reload_time: f32,
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
            base_reload_time: value.reload_time,
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

            // fire the weapon and return the projectile data if automatic
            // othewise return none
            match self.fire_mode {
                FireMode::Automatic => self.fire_weapon(),
                FireMode::Manual => None,
            }
        }
    }

    /// Returns ture if the weapon can be fired
    pub fn can_fire(&self) -> bool {
        self.reload_timer.finished()
    }

    /// Returs the projectiles data if the weapon can be fired and resets the reload timer
    pub fn fire_weapon(&mut self) -> Option<WeaponProjectileData> {
        if self.can_fire() {
            self.reload_timer.reset();
            Some(self.projectile_data.clone())
        } else {
            None
        }
    }

    /// Gain projectiles, but limit to the capacity of the weapon
    pub fn gain_projectiles(&mut self, projectiles: usize) {
        self.projectile_data.count = (self.projectile_data.count + projectiles).min(self.capacity);
    }

    /// Set reload time to a new duration
    pub fn set_reload_time(&mut self, new_reload_time: f32) {
        self.reload_timer
            .set_duration(Duration::from_secs_f32(new_reload_time));
    }

    /// Set the fire rate of the weapon based on the amount of money colleted
    pub fn set_reload_time_from_money(&mut self, money: usize) {
        let diminishing_factor = 0.08;
        let min_reload_time = 0.1;
        let adjusted_money = (1.0 + money as f32).ln();
        self.set_reload_time(
            (self.base_reload_time - (adjusted_money * diminishing_factor)).max(min_reload_time),
        );
    }
}
