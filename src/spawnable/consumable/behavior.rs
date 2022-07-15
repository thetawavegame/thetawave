use crate::{
    collision::SortedCollisionEvent,
    run::{ObjectiveType, RunResource},
    spawnable::{EffectType, Faction, PlayerComponent, SpawnEffectEvent, SpawnableComponent},
};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use serde::Deserialize;

use super::ConsumableEffect;

#[derive(Deserialize, Clone)]
pub enum ConsumableBehavior {
    ApplyEffectsOnImpact,
}

pub fn consumable_execute_behavior_system(
    mut consumable_query: Query<(
        Entity,
        &Transform,
        &mut SpawnableComponent,
        &mut super::ConsumableComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut run_resource: ResMut<RunResource>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (entity, consumable_transform, mut spawnable_component, consumable_component) in
        consumable_query.iter_mut()
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
            }
        }
    }
}

fn apply_effects_on_impact(
    entity: Entity,
    transform: &Transform,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
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
                for (player_entity_q, mut player_component) in player_query.iter_mut() {
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
