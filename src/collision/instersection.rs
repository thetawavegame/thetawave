use crate::{
    player::PlayerComponent,
    spawnable::{ConsumableComponent, MobComponent, MobSegmentComponent, ProjectileComponent},
};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use thetawave_interface::spawnable::{Faction, MobSegmentType, MobType, ProjectileType};

use super::{CollidingEntityPair, SortedCollisionEvent};

/// Creates events from intersection (sensor) collisions
pub fn intersection_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<PlayerComponent>>,
    consumable_query: Query<Entity, With<ConsumableComponent>>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
) {
    // loop through all collision events
    'collision_events: for collision_event in collision_events.iter() {
        debug!("{collision_event:?}");
        if let CollisionEvent::Started(
            collider1_entity,
            collider2_entity,
            CollisionEventFlags::SENSOR,
        ) = collision_event
        {
            //check if player was collided with
            for player_entity in player_query.iter() {
                // first entity is player second, is the other colliding entity
                let colliding_entities: Option<CollidingEntityPair> =
                    if player_entity == *collider1_entity {
                        Some(CollidingEntityPair {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if player_entity == *collider2_entity {
                        Some(CollidingEntityPair {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                if let Some(colliding_entities) = colliding_entities {
                    // check for projectile
                    for (projectile_entity, projectile_component) in projectile_query.iter() {
                        if colliding_entities.secondary == projectile_entity {
                            collision_event_writer.send(
                                SortedCollisionEvent::PlayerToProjectileIntersection {
                                    player_entity: colliding_entities.primary,
                                    projectile_entity: colliding_entities.secondary,
                                    projectile_faction: match projectile_component
                                        .projectile_type
                                        .clone()
                                    {
                                        ProjectileType::Blast(faction) => faction,
                                        ProjectileType::Bullet(faction) => faction,
                                    },
                                    projectile_damage: projectile_component.damage,
                                },
                            );
                            continue 'collision_events;
                        }
                    }
                    // check for consumable
                    for consumable_entity in consumable_query.iter() {
                        if colliding_entities.secondary == consumable_entity {
                            collision_event_writer.send(
                                SortedCollisionEvent::PlayerToConsumableIntersection {
                                    player_entity: colliding_entities.primary,
                                    consumable_entity: colliding_entities.secondary,
                                },
                            );
                            continue 'collision_events;
                        }
                    }
                }
            }

            // check of mob was collided with
            for (mob_entity, mob_component) in mob_query.iter() {
                // first entity is mob, second is the other colliding entity
                let colliding_entities: Option<CollidingEntityPair> =
                    if mob_entity == *collider1_entity {
                        Some(CollidingEntityPair {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_entity == *collider2_entity {
                        Some(CollidingEntityPair {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                if let Some(colliding_entities) = colliding_entities {
                    // check for projectile
                    for (projectile_entity, projectile_component) in projectile_query.iter() {
                        if colliding_entities.secondary == projectile_entity {
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
                                    projectile_faction: match projectile_component
                                        .projectile_type
                                        .clone()
                                    {
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

            // check if mob segment was collided with
            for (mob_segment_entity, mob_segment_component) in mob_segment_query.iter() {
                // first entity is mob_segment, second is the other colliding entity
                let colliding_entities: Option<CollidingEntityPair> =
                    if mob_segment_entity == *collider1_entity {
                        Some(CollidingEntityPair {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_segment_entity == *collider2_entity {
                        Some(CollidingEntityPair {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                if let Some(colliding_entities) = colliding_entities {
                    // check for projectile
                    for (projectile_entity, projectile_component) in projectile_query.iter() {
                        if colliding_entities.secondary == projectile_entity {
                            collision_event_writer.send(
                                SortedCollisionEvent::MobSegmentToProjectileIntersection {
                                    mob_segment_entity: colliding_entities.primary,
                                    projectile_entity: colliding_entities.secondary,
                                    mob_segment_faction: match mob_segment_component
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    projectile_faction: match projectile_component
                                        .projectile_type
                                        .clone()
                                    {
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
    }
}
