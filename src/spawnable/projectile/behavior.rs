use crate::{
    collision::SortedCollisionEvent,
    spawnable::{MobComponent, MobSegmentComponent, SpawnEffectEvent},
};
use bevy::prelude::*;
use serde::Deserialize;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    health::DamageDealtEvent,
    player::PlayerComponent,
    spawnable::{EffectType, Faction, ProjectileType},
};

use super::ProjectileComponent;

/// Types of behaviors that can be performed by projectiles
#[derive(Deserialize, Clone)]
pub enum ProjectileBehavior {
    ExplodeOnIntersection,
    ExplodeOnContact,
    DealDamageOnIntersection,
    DealDamageOnContact,
    TimedDespawn { despawn_time: f32 },
}

/// Manages executing behaviors of all projectiles
#[allow(clippy::too_many_arguments)]
pub fn projectile_execute_behavior_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &mut ProjectileComponent)>,
    player_query: Query<(Entity, &PlayerComponent)>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    time: Res<Time>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    mut damage_dealt_event_writer: EventWriter<DamageDealtEvent>,
) {
    // Put all collision events in a vec so they can be read more than once
    let collision_events_vec: Vec<_> = collision_events.read().collect();
    // iterate through all projectiles
    for (entity, projectile_transform, mut projectile_component) in projectile_query.iter_mut() {
        let projectile_type = projectile_component.projectile_type.clone();
        for behavior in projectile_component.behaviors.clone() {
            match behavior {
                ProjectileBehavior::ExplodeOnIntersection => explode_on_intersection(
                    &mut commands,
                    entity,
                    projectile_transform,
                    &collision_events_vec,
                    &mut spawn_effect_event_writer,
                    &mut sound_effect_event_writer,
                ),
                ProjectileBehavior::ExplodeOnContact => explode_on_contact(
                    &mut commands,
                    entity,
                    projectile_transform,
                    &collision_events_vec,
                    &mut spawn_effect_event_writer,
                    &mut sound_effect_event_writer,
                ),
                ProjectileBehavior::DealDamageOnContact => deal_damage_on_contact(
                    entity,
                    &collision_events_vec,
                    &player_query,
                    &mob_query,
                    &mob_segment_query,
                    &mut sound_effect_event_writer,
                    &mut damage_dealt_event_writer,
                ),
                ProjectileBehavior::DealDamageOnIntersection => deal_damage_on_intersection(
                    entity,
                    &collision_events_vec,
                    &player_query,
                    &mob_query,
                    &mob_segment_query,
                    &mut sound_effect_event_writer,
                    &mut damage_dealt_event_writer,
                ),
                ProjectileBehavior::TimedDespawn { despawn_time } => {
                    projectile_component.time_alive += time.delta_seconds();
                    if projectile_component.time_alive > despawn_time {
                        match &projectile_type {
                            ProjectileType::Blast(faction) => match faction {
                                Faction::Enemy => {
                                    spawn_effect_event_writer.send(SpawnEffectEvent {
                                        effect_type: EffectType::EnemyBlastDespawn,
                                        transform: Transform {
                                            translation: projectile_transform.translation,
                                            scale: projectile_transform.scale,
                                            ..Default::default()
                                        },
                                        ..default()
                                    });
                                }
                                Faction::Ally => {
                                    spawn_effect_event_writer.send(SpawnEffectEvent {
                                        effect_type: EffectType::AllyBlastDespawn,
                                        transform: Transform {
                                            translation: projectile_transform.translation,
                                            scale: projectile_transform.scale,
                                            ..Default::default()
                                        },
                                        ..default()
                                    });
                                }
                                _ => {}
                            },
                            ProjectileType::Bullet(faction) => match faction {
                                Faction::Enemy => {
                                    spawn_effect_event_writer.send(SpawnEffectEvent {
                                        effect_type: EffectType::EnemyBulletDespawn,
                                        transform: Transform {
                                            translation: projectile_transform.translation,
                                            scale: projectile_transform.scale,
                                            ..Default::default()
                                        },
                                        ..default()
                                    });
                                }

                                Faction::Ally => {
                                    spawn_effect_event_writer.send(SpawnEffectEvent {
                                        effect_type: EffectType::AllyBulletDespawn,
                                        transform: Transform {
                                            translation: projectile_transform.translation,
                                            scale: projectile_transform.scale,
                                            ..Default::default()
                                        },
                                        ..default()
                                    });
                                }
                                _ => {}
                            },
                        }

                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn deal_damage_on_contact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    player_query: &Query<(Entity, &PlayerComponent)>,
    mob_query: &Query<(Entity, &MobComponent)>,
    mob_segment_query: &Query<(Entity, &MobSegmentComponent)>,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    damage_dealt_event_writer: &mut EventWriter<DamageDealtEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileContact {
                player_entity,
                projectile_entity,
                projectile_faction,
                player_damage: _,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && matches!(
                        projectile_faction.clone(),
                        Faction::Neutral | Faction::Enemy
                    )
                {
                    // deal damage to player
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::PlayerHit,
                    });
                    if player_query.contains(*player_entity) && *projectile_damage > 0 {
                        damage_dealt_event_writer.send(DamageDealtEvent {
                            damage: *projectile_damage,
                            target: *player_entity,
                        });
                    }

                    continue;
                }
            }
            SortedCollisionEvent::MobToProjectileContact {
                mob_entity,
                projectile_entity,
                mob_faction,
                projectile_faction,
                projectile_damage,
                projectile_source: _,
            } => {
                if entity == *projectile_entity
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    // deal damage to mob
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BulletDing,
                    });
                    if mob_query.contains(*mob_entity) && *projectile_damage > 0 {
                        damage_dealt_event_writer.send(DamageDealtEvent {
                            damage: *projectile_damage,
                            target: *mob_entity,
                        });
                    }

                    continue;
                }
            }
            SortedCollisionEvent::MobSegmentToProjectileContact {
                mob_segment_entity,
                projectile_entity,
                mob_segment_faction,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    // deal damage to mob
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BulletDing,
                    });
                    if mob_segment_query.contains(*mob_segment_entity) && *projectile_damage > 0 {
                        damage_dealt_event_writer.send(DamageDealtEvent {
                            damage: *projectile_damage,
                            target: *mob_segment_entity,
                        });
                    }

                    continue;
                }
            }
            _ => {}
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn deal_damage_on_intersection(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    player_query: &Query<(Entity, &PlayerComponent)>,
    mob_query: &Query<(Entity, &MobComponent)>,
    mob_segment_query: &Query<(Entity, &MobSegmentComponent)>,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    damage_dealt_event_writer: &mut EventWriter<DamageDealtEvent>,
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
                    && player_query.contains(*player_entity)
                    && *projectile_damage > 0
                {
                    // deal damage to player
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *player_entity,
                    });
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::PlayerHit,
                    });
                }
            }

            SortedCollisionEvent::MobToProjectileIntersection {
                mob_entity,
                projectile_entity,
                mob_faction,
                projectile_faction,
                projectile_damage,
                projectile_source: _,
            } => {
                if entity == *projectile_entity
                    && mob_faction != projectile_faction
                    && mob_query.contains(*mob_entity)
                    && *projectile_damage > 0
                {
                    // deal damage to mob
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_entity,
                    });
                }
            }
            SortedCollisionEvent::MobSegmentToProjectileIntersection {
                mob_segment_entity,
                projectile_entity,
                mob_segment_faction,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && mob_segment_faction != projectile_faction
                    && mob_segment_query.contains(*mob_segment_entity)
                    && *projectile_damage > 0
                {
                    // deal damage to mob
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_segment_entity,
                    });
                }
            }
            _ => {}
        }
    }
}

/// Explode projectile on impact
fn explode_on_intersection(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    collision_events: &[&SortedCollisionEvent],
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileIntersection {
                player_entity: _,
                projectile_entity,
                projectile_faction,
                projectile_damage: _,
            } => {
                if entity == *projectile_entity
                    && matches!(
                        projectile_faction.clone(),
                        Faction::Neutral | Faction::Enemy
                    )
                {
                    // spawn explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::EnemyBlastExplosion,
                        transform: Transform {
                            translation: transform.translation,
                            scale: transform.scale,
                            ..Default::default()
                        },
                        ..default()
                    });

                    // despawn blast
                    commands.entity(entity).despawn_recursive();

                    continue;
                }
            }

            SortedCollisionEvent::MobToProjectileIntersection {
                mob_entity: _,
                projectile_entity,
                mob_faction,
                projectile_faction,
                projectile_damage: _,
                projectile_source: _,
            } => {
                if entity == *projectile_entity
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBlastExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Enemy => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::EnemyBlastExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            SortedCollisionEvent::MobSegmentToProjectileIntersection {
                mob_segment_entity: _,
                projectile_entity,
                mob_segment_faction,
                projectile_faction,
                projectile_damage: _,
            } => {
                if entity == *projectile_entity
                    && !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBlastExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Enemy => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::EnemyBlastExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            _ => {}
        }
    }
}

/// Explode projectile on impact
fn explode_on_contact(
    commands: &mut Commands,
    entity: Entity,
    transform: &Transform,
    collision_events: &[&SortedCollisionEvent],
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileContact {
                player_entity: _,
                projectile_entity,
                projectile_faction,
                projectile_damage: _,
                player_damage: _,
            } => {
                if entity == *projectile_entity
                    && matches!(
                        projectile_faction.clone(),
                        Faction::Neutral | Faction::Enemy
                    )
                {
                    // spawn explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::EnemyBulletExplosion,
                        transform: Transform {
                            translation: transform.translation,
                            scale: transform.scale,
                            ..Default::default()
                        },
                        ..default()
                    });

                    // despawn blast
                    commands.entity(entity).despawn_recursive();

                    continue;
                }
            }

            SortedCollisionEvent::MobToProjectileContact {
                mob_entity: _,
                projectile_entity,
                mob_faction: _,
                projectile_faction,
                projectile_damage: _,
                projectile_source: _,
            } => {
                if entity == *projectile_entity {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Enemy => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::EnemyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }

            SortedCollisionEvent::MobSegmentToProjectileContact {
                mob_segment_entity: _,
                projectile_entity,
                mob_segment_faction: _,
                projectile_faction,
                projectile_damage: _,
            } => {
                if entity == *projectile_entity {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Enemy => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::EnemyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }

            SortedCollisionEvent::ProjectileToProjectileContact {
                projectile_entity_1,
                projectile_faction_1,
                projectile_entity_2: _,
                projectile_faction_2: _,
            } => {
                if entity == *projectile_entity_1 {
                    //audio_channel.play(audio_assets.mob_hit.clone());
                    match projectile_faction_1 {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Enemy => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::EnemyBulletExplosion,
                                transform: Transform {
                                    translation: transform.translation,
                                    scale: transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            _ => {}
        }
    }
}
