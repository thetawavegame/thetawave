use std::time::Duration;

use bevy_ecs::prelude::{Component, Entity};
use bevy_ecs_macros::Event;
use bevy_time::{Timer, TimerMode};

#[derive(Event)]
pub struct DamageDealtEvent {
    pub damage: usize,
    pub target: Entity,
}

/// Tracks health for an entity
#[derive(Component, Default)]
pub struct HealthComponent {
    /// Current health value
    health: usize,
    /// Maxumum health value
    max_health: usize,
    /// Amount of armor, which absorbs full damage from a single hit
    armor: usize,
    /// Current shields value
    shields: usize,
    /// Maximum shields value
    max_shields: usize,
    /// Time it takes to regenerate one unit of shields
    shields_recharge_timer: Timer,
}

impl HealthComponent {
    /// Create a new health struct from a maximum health value
    pub fn new(health: usize, shields: usize, shields_recharge_rate: f32) -> Self {
        HealthComponent {
            max_health: health,
            health,
            max_shields: shields,
            shields,
            armor: 0,
            //shields_recharge_rate,
            shields_recharge_timer: Timer::from_seconds(
                shields_recharge_rate,
                TimerMode::Repeating,
            ),
        }
    }

    pub fn regenerate_shields(&mut self, delta_time: Duration) {
        self.shields_recharge_timer.tick(delta_time);
        if self.shields_recharge_timer.just_finished() && self.shields < self.max_shields {
            self.shields += 1
        }
    }

    /// Check if health is below zero
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    /// Take damage (deplete armor, then shields, then health  in that order)
    pub fn take_damage(&mut self, damage: usize) {
        if self.armor == 0 {
            let damage_piercing_shields = damage.saturating_sub(self.shields);
            self.shields = self.shields.saturating_sub(damage);

            if damage_piercing_shields > 0 {
                self.health = self.health.saturating_sub(damage_piercing_shields);
            }
        } else {
            self.armor -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn get_max_health(&self) -> usize {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> usize {
        self.health
    }

    #[allow(dead_code)]
    pub fn get_max_shields(&self) -> usize {
        self.max_shields
    }

    /// Get current health
    #[allow(dead_code)]
    pub fn get_shields(&self) -> usize {
        self.shields
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
    }

    /// Add to health
    pub fn heal(&mut self, health: usize) {
        self.health = (self.health + health).min(self.max_health);
    }

    /// Add to armor
    pub fn gain_armor(&mut self, armor: usize) {
        self.armor += armor;
    }

    /// Percentage of defense left
    pub fn get_health_percentage(&self) -> f32 {
        if self.max_health > 0 {
            self.health as f32 / self.max_health as f32
        } else {
            0.0
        }
    }

    /// Percentage of defense left
    pub fn get_shields_percentage(&self) -> f32 {
        if self.max_shields > 0 {
            self.shields as f32 / self.max_shields as f32
        } else {
            0.0
        }
    }

    pub fn increase_max_health(&mut self, value: usize) {
        self.max_health += value;
    }
}
