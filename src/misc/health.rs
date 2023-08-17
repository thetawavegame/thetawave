use bevy::prelude::*;
use thetawave_interface::{
    health::DamageDealtEvent,
    spawnable::{EffectType, TextEffectType},
};

use crate::{
    player::Character,
    spawnable::{MobData, MobSegmentData, SpawnEffectEvent},
};

/// Handle player health regeneration
pub fn regenerate_shields_system(mut health_query: Query<&mut HealthComponent>, time: Res<Time>) {
    for mut health in health_query.iter_mut() {
        health.regenerate_shields(time.delta_seconds());
    }
}

/// Receive damage dealt events, apply damage, and spawn effects
pub fn damage_system(
    mut damage_dealt_event: EventReader<DamageDealtEvent>,
    mut health_query: Query<(Entity, &Transform, &mut HealthComponent)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    'events: for event in damage_dealt_event.iter() {
        for (entity, transform, mut health_component) in health_query.iter_mut() {
            if event.target == entity {
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

                continue 'events;
            }
        }
    }
}

/// Tracks health for an entity
#[derive(Component, Default)]
pub struct HealthComponent {
    max_health: f32,
    health: f32,
    armor: usize,
    shields: f32,
    max_shields: f32,
    shields_recharge_rate: f32,
}

impl From<&Character> for HealthComponent {
    fn from(character: &Character) -> Self {
        HealthComponent::new(
            character.health,
            character.shields,
            character.shields_recharge_rate,
        )
    }
}

impl From<&MobData> for HealthComponent {
    fn from(mob_data: &MobData) -> Self {
        HealthComponent::new(mob_data.health, 0.0, 0.0)
    }
}

impl From<&MobSegmentData> for HealthComponent {
    fn from(mob_segment_data: &MobSegmentData) -> Self {
        HealthComponent::new(mob_segment_data.health, 0.0, 0.0)
    }
}

impl HealthComponent {
    /// Create a new health struct from a maximum health value
    pub fn new(health: f32, shields: f32, shields_recharge_rate: f32) -> Self {
        HealthComponent {
            max_health: health,
            health,
            max_shields: shields,
            shields,
            armor: 0,
            shields_recharge_rate,
        }
    }

    pub fn regenerate_shields(&mut self, delta_time: f32) {
        self.shields += self.shields_recharge_rate * delta_time;
        self.shields = self.shields.clamp(0.0, self.max_shields);
    }

    /// Check if health is below zero
    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }

    /// Take damage (deplete armor, then shields, then health  in that order)
    pub fn take_damage(&mut self, damage: f32) {
        if self.armor == 0 {
            let damage_piercing_shields = (damage - self.shields).clamp(0.0, f32::MAX);
            self.shields = (self.shields - damage).clamp(0.0, self.max_shields);
            if damage_piercing_shields.is_sign_positive() {
                self.health = (self.health - damage_piercing_shields).clamp(0.0, self.max_health);
            }
        } else {
            self.armor -= 1;
        }
    }

    /// Get maximum health
    pub fn get_max_health(&self) -> f32 {
        self.max_health
    }

    /// Get current health
    pub fn get_health(&self) -> f32 {
        self.health
    }

    /// Get maximum health
    pub fn get_max_shields(&self) -> f32 {
        self.max_shields
    }

    /// Get current health
    pub fn get_shields(&self) -> f32 {
        self.shields
    }

    /// Get available armor count
    pub fn get_armor(&self) -> usize {
        self.armor
    }

    /// Add to health
    pub fn heal(&mut self, health: f32) {
        if health < 0.0 {
            panic!("Attempted to heal by negative value. Use take_damage function instead?");
        }
        self.health = (self.health + health).clamp(0.0, self.max_health);
    }

    /// Add to armor
    pub fn gain_armor(&mut self, armor: usize) {
        self.armor += armor;
    }
}
