use std::time::Duration;

use bevy::prelude::*;
use thetawave_interface::{
    health::DamageDealtEvent,
    spawnable::{EffectType, TextEffectType},
};

use crate::spawnable::SpawnEffectEvent;

/// Handle player health regeneration
pub fn regenerate_shields_system(mut health_query: Query<&mut HealthComponent>, time: Res<Time>) {
    for mut health in health_query.iter_mut() {
        health.regenerate_shields(time.delta());
    }
}

/// Receive damage dealt events, apply damage, and spawn effects
pub fn damage_system(
    mut damage_dealt_events: EventReader<DamageDealtEvent>,
    mut health_query: Query<(Entity, &Transform, &mut HealthComponent)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    for event in damage_dealt_events.iter() {
        if let Ok((_entity, transform, mut health_component)) =
            health_query.get_mut(event.target.clone())
        {
            // take damage from health
            health_component.take_damage(event.damage);

            // spawn damage dealt text effect
            spawn_effect_event_writer.send(SpawnEffectEvent {
                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                transform: Transform {
                    translation: transform.translation,
                    scale: transform.scale,
                    ..Default::default()
                },
                text: Some(event.damage.to_string()),
                ..default()
            });
        }
    }
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
            //let damage_piercing_shields = (damage - self.shields).clamp(0, usize::MAX);
            let damage_piercing_shields = damage.checked_sub(self.shields).unwrap_or(0);
            //self.shields = (self.shields - damage).clamp(0, self.max_shields);
            self.shields = self.shields.checked_sub(damage).unwrap_or(0);
            /*
            if damage_piercing_shields.is_sign_positive() {
                self.health = (self.health - damage_piercing_shields).clamp(0, self.max_health);
            }
            */
            if damage_piercing_shields > 0 {
                self.health = self
                    .health
                    .checked_sub(damage_piercing_shields)
                    .unwrap_or(0);
            }
        } else {
            self.armor -= 1;
        }
    }

    /// Get maximum health
    pub fn get_max_health(&self) -> usize {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> usize {
        self.health
    }

    /// Get maximum health
    pub fn get_max_shields(&self) -> usize {
        self.max_shields
    }

    /// Get current health
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
        self.health as f32 / self.max_health as f32
    }

    /// Percentage of defense left
    pub fn get_shields_percentage(&self) -> f32 {
        self.shields as f32 / self.max_shields as f32
    }
}
