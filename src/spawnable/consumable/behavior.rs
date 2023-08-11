use crate::{
    assets::GameAudioAssets,
    audio,
    collision::SortedCollisionEvent,
    game::GameParametersResource,
    spawnable::{InitialMotion, PlayerComponent, SpawnEffectEvent},
};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use thetawave_interface::spawnable::{ConsumableType, EffectType, TextEffectType};

use super::ConsumableEffect;

/// Behaviors specific to consumables
#[derive(Deserialize, Clone)]
pub enum ConsumableBehavior {
    ApplyEffectsOnImpact,
    AttractToPlayer,
}

/// Execute consumable behaviors on all consumable entities
#[allow(clippy::too_many_arguments)]
pub fn consumable_execute_behavior_system(
    mut commands: Commands,
    mut consumable_query: Query<(
        Entity,
        &Transform,
        &mut Velocity,
        &mut super::ConsumableComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent, &Transform)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
    game_parameters_res: Res<GameParametersResource>,
) {
    // put all collision events in a vector first (so that they can be looked at multiple times)
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    // iterate through all consumable entities
    for (entity, consumable_transform, mut velocity, consumable_component) in
        consumable_query.iter_mut()
    {
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
                        &audio_channel,
                        &audio_assets,
                        &game_parameters_res,
                        consumable_component.consumable_type.clone(),
                    );
                }
                ConsumableBehavior::AttractToPlayer => {
                    attract_to_player(&mut velocity, consumable_transform, &mut player_query);
                }
            }
        }
    }
}

/// Behavior that moves the consumable closer to the player
fn attract_to_player(
    velocity: &mut Velocity,
    consumable_transform: &Transform,
    player_query: &mut Query<(Entity, &mut PlayerComponent, &Transform)>,
) {
    // get position and attraction distance of closest player
    let mut closest_player_pos: Option<(Vec2, f32)> = None;

    // set the position to be attracted to, to that of the closest player
    for (_, player_component, player_transform) in player_query.iter_mut() {
        // get distance between the player and the consumable
        let distance = player_transform
            .translation
            .xy()
            .distance(consumable_transform.translation.xy());

        // if the distance is less than the player's attraction stat
        if distance < player_component.attraction_distance {
            // set it closer player
            match closest_player_pos {
                Some((pos, _)) => {
                    if distance < consumable_transform.translation.xy().distance(pos) {
                        closest_player_pos = Some((
                            player_transform.translation.xy(),
                            player_component.attraction_acceleration,
                        ));
                    }
                }
                None => {
                    closest_player_pos = Some((
                        player_transform.translation.xy(),
                        player_component.attraction_acceleration,
                    ));
                }
            }
        }
    }

    // accelerate the consumabe in the direction of the closest player
    if let Some((player_pos, accel)) = closest_player_pos {
        let direction = (player_pos - consumable_transform.translation.xy()).normalize();
        velocity.linvel.x += accel * direction.x;
        velocity.linvel.y += accel * direction.y;
    }
}

/// Apply effects to the player on collistion
#[allow(clippy::too_many_arguments)]
fn apply_effects_on_impact(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent, &Transform)>,
    consumable_effects: &Vec<ConsumableEffect>,
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    audio_channel: &AudioChannel<audio::SoundEffectsAudioChannel>,
    audio_assets: &GameAudioAssets,
    game_parameters_res: &GameParametersResource,
    consumable_type: ConsumableType,
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

                // spawn the consumable despawn effeect
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
                    initial_motion: InitialMotion::default(),
                    text: None,
                });

                spawn_effect_event_writer.send(SpawnEffectEvent {
                    effect_type: EffectType::Text(TextEffectType::ConsumableCollected(
                        consumable_type.clone(),
                    )),
                    transform: Transform {
                        translation: transform.translation,
                        scale: transform.scale,
                        ..default()
                    },
                    initial_motion: InitialMotion::default(),
                    text: None,
                });

                //apply effect to player
                for (player_entity_q, mut player_component, _) in player_query.iter_mut() {
                    if *player_entity == player_entity_q {
                        // play consumable pickup sound
                        audio_channel.play(audio_assets.consumable_pickup.clone());

                        // apply the effects to the player
                        for consumable_effect in consumable_effects {
                            match consumable_effect {
                                ConsumableEffect::GainHealth(health) => {
                                    player_component.health.heal(*health);
                                }
                                ConsumableEffect::GainArmor(armor) => {
                                    player_component.health.gain_armor(*armor);
                                }
                                ConsumableEffect::GainMoney(money) => {
                                    player_component.money += *money;
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
