use crate::{
    arena::ArenaBarrierComponent,
    assets::GameAudioAssets,
    audio,
    player::PlayerComponent,
    spawnable::{
        Faction, MobComponent, MobSegmentComponent, MobSegmentType, MobType, ProjectileComponent,
        ProjectileType,
    },
};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use super::{CollidingEntityPair, SortedCollisionEvent};

/// Creates events from contact collisions
pub fn contact_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &PlayerComponent)>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    barrier_query: Query<Entity, With<ArenaBarrierComponent>>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    // loop through all collision events
    'collision_events: for contact_event in collision_events.iter() {
        if let CollisionEvent::Stopped(collider1_entity, collider2_entity, _) = contact_event {
            //check if player was collided with
            for (player_entity, player_component) in player_query.iter() {
                // first entity is the player, the second is the other colliding entity
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

                    // check if player collided with segment
                    for (mob_segment_entity, mob_segment_component) in mob_segment_query.iter() {
                        if colliding_entities.secondary == mob_segment_entity {
                            audio_channel.play(audio_assets.collision.clone());
                            collision_event_writer.send(
                                SortedCollisionEvent::PlayerToMobSegmentContact {
                                    player_entity: colliding_entities.primary,
                                    mob_segment_entity: colliding_entities.secondary,
                                    mob_segment_faction: match mob_segment_component
                                        .mob_segment_type
                                        .clone()
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    player_damage: player_component.collision_damage,
                                    mob_segment_damage: mob_segment_component.collision_damage,
                                },
                            );
                            continue 'collision_events;
                        }
                    }

                    // check if player collided with a projectile
                    for (projectile_entity, projectile_component) in projectile_query.iter() {
                        if colliding_entities.secondary == projectile_entity {
                            audio_channel.play(audio_assets.bullet_ding.clone());
                            collision_event_writer.send(
                                SortedCollisionEvent::PlayerToProjectileContact {
                                    player_entity: colliding_entities.primary,
                                    projectile_entity: colliding_entities.secondary,
                                    projectile_faction: match player_component
                                        .projectile_type
                                        .clone()
                                    {
                                        ProjectileType::Blast(faction) => faction,
                                        ProjectileType::Bullet(faction) => faction,
                                    },
                                    player_damage: player_component.collision_damage,
                                    projectile_damage: projectile_component.damage,
                                },
                            );
                            continue 'collision_events;
                        }
                    }
                }
            }

            // check if mob was in collision
            for (mob_entity_1, mob_component_1) in mob_query.iter() {
                // first entity is the mob, the second entity is the other colliding entity
                let colliding_entities: Option<CollidingEntityPair> =
                    if mob_entity_1 == *collider1_entity {
                        Some(CollidingEntityPair {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_entity_1 == *collider2_entity {
                        Some(CollidingEntityPair {
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
                    // check if mob collided with mob segment
                    for (mob_segment_entity, mob_segment_component) in mob_segment_query.iter() {
                        // check if secondary entity is a mob segment
                        if colliding_entities.secondary == mob_segment_entity {
                            // send  a sorted collision event
                            audio_channel.play(audio_assets.collision.clone());
                            collision_event_writer.send(
                                SortedCollisionEvent::MobToMobSegmentContact {
                                    mob_entity: colliding_entities.primary,
                                    mob_faction: match mob_component_1.mob_type {
                                        MobType::Enemy(_) => Faction::Enemy,
                                        MobType::Ally(_) => Faction::Ally,
                                        MobType::Neutral(_) => Faction::Neutral,
                                    },
                                    mob_damage: mob_component_1.collision_damage,
                                    mob_segment_entity: colliding_entities.secondary,
                                    mob_segment_faction: match mob_segment_component
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    mob_segment_damage: mob_segment_component.collision_damage,
                                },
                            );
                            continue 'collision_events;
                        }
                    }
                }
            }

            // check if mob segment was in collision
            for (mob_segment_entity_1, mob_segment_component_1) in mob_segment_query.iter() {
                // first entity is the mob segment, the second entity is the other colliding entity
                let colliding_entities: Option<CollidingEntityPair> =
                    if mob_segment_entity_1 == *collider1_entity {
                        Some(CollidingEntityPair {
                            primary: *collider1_entity,
                            secondary: *collider2_entity,
                        })
                    } else if mob_segment_entity_1 == *collider2_entity {
                        Some(CollidingEntityPair {
                            primary: *collider2_entity,
                            secondary: *collider1_entity,
                        })
                    } else {
                        None
                    };

                // check if colliding entities were found
                if let Some(colliding_entities) = colliding_entities {
                    // check if mob segment collided with other mob segment
                    for (mob_segment_entity_2, mob_segment_component_2) in mob_segment_query.iter()
                    {
                        if colliding_entities.secondary == mob_segment_entity_2 {
                            // play collision sound
                            audio_channel.play(audio_assets.collision.clone());

                            // send two sorted collision events, swapping the position of the mob segments in the struct
                            collision_event_writer.send(
                                SortedCollisionEvent::MobSegmentToMobSegmentContact {
                                    mob_segment_entity_1: colliding_entities.primary,
                                    mob_segment_faction_1: match mob_segment_component_1
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    mob_segment_damage_1: mob_segment_component_1.collision_damage,
                                    mob_segment_entity_2: colliding_entities.secondary,
                                    mob_segment_faction_2: match mob_segment_component_2
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    mob_segment_damage_2: mob_segment_component_2.collision_damage,
                                },
                            );
                            collision_event_writer.send(
                                SortedCollisionEvent::MobSegmentToMobSegmentContact {
                                    mob_segment_entity_1: colliding_entities.secondary,
                                    mob_segment_faction_1: match mob_segment_component_2
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    mob_segment_damage_1: mob_segment_component_2.collision_damage,
                                    mob_segment_entity_2: colliding_entities.primary,
                                    mob_segment_faction_2: match mob_segment_component_1
                                        .mob_segment_type
                                    {
                                        MobSegmentType::Neutral(_) => Faction::Neutral,
                                        MobSegmentType::Enemy(_) => Faction::Enemy,
                                    },
                                    mob_segment_damage_2: mob_segment_component_1.collision_damage,
                                },
                            );
                            continue 'collision_events;
                        }
                    }

                    // check if mob collided with barrier
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
        }
    }
}
