use crate::{
    collision::SortedCollisionEvent,
    run::{ObjectiveType, RunResource},
    spawnable::{EffectType, Faction, PlayerComponent, SpawnEffectEvent, SpawnableComponent},
};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

use super::ConsumableEffect;

#[derive(Deserialize, Clone)]
pub enum ConsumableBehavior {
    ApplyEffectsOnImpact,
    AttractToPlayer,
}

pub fn consumable_execute_behavior_system(
    mut consumable_query: Query<(
        Entity,
        &Transform,
        &mut Velocity,
        &mut SpawnableComponent,
        &mut super::ConsumableComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent, &Transform)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut run_resource: ResMut<RunResource>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (
        entity,
        consumable_transform,
        mut velocity,
        mut spawnable_component,
        consumable_component,
    ) in consumable_query.iter_mut()
    {
        for behavior in &consumable_component.behaviors {
            match behavior {
                ConsumableBehavior::ApplyEffectsOnImpact => {
                    apply_effects_on_impact(
                        entity,
                        consumable_transform,
                        &mut spawnable_component,
                        &collision_events_vec,
                        &mut player_query,
                        &consumable_component.consumable_effects,
                        &mut run_resource,
                        &mut spawn_effect_event_writer,
                    );
                }
                ConsumableBehavior::AttractToPlayer => {
                    attract_to_player(&mut velocity, consumable_transform, &mut player_query);
                }
            }
        }
    }
}

fn attract_to_player(
    velocity: &mut Velocity,
    consumable_transform: &Transform,
    player_query: &mut Query<(Entity, &mut PlayerComponent, &Transform)>,
) {
    // get position and attraction distance of closest player
    let mut closest_player_pos: Option<(Vec2, f32)> = None;

    for (_, mut player_component, player_transform) in player_query.iter_mut() {
        let distance = player_transform
            .translation
            .xy()
            .distance(consumable_transform.translation.xy());

        if distance < player_component.attraction_distance {
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

    if let Some((player_pos, accel)) = closest_player_pos {
        let direction = (player_pos - consumable_transform.translation.xy()).normalize();
        velocity.linvel.x += accel * direction.x;
        velocity.linvel.y += accel * direction.y;
    }
}

fn apply_effects_on_impact(
    entity: Entity,
    transform: &Transform,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent, &Transform)>,
    consumable_effects: &Vec<ConsumableEffect>,
    run_resource: &mut RunResource,
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToConsumableIntersection {
            player_entity,
            consumable_entity,
        } = collision_event
        {
            if entity == *consumable_entity {
                // despawn consumable
                spawnable_component.should_despawn = true;

                // TODO: spawn effect
                spawn_effect_event_writer.send(SpawnEffectEvent {
                    effect_type: EffectType::ConsumableDespawn,
                    position: transform.translation.xy(),
                });

                //apply effect to player
                for (player_entity_q, mut player_component, _) in player_query.iter_mut() {
                    if *player_entity == player_entity_q {
                        for consumable_effect in consumable_effects {
                            match consumable_effect {
                                ConsumableEffect::GainHealth(health) => {
                                    player_component.health.heal(*health);
                                }
                                ConsumableEffect::GainDefense(defense) => {
                                    if let ObjectiveType::Defense(health) =
                                        &mut run_resource.levels[run_resource.level_idx].objective
                                    {
                                        health.heal(*defense);
                                    }
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
