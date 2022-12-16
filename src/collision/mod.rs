use crate::{
    arena::ArenaBarrierComponent,
    assets::GameAudioAssets,
    audio,
    player::PlayerComponent,
    spawnable::{
        ConsumableComponent, Faction, MobComponent, MobType, ProjectileComponent, ProjectileType,
    },
};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

/// Types of collisions
#[derive(Debug)]
pub enum SortedCollisionEvent {
    PlayerToProjectileIntersection {
        player_entity: Entity,
        projectile_entity: Entity,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    MobToProjectileIntersection {
        mob_entity: Entity,
        projectile_entity: Entity,
        mob_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    BossPartToProjectileIntersection {
        boss_part_entity: Entity,
        projectile_entity: Entity,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    PlayerToConsumableIntersection {
        player_entity: Entity,
        consumable_entity: Entity,
    },
    PlayerToMobContact {
        player_entity: Entity,
        mob_entity: Entity,
        mob_faction: Faction,
        player_damage: f32,
        mob_damage: f32,
    },
    MobToMobContact {
        mob_entity_1: Entity,
        mob_faction_1: Faction,
        mob_damage_1: f32,
        mob_entity_2: Entity,
        mob_faction_2: Faction,
        mob_damage_2: f32,
    },
    MobToBarrierContact {
        mob_entity: Entity,
        barrier_entity: Entity,
    },
}

/// Stores two colliding entities
#[derive(Clone, Copy, Debug)]
pub struct CollidingEntities {
    primary: Entity,
    secondary: Entity,
}

/// Creates events from intersection (sensor) collisions
pub fn intersection_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<PlayerComponent>>,
    consumable_query: Query<Entity, With<ConsumableComponent>>,
    mob_query: Query<(Entity, &MobComponent)>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
) {
    // loop through all collision events
    'collision_events: for collision_event in collision_events.iter() {
        println!("{collision_event:?}");
        if let CollisionEvent::Started(
            collider1_entity,
            collider2_entity,
            CollisionEventFlags::SENSOR,
        ) = collision_event
        {
            //check if player was collided with
            for player_entity in player_query.iter() {
                // first entity is player second, is the other colliding entity
                let colliding_entities: Option<CollidingEntities> =
                    if player_entity == *collider1_entity {
                        Some(CollidingEntities {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if player_entity == *collider2_entity {
                        Some(CollidingEntities {
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

            for (mob_entity, mob_component) in mob_query.iter() {
                // first entity is projectile, second is the other colliding entity
                let colliding_entities: Option<CollidingEntities> =
                    if mob_entity == *collider1_entity {
                        Some(CollidingEntities {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_entity == *collider2_entity {
                        Some(CollidingEntities {
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

/// Creates events from contact collisions
pub fn contact_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &PlayerComponent)>,
    mob_query: Query<(Entity, &MobComponent)>,
    barrier_query: Query<Entity, With<ArenaBarrierComponent>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    // loop through all collision events
    'collision_events: for contact_event in collision_events.iter() {
        if let CollisionEvent::Stopped(collider1_entity, collider2_entity, _) = contact_event {
            //check if player was collided with
            for (player_entity, player_component) in player_query.iter() {
                // first entity is the player, the second is the other colliding entity
                let colliding_entities: Option<CollidingEntities> =
                    if player_entity == *collider1_entity {
                        Some(CollidingEntities {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if player_entity == *collider2_entity {
                        Some(CollidingEntities {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                // check if colliding entities were found
                if let Some(colliding_entities) = colliding_entities {
                    // check if player collided with a mob
                    for (mob_entity, mob_component) in mob_query.iter() {
                        if colliding_entities.secondary == mob_entity {
                            audio_channel.play(audio_assets.collision.clone());
                            collision_event_writer.send(SortedCollisionEvent::PlayerToMobContact {
                                player_entity: colliding_entities.primary,
                                mob_entity: colliding_entities.secondary,
                                mob_faction: match mob_component.mob_type.clone() {
                                    MobType::Enemy(_) => Faction::Enemy,
                                    MobType::Ally(_) => Faction::Ally,
                                    MobType::Neutral(_) => Faction::Neutral,
                                },
                                player_damage: player_component.collision_damage,
                                mob_damage: mob_component.collision_damage,
                            });
                            continue 'collision_events;
                        }
                    }

                    // check if player collided with a barrier
                    for barrier_entity in barrier_query.iter() {
                        // check if secondary entity is a barrier
                        if colliding_entities.secondary == barrier_entity {
                            // play the barrier bounce sound
                            audio_channel.play(audio_assets.barrier_bounce.clone());
                            continue 'collision_events;
                        }
                    }
                }
            }

            // check if mob was in collision
            for (mob_entity_1, mob_component_1) in mob_query.iter() {
                // first entity is the mob, the second entity is the other colliding entity
                let colliding_entities: Option<CollidingEntities> =
                    if mob_entity_1 == *collider1_entity {
                        Some(CollidingEntities {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_entity_1 == *collider2_entity {
                        Some(CollidingEntities {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                // check if colliding entities were found
                if let Some(colliding_entities) = colliding_entities {
                    // check if mob collided with other mob
                    for (mob_entity_2, mob_component_2) in mob_query.iter() {
                        // check if secondary entity is another mob
                        if colliding_entities.secondary == mob_entity_2 {
                            // play collision sound
                            audio_channel.play(audio_assets.collision.clone());

                            // send two sorted collision events, swapping the position of the mobs in the struct
                            collision_event_writer.send(SortedCollisionEvent::MobToMobContact {
                                mob_entity_1: colliding_entities.primary,
                                mob_faction_1: match mob_component_1.mob_type {
                                    MobType::Enemy(_) => Faction::Enemy,
                                    MobType::Ally(_) => Faction::Ally,
                                    MobType::Neutral(_) => Faction::Neutral,
                                },
                                mob_damage_1: mob_component_1.collision_damage,
                                mob_entity_2: colliding_entities.secondary,
                                mob_faction_2: match mob_component_2.mob_type {
                                    MobType::Enemy(_) => Faction::Enemy,
                                    MobType::Ally(_) => Faction::Ally,
                                    MobType::Neutral(_) => Faction::Neutral,
                                },
                                mob_damage_2: mob_component_2.collision_damage,
                            });
                            collision_event_writer.send(SortedCollisionEvent::MobToMobContact {
                                mob_entity_1: colliding_entities.secondary,
                                mob_faction_1: match mob_component_2.mob_type {
                                    MobType::Enemy(_) => Faction::Enemy,
                                    MobType::Ally(_) => Faction::Ally,
                                    MobType::Neutral(_) => Faction::Neutral,
                                },
                                mob_damage_1: mob_component_2.collision_damage,
                                mob_entity_2: colliding_entities.primary,
                                mob_faction_2: match mob_component_1.mob_type {
                                    MobType::Enemy(_) => Faction::Enemy,
                                    MobType::Ally(_) => Faction::Ally,
                                    MobType::Neutral(_) => Faction::Neutral,
                                },
                                mob_damage_2: mob_component_1.collision_damage,
                            });
                            continue 'collision_events;
                        }
                    }

                    // check if mob collided with barrier
                    for barrier_entity in barrier_query.iter() {
                        // check if secondary entity is a barrier
                        if colliding_entities.secondary == barrier_entity {
                            // send a sorted collision event
                            collision_event_writer.send(
                                SortedCollisionEvent::MobToBarrierContact {
                                    mob_entity: colliding_entities.primary,
                                    barrier_entity,
                                },
                            );
                            // play the barrier bounce sound
                            audio_channel.play(audio_assets.barrier_bounce.clone());
                            continue 'collision_events;
                        }
                    }
                }
            }
        }
    }
}
