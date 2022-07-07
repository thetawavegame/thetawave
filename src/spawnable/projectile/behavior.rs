use crate::{
    collision::SortedCollisionEvent,
    spawnable::{
        EffectType, Faction, MobComponent, PlayerComponent, SpawnEffectEvent, SpawnableComponent,
    },
};
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
//use bevy_rapier2d::prelude::{RigidBodyPosition, RigidBodyPositionComponent};
use serde::Deserialize;

/// Types of behaviors that can be performed by projectiles
#[derive(Deserialize, Clone)]
pub enum ProjectileBehavior {
    ExplodeOnImpact,
    TimedDespawn {
        despawn_time: f32,
        current_time: f32,
    },
}

/// Manages executing behaviors of mobs
pub fn projectile_execute_behavior_system(
    mut projectile_query: Query<(
        Entity,
        &Transform,
        &mut SpawnableComponent,
        &mut super::ProjectileComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut mob_query: Query<(Entity, &mut MobComponent)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    time: Res<Time>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (entity, projectile_transform, mut spawnable_component, mut projectile_component) in
        projectile_query.iter_mut()
    {
        //let behaviors = projectile_component.behaviors.clone();
        for behavior in &mut projectile_component.behaviors {
            match behavior {
                ProjectileBehavior::ExplodeOnImpact => explode_on_impact(
                    entity,
                    projectile_transform,
                    &mut spawnable_component,
                    &collision_events_vec,
                    &mut spawn_effect_event_writer,
                    &mut player_query,
                    &mut mob_query,
                ),
                ProjectileBehavior::TimedDespawn {
                    despawn_time,
                    current_time,
                } => {
                    *current_time += time.delta_seconds();
                    if current_time > despawn_time {
                        spawnable_component.should_despawn = true;
                        // TODO: spawn fizzle out animation

                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::AllyBlastDespawn,
                            position: Vec2::new(
                                projectile_transform.translation.x,
                                projectile_transform.translation.y,
                            ),
                        });
                    }
                }
            }
        }
    }
}

/// Explode projectile on impact
fn explode_on_impact(
    entity: Entity,
    transform: &Transform,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&SortedCollisionEvent],
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    mob_query: &mut Query<(Entity, &mut MobComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileIntersection {
                player_entity,
                projectile_entity,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && matches!(
                        projectile_faction.clone(),
                        Faction::Neutral | Faction::Enemy
                    )
                {
                    // despawn blast
                    spawnable_component.should_despawn = true;
                    // spawn explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::AllyBlastExplosion,
                        position: transform.translation.xy(),
                    });
                    // deal damage to player
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if *player_entity == player_entity_q {
                            player_component.health.take_damage(*projectile_damage);
                        }
                    }
                    continue;
                }
            }

            SortedCollisionEvent::MobToProjectileIntersection {
                mob_entity,
                projectile_entity,
                mob_faction,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    // despawn blast
                    spawnable_component.should_despawn = true;
                    // spawn explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::AllyBlastExplosion,
                        position: transform.translation.xy(),
                    });
                    // deal damage to mob
                    for (mob_entity_q, mut mob_component) in mob_query.iter_mut() {
                        if *mob_entity == mob_entity_q {
                            mob_component.health.take_damage(*projectile_damage);
                        }
                    }
                    continue;
                }
            }
            _ => {}
        }
    }
}
