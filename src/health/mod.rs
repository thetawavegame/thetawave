//! Exposes a plugin that changes a player/mob's health and shields based on time and events
use crate::spawnable::SpawnEffectEvent;
use bevy::prelude::{
    App, Entity, EventReader, EventWriter, Plugin, Query, Res, Time, Transform, Update,
};
use thetawave_interface::{
    health::{DamageDealtEvent, HealthComponent},
    spawnable::{EffectType, TextEffectType},
};
/// Includes systems to decrease a player's health and regenerate their shields over time.
pub(super) struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>()
            .add_systems(Update, (damage_system, regenerate_shields_system));
    }
}

/// Handle player health regeneration
fn regenerate_shields_system(mut health_query: Query<&mut HealthComponent>, time: Res<Time>) {
    for mut health in health_query.iter_mut() {
        health.regenerate_shields(time.delta());
    }
}

/// Receive damage dealt events, apply damage, and spawn effects
fn damage_system(
    mut damage_dealt_events: EventReader<DamageDealtEvent>,
    mut health_query: Query<(Entity, &Transform, &mut HealthComponent)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    for event in damage_dealt_events.read() {
        if let Ok((_entity, transform, mut health_component)) = health_query.get_mut(event.target) {
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
                ..Default::default()
            });
        }
    }
}
