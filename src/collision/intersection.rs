use crate::spawnable::{
    ConsumableComponent, MobComponent, MobSegmentComponent, ProjectileComponent,
};
use bevy::prelude::{debug, Entity, EventReader, EventWriter, Query, With};
use bevy_rapier2d::{prelude::CollisionEvent, rapier::prelude::CollisionEventFlags};
use thetawave_interface::{
    player::PlayerComponent,
    spawnable::{Faction, ItemComponent, MobSegmentType, MobType, ProjectileType},
};

use super::{CollidingEntityPair, SortedCollisionEvent};

/// Creates events from intersection (sensor) collisions
pub(super) fn intersection_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<PlayerComponent>>,
    consumable_query: Query<Entity, With<ConsumableComponent>>,
    item_query: Query<Entity, With<ItemComponent>>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
) {
    // loop through all collision events
    'collision_events: for collision_event in collision_events.read() {
        debug!("{collision_event:?}");
        if let CollisionEvent::Started(
            collider1_entity,
            collider2_entity,
            CollisionEventFlags::SENSOR,
        ) = collision_event
        {
            // 'Canonicalized' pair to deal with/normalize out E.x. (mob, player) vs (player, mob)
            // intersection symmetry of pattern match arms.
            let colliding_entities = {
                let mut entities = [*collider1_entity, *collider2_entity];
                entities.sort_by_key(|e| {
                    (
                        // (0,...,1) ascending ordering.
                        player_query.get(*e).is_ok(), // Most important. Check first
                        mob_query.get(*e).is_ok(),
                        mob_segment_query.get(*e).is_ok(),
                        projectile_query.get(*e).is_ok(),
                        item_query.get(*e).is_ok(), // least important thing to check.
                    )
                });
                CollidingEntityPair {
                    primary: entities[1],
                    secondary: entities[0],
                }
            };
            //check if player was collided with
            if player_query.get(colliding_entities.primary).is_ok() {
                // check for player-to-projectile intersection
                if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(
                        SortedCollisionEvent::PlayerToProjectileIntersection {
                            player_entity: colliding_entities.primary,
                            projectile_entity: colliding_entities.secondary,
                            projectile_faction: match projectile_component.projectile_type.clone() {
                                ProjectileType::Blast(faction) => faction,
                                ProjectileType::Bullet(faction) => faction,
                            },
                            projectile_damage: projectile_component.damage,
                        },
                    );
                    continue 'collision_events;
                }
                // check for player-consumable intersection
                else if consumable_query.get(colliding_entities.secondary).is_ok() {
                    collision_event_writer.send(
                        SortedCollisionEvent::PlayerToConsumableIntersection {
                            player_entity: colliding_entities.primary,
                            consumable_entity: colliding_entities.secondary,
                        },
                    );
                    continue 'collision_events;
                }
                // check for player-item intersection
                else if item_query.get(colliding_entities.secondary).is_ok() {
                    collision_event_writer.send(SortedCollisionEvent::PlayerToItemIntersection {
                        player_entity: colliding_entities.primary,
                        item_entity: colliding_entities.secondary,
                    });
                    continue 'collision_events;
                }
            }
            // check of mob was collided with
            else if let Ok((_mob_entity, mob_component)) =
                mob_query.get(colliding_entities.primary)
            {
                // check for mob-to-projectile intersection
                if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(
                        SortedCollisionEvent::MobToProjectileIntersection {
                            projectile_source: projectile_component.source,
                            mob_entity: colliding_entities.primary,
                            projectile_entity: colliding_entities.secondary,
                            mob_faction: match mob_component.mob_type {
                                MobType::Enemy(_) => Faction::Enemy,
                                MobType::Ally(_) => Faction::Ally,
                                MobType::Neutral(_) => Faction::Neutral,
                            },
                            projectile_faction: match projectile_component.projectile_type.clone() {
                                ProjectileType::Blast(faction) => faction,
                                ProjectileType::Bullet(faction) => faction,
                            },
                            projectile_damage: projectile_component.damage,
                        },
                    );
                    continue 'collision_events;
                }
            }
            // check if mob segment was collided with
            else if let Ok((_mob_segment_entity, mob_segment_component)) =
                mob_segment_query.get(colliding_entities.primary)
            {
                // check for mobSegment-to-projectile intersection
                if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(
                        SortedCollisionEvent::MobSegmentToProjectileIntersection {
                            mob_segment_entity: colliding_entities.primary,
                            projectile_entity: colliding_entities.secondary,
                            mob_segment_faction: match mob_segment_component.mob_segment_type {
                                MobSegmentType::Neutral(_) => Faction::Neutral,
                                MobSegmentType::Enemy(_) => Faction::Enemy,
                            },
                            projectile_faction: match projectile_component.projectile_type.clone() {
                                ProjectileType::Blast(faction) => faction,
                                ProjectileType::Bullet(faction) => faction,
                            },
                            projectile_damage: projectile_component.damage,
                        },
                    );
                    continue 'collision_events;
                }
            }
        }
    }
}
