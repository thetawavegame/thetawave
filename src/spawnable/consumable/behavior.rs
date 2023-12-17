use crate::{
    collision::SortedCollisionEvent, game::GameParametersResource, spawnable::SpawnEffectEvent,
};
use bevy::prelude::*;
use serde::Deserialize;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    health::HealthComponent,
    player::PlayerComponent,
    spawnable::{ConsumableType, EffectType, TextEffectType},
    weapon::{self, WeaponComponent},
};

use super::ConsumableEffect;

/// Behaviors specific to consumables
#[derive(Deserialize, Clone)]
pub enum ConsumableBehavior {
    ApplyEffectsOnImpact,
}

/// Execute consumable behaviors on all consumable entities
#[allow(clippy::too_many_arguments)]
pub fn consumable_execute_behavior_system(
    mut commands: Commands,
    mut consumable_query: Query<(Entity, &Transform, &mut super::ConsumableComponent)>,
    mut player_query: Query<(
        Entity,
        &mut PlayerComponent,
        &Transform,
        &mut HealthComponent,
        &mut WeaponComponent,
    )>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    game_parameters_res: Res<GameParametersResource>,
) {
    // put all collision events in a vector first (so that they can be looked at multiple times)
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.read() {
        collision_events_vec.push(collision_event);
    }

    // iterate through all consumable entities
    for (entity, consumable_transform, consumable_component) in consumable_query.iter_mut() {
        // perform each behavior
        for behavior in &consumable_component.behaviors {
            match behavior {
                ConsumableBehavior::ApplyEffectsOnImpact => {
                    apply_effects_on_impact(
                        &mut commands,
                        entity,
                        consumable_transform,
                        &collision_events_vec,
                        &mut player_query,
                        &consumable_component.consumable_effects,
                        &mut spawn_effect_event_writer,
                        &game_parameters_res,
                        consumable_component.consumable_type.clone(),
                        &mut sound_effect_event_writer,
                    );
                }
            }
        }
    }
}

/// Apply effects to the player on collistion
#[allow(clippy::too_many_arguments)]
fn apply_effects_on_impact(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(
        Entity,
        &mut PlayerComponent,
        &Transform,
        &mut HealthComponent,
        &mut WeaponComponent,
    )>,
    consumable_effects: &Vec<ConsumableEffect>,
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    game_parameters_res: &GameParametersResource,
    consumable_type: ConsumableType,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToConsumableIntersection {
            player_entity,
            consumable_entity,
        } = collision_event
        {
            if entity == *consumable_entity {
                // despawn consumable
                commands.entity(entity).despawn_recursive();

                // spawn the consumable despawn effect
                spawn_effect_event_writer.send(SpawnEffectEvent {
                    effect_type: EffectType::ConsumableDespawn,
                    transform: Transform {
                        translation: transform.translation,
                        scale: Vec3::new(
                            game_parameters_res.sprite_scale,
                            game_parameters_res.sprite_scale,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..default()
                });

                // spawn consumable collected text
                spawn_effect_event_writer.send(SpawnEffectEvent {
                    effect_type: EffectType::Text(TextEffectType::ConsumableCollected(
                        consumable_type.clone(),
                    )),
                    transform: Transform {
                        translation: transform.translation,
                        scale: transform.scale,
                        ..default()
                    },
                    ..default()
                });

                //apply effect to player
                for (
                    player_entity_q,
                    mut player_component,
                    _,
                    mut health_component,
                    mut weapon_component,
                ) in player_query.iter_mut()
                {
                    if *player_entity == player_entity_q {
                        // play consumable pickup sound
                        sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::ConsumablePickup,
                        });

                        // apply the effects to the player
                        for consumable_effect in consumable_effects {
                            match consumable_effect {
                                ConsumableEffect::GainHealth(health) => {
                                    health_component.heal(*health);
                                }
                                ConsumableEffect::GainArmor(armor) => {
                                    health_component.gain_armor(*armor);
                                }
                                ConsumableEffect::GainMoney(money) => {
                                    player_component.money += *money;
                                }
                                ConsumableEffect::GainProjectiles(projectile) => {
                                    weapon_component.gain_projectiles(*projectile)
                                }
                            }
                        }
                    }
                }
                continue;
            }
        }
    }
}
