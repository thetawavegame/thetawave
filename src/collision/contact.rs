use crate::{
    arena::ArenaBarrierComponent,
    spawnable::{MobComponent, MobSegmentComponent, ProjectileComponent},
};

use bevy::prelude::{Entity, EventReader, EventWriter, Query, With};
use bevy_rapier2d::prelude::CollisionEvent;
use thetawave_interface::{
    audio::{CollisionSoundType, PlaySoundEffectEvent, SoundEffectType},
    player::PlayerOutgoingDamageComponent,
    spawnable::{Faction, MobSegmentType, MobType, ProjectileType},
};

use super::{CollidingEntityPair, SortedCollisionEvent};

/// Creates events from contact collisions
#[allow(clippy::too_many_arguments)]
pub(super) fn contact_collision_system(
    mut collision_event_writer: EventWriter<SortedCollisionEvent>,
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<(Entity, &PlayerOutgoingDamageComponent)>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    barrier_query: Query<Entity, With<ArenaBarrierComponent>>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
) {
    'collision_events: for contact_event in collision_events.read() {
        if let CollisionEvent::Stopped(collider1_entity, collider2_entity, _) = contact_event {
            // Prioritize by the importance of components to eliminate cases to check due to
            // `x collided with y` and `y collided with x` symmetry
            let colliding_entities = {
                let mut entities = [*collider1_entity, *collider2_entity];
                // This sort order is key to making other logic in the function work
                // (0,...,1) ascending ordering. E.x. [...,mob segments,  mobs, players]
                entities.sort_by_key(|e| {
                    (
                        player_query.get(*e).is_ok(), // Most important. Check first
                        mob_query.get(*e).is_ok(),
                        mob_segment_query.get(*e).is_ok(),
                        projectile_query.get(*e).is_ok(), // least important thing to check.
                    )
                });
                CollidingEntityPair {
                    primary: entities[1],
                    secondary: entities[0],
                }
            };
            // Now we pattern match to dynamic dispatch based on component type.
            // check if player was collided with. (will be primary due to sort)
            if let Ok((_player_entity, player_damage)) =
                player_query.get(colliding_entities.primary)
            {
                // check if player collided with a mob
                if let Ok((_entity, mob_component)) = mob_query.get(colliding_entities.secondary) {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::Collision(
                            mob_component.collision_sound.clone(),
                        ),
                    });
                    collision_event_writer.send(SortedCollisionEvent::PlayerToMobContact {
                        player_entity: colliding_entities.primary,
                        mob_entity: colliding_entities.secondary,
                        player_damage: player_damage.collision_damage,
                        mob_damage: mob_component.collision_damage,
                    });
                    continue 'collision_events;
                }
                // check if player collided with a barrier
                else if barrier_query.get(colliding_entities.secondary).is_ok() {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BarrierBounce,
                    });
                    continue 'collision_events;
                }
                // check if player collided with segment
                else if let Ok((_mob_segment_entity, mob_segment_component)) =
                    mob_segment_query.get(colliding_entities.secondary)
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::Collision(
                            mob_segment_component.collision_sound.clone(),
                        ),
                    });
                    collision_event_writer.send(SortedCollisionEvent::PlayerToMobSegmentContact {
                        player_entity: colliding_entities.primary,
                        mob_segment_entity: colliding_entities.secondary,
                        player_damage: player_damage.collision_damage,
                        mob_segment_damage: mob_segment_component.collision_damage,
                    });
                    continue 'collision_events;
                }
                // check if player collided with a projectile
                else if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(SortedCollisionEvent::PlayerToProjectileContact {
                        player_entity: colliding_entities.primary,
                        projectile_entity: colliding_entities.secondary,
                        projectile_faction: match projectile_component.projectile_type.clone() {
                            ProjectileType::Blast(faction) => faction,
                            ProjectileType::Bullet(faction) => faction,
                        },
                        projectile_damage: projectile_component.damage,
                    });
                    continue 'collision_events;
                }
            }
            // check if mob was the 'most important thing' that was in the collision
            else if let Ok((_mob_entity_1, mob_component_1)) =
                mob_query.get(colliding_entities.primary)
            {
                // check if mob collided with other mob
                if let Ok((_mob_entity, mob_component_2)) =
                    mob_query.get(colliding_entities.secondary)
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::Collision(
                            if mob_component_1.collision_sound != CollisionSoundType::default() {
                                mob_component_1.collision_sound.clone()
                            } else if mob_component_2.collision_sound
                                != CollisionSoundType::default()
                            {
                                mob_component_2.collision_sound.clone()
                            } else {
                                CollisionSoundType::default()
                            },
                        ),
                    });

                    // send two sorted collision events, swapping the position of the mobs in the struct
                    collision_event_writer.send(SortedCollisionEvent::MobToMobContact {
                        mob_entity_1: colliding_entities.primary,
                        mob_damage_2: mob_component_2.collision_damage,
                    });
                    collision_event_writer.send(SortedCollisionEvent::MobToMobContact {
                        mob_entity_1: colliding_entities.secondary,
                        mob_damage_2: mob_component_1.collision_damage,
                    });
                    continue 'collision_events;
                }
                // check if mob collided with barrier
                else if let Ok(_barrier_entity) = barrier_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(SortedCollisionEvent::MobToBarrierContact {
                        mob_entity: colliding_entities.primary,
                    });
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BarrierBounce,
                    });
                    continue 'collision_events;
                }
                // check if mob collided with mob segment
                else if let Ok((_mob_segment_entity, mob_segment_component)) =
                    mob_segment_query.get(colliding_entities.secondary)
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::Collision(
                            if mob_component_1.collision_sound != CollisionSoundType::default() {
                                mob_component_1.collision_sound.clone()
                            } else if mob_segment_component.collision_sound
                                != CollisionSoundType::default()
                            {
                                mob_segment_component.collision_sound.clone()
                            } else {
                                CollisionSoundType::default()
                            },
                        ),
                    });

                    collision_event_writer.send(SortedCollisionEvent::MobToMobSegmentContact {
                        mob_entity: colliding_entities.primary,
                        mob_damage: mob_component_1.collision_damage,
                        mob_segment_entity: colliding_entities.secondary,
                        mob_segment_damage: mob_segment_component.collision_damage,
                    });
                    continue 'collision_events;
                }
                // check if mob collided with projectile
                else if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    collision_event_writer.send(SortedCollisionEvent::MobToProjectileContact {
                        projectile_source: projectile_component.source,
                        mob_entity: colliding_entities.primary,
                        projectile_entity: colliding_entities.secondary,
                        projectile_faction: match &projectile_component.projectile_type {
                            ProjectileType::Blast(faction) => faction.clone(),
                            ProjectileType::Bullet(faction) => faction.clone(),
                        },
                        mob_faction: match mob_component_1.mob_type {
                            MobType::Enemy(_) => Faction::Enemy,
                            MobType::Ally(_) => Faction::Ally,
                            MobType::Neutral(_) => Faction::Neutral,
                        },
                        projectile_damage: projectile_component.damage,
                    });
                    continue 'collision_events;
                }
            }
            // check if mob segment was the 'most important thing' that was in the collision
            else if let Ok((_mob_segment_entity_1, mob_segment_component_1)) =
                mob_segment_query.get(colliding_entities.primary)
            {
                // check if mob segment collided with other mob segment
                if let Ok((_mob_segment_entity_2, mob_segment_component_2)) =
                    mob_segment_query.get(colliding_entities.secondary)
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::Collision(
                            if mob_segment_component_1.collision_sound
                                != CollisionSoundType::default()
                            {
                                mob_segment_component_1.collision_sound.clone()
                            } else if mob_segment_component_2.collision_sound
                                != CollisionSoundType::default()
                            {
                                mob_segment_component_2.collision_sound.clone()
                            } else {
                                CollisionSoundType::default()
                            },
                        ),
                    });

                    // send two sorted collision events, swapping the position of the mob segments in the struct
                    collision_event_writer.send(
                        SortedCollisionEvent::MobSegmentToMobSegmentContact {
                            mob_segment_entity_1: colliding_entities.primary,
                            mob_segment_damage_2: mob_segment_component_2.collision_damage,
                        },
                    );
                    collision_event_writer.send(
                        SortedCollisionEvent::MobSegmentToMobSegmentContact {
                            mob_segment_entity_1: colliding_entities.secondary,
                            mob_segment_damage_2: mob_segment_component_1.collision_damage,
                        },
                    );
                    continue 'collision_events;
                }
                // check if mob segment collided with projectile
                else if let Ok((_projectile_entity, projectile_component)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BulletBounce,
                    });

                    collision_event_writer.send(
                        SortedCollisionEvent::MobSegmentToProjectileContact {
                            mob_segment_entity: colliding_entities.primary,
                            projectile_entity: colliding_entities.secondary,
                            projectile_faction: match &projectile_component.projectile_type {
                                ProjectileType::Blast(faction) => faction.clone(),
                                ProjectileType::Bullet(faction) => faction.clone(),
                            },
                            mob_segment_faction: match mob_segment_component_1.mob_segment_type {
                                MobSegmentType::Enemy(_) => Faction::Enemy,
                                MobSegmentType::Neutral(_) => Faction::Neutral,
                            },
                            projectile_damage: projectile_component.damage,
                        },
                    );
                    continue 'collision_events;
                }
                // check if mob segment collided with barrier
                else if barrier_query.get(colliding_entities.secondary).is_ok() {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BarrierBounce,
                    });
                    continue 'collision_events;
                }
            }
            // check if projectile was the 'most important thing' that was in the collision
            else if let Ok((projectile_entity_1, projectile_component_1)) =
                projectile_query.get(colliding_entities.primary)
            {
                // check if the projectile collided with another projectile
                if let Ok((projectile_entity_2, projectile_component_2)) =
                    projectile_query.get(colliding_entities.secondary)
                {
                    if matches!(
                        projectile_component_1.projectile_type,
                        ProjectileType::Bullet(_)
                    ) && matches!(
                        projectile_component_2.projectile_type,
                        ProjectileType::Bullet(_)
                    ) {
                        collision_event_writer.send(
                            SortedCollisionEvent::ProjectileToProjectileContact {
                                projectile_entity_1,
                                projectile_faction_1: match &projectile_component_1.projectile_type
                                {
                                    ProjectileType::Blast(faction) => faction.clone(),
                                    ProjectileType::Bullet(faction) => faction.clone(),
                                },
                            },
                        );
                        collision_event_writer.send(
                            SortedCollisionEvent::ProjectileToProjectileContact {
                                projectile_entity_1: projectile_entity_2,
                                projectile_faction_1: match &projectile_component_2.projectile_type
                                {
                                    ProjectileType::Blast(faction) => faction.clone(),
                                    ProjectileType::Bullet(faction) => faction.clone(),
                                },
                            },
                        );
                        continue 'collision_events;
                    }
                }
            }
        }
    }
}
