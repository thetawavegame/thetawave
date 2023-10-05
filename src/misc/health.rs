use bevy::prelude::*;
use thetawave_interface::{
    health::{DamageDealtEvent, HealthComponent},
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
                ..default()
            });
        }
    }
}
