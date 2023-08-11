use std::thread::spawn;

use crate::{
    assets::GameAudioAssets,
    audio,
    collision::SortedCollisionEvent,
    spawnable::{
        InitialMotion, MobComponent, MobSegmentComponent, PlayerComponent, SpawnEffectEvent,
    },
};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use serde::Deserialize;
use thetawave_interface::spawnable::{EffectType, Faction, ProjectileType, TextEffectType};

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

/// Manages executing behaviors of mobs
#[allow(clippy::too_many_arguments)]
pub fn projectile_execute_behavior_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &mut ProjectileComponent)>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut mob_query: Query<(Entity, &Transform, &mut MobComponent)>,
    mut mob_segment_query: Query<(Entity, &Transform, &mut MobSegmentComponent)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    time: Res<Time>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    // Put all collision events in a vec so they can be read more than once
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

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
                    &audio_channel,
                    &audio_assets,
                ),
                ProjectileBehavior::ExplodeOnContact => explode_on_contact(
                    &mut commands,
                    entity,
                    projectile_transform,
                    &collision_events_vec,
                    &mut spawn_effect_event_writer,
                    &audio_channel,
                    &audio_assets,
                ),
                ProjectileBehavior::DealDamageOnContact => deal_damage_on_contact(
                    entity,
                    &collision_events_vec,
                    &mut player_query,
                    &mut mob_query,
                    &mut mob_segment_query,
                    &audio_channel,
                    &audio_assets,
                    &mut spawn_effect_event_writer,
                ),
                ProjectileBehavior::DealDamageOnIntersection => deal_damage_on_intersection(
                    entity,
                    &collision_events_vec,
                    &mut player_query,
                    &mut mob_query,
                    &mut mob_segment_query,
                    &audio_channel,
                    &audio_assets,
                    &mut spawn_effect_event_writer,
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
                                        initial_motion: InitialMotion::default(),
                                        text: None,
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
                                        initial_motion: InitialMotion::default(),
                                        text: None,
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
                                        initial_motion: InitialMotion::default(),
                                        text: None,
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
                                        initial_motion: InitialMotion::default(),
                                        text: None,
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
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    mob_query: &mut Query<(Entity, &Transform, &mut MobComponent)>,
    mob_segment_query: &mut Query<(Entity, &Transform, &mut MobSegmentComponent)>,
    audio_channel: &AudioChannel<audio::SoundEffectsAudioChannel>,
    audio_assets: &GameAudioAssets,
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
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
                    audio_channel.play(audio_assets.player_hit.clone());
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if *player_entity == player_entity_q {
                            player_component.health.take_damage(*projectile_damage);
                        }
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
                    audio_channel.play(audio_assets.bullet_ding.clone());
                    for (mob_entity_q, mob_transform, mut mob_component) in mob_query.iter_mut() {
                        if *mob_entity == mob_entity_q && *projectile_damage > 0.0 {
                            mob_component.health.take_damage(*projectile_damage);
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                                transform: Transform {
                                    translation: mob_transform.translation,
                                    scale: mob_transform.scale,
                                    ..Default::default()
                                },
                                initial_motion: InitialMotion::default(),
                                text: Some(projectile_damage.to_string()),
                            });
                        }
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
                    audio_channel.play(audio_assets.bullet_ding.clone());
                    for (mob_segment_entity_q, mob_segment_transform, mut mob_segment_component) in
                        mob_segment_query.iter_mut()
                    {
                        if *mob_segment_entity == mob_segment_entity_q && *projectile_damage > 0.0 {
                            mob_segment_component.health.take_damage(*projectile_damage);
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                                transform: Transform {
                                    translation: mob_segment_transform.translation,
                                    scale: mob_segment_transform.scale,
                                    ..Default::default()
                                },
                                initial_motion: InitialMotion::default(),
                                text: Some(projectile_damage.to_string()),
                            });
                        }
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
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    mob_query: &mut Query<(Entity, &Transform, &mut MobComponent)>,
    mob_segment_query: &mut Query<(Entity, &Transform, &mut MobSegmentComponent)>,
    audio_channel: &AudioChannel<audio::SoundEffectsAudioChannel>,
    audio_assets: &GameAudioAssets,
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
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
                    // deal damage to player
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if *player_entity == player_entity_q {
                            player_component.health.take_damage(*projectile_damage);
                            audio_channel.play(audio_assets.player_hit.clone());
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
                    for (mob_entity_q, mob_transform, mut mob_component) in mob_query.iter_mut() {
                        if *mob_entity == mob_entity_q && *projectile_damage > 0.0 {
                            mob_component.health.take_damage(*projectile_damage);
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                                transform: Transform {
                                    translation: mob_transform.translation,
                                    scale: mob_transform.scale,
                                    ..Default::default()
                                },
                                initial_motion: InitialMotion::default(),
                                text: Some(projectile_damage.to_string()),
                            });
                        }
                    }

                    continue;
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
                    && !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    // deal damage to mob
                    for (mob_segment_entity_q, mob_segment_transform, mut mob_segment_component) in
                        mob_segment_query.iter_mut()
                    {
                        if *mob_segment_entity == mob_segment_entity_q && *projectile_damage > 0.0 {
                            mob_segment_component.health.take_damage(*projectile_damage);
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::Text(TextEffectType::DamageDealt),
                                transform: Transform {
                                    translation: mob_segment_transform.translation,
                                    scale: mob_segment_transform.scale,
                                    ..Default::default()
                                },
                                initial_motion: InitialMotion::default(),
                                text: Some(projectile_damage.to_string()),
                            });
                        }
                    }

                    continue;
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
    audio_channel: &AudioChannel<audio::SoundEffectsAudioChannel>,
    audio_assets: &GameAudioAssets,
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
                        initial_motion: InitialMotion::default(),
                        text: None,
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
                    audio_channel.play(audio_assets.mob_hit.clone());
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                    audio_channel.play(audio_assets.mob_hit.clone());
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
    audio_channel: &AudioChannel<audio::SoundEffectsAudioChannel>,
    audio_assets: &GameAudioAssets,
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
                        initial_motion: InitialMotion::default(),
                        text: None,
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
                    audio_channel.play(audio_assets.mob_hit.clone());
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                    audio_channel.play(audio_assets.mob_hit.clone());
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
                                initial_motion: InitialMotion::default(),
                                text: None,
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
