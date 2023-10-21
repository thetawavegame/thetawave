use crate::{
    collision::SortedCollisionEvent,
    spawnable::{MobComponent, MobSegmentComponent, SpawnEffectEvent},
};
use bevy::{math::Vec3Swizzles, prelude::*};
use serde::Deserialize;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    health::DamageDealtEvent,
    player::PlayerComponent,
    spawnable::{EffectType, Faction, ProjectileType},
    states,
};

use super::ProjectileComponent;

pub struct ProjectileBehaviorPlugin;

impl Plugin for ProjectileBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                deal_damage_on_intersection_system,
                explode_on_intersection_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing))
                .chain(),
        );

        app.add_systems(
            Update,
            (deal_damage_on_contact_system, explode_on_contact_system)
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing))
                .chain(),
        );

        app.add_systems(
            Update,
            (timed_despawn_system, follow_source_system)
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Types of behaviors that can be performed by projectiles
#[derive(Deserialize, Clone)]
pub enum ProjectileBehavior {
    ExplodeOnIntersection,
    ExplodeOnContact,
    DealDamageOnIntersection,
    DealDamageOnContact,
    TimedDespawn { despawn_time: f32 },
    FollowSource,
}

#[derive(Component)]
pub struct ExplodeOnIntersection;

#[derive(Component)]
pub struct ExplodeOnContact;

#[derive(Component)]
pub struct DealDamageOnIntersection;

#[derive(Component)]
pub struct DealDamageOnContact;

#[derive(Component)]
pub struct TimedDespawn(pub Timer);

#[derive(Component)]
pub struct FollowSource {
    pub source: Entity,
    pub pos_vec: Vec2,
}

fn deal_damage_on_contact_system(
    mut collision_events: EventReader<SortedCollisionEvent>,
    projectile_query: Query<&DealDamageOnContact, With<ProjectileComponent>>,
    player_query: Query<&PlayerComponent>,
    mob_query: Query<&MobComponent>,
    mob_segment_query: Query<&MobSegmentComponent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    mut damage_dealt_event_writer: EventWriter<DamageDealtEvent>,
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
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !matches!(projectile_faction, Faction::Ally)
                    && player_query.contains(*player_entity)
                    && *projectile_damage > 0
                {
                    // play sound effect
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::PlayerHit,
                    });

                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *player_entity,
                    });

                    continue;
                }
            }
            SortedCollisionEvent::MobToProjectileContact {
                projectile_source: _,
                mob_entity,
                projectile_entity,
                projectile_faction,
                mob_faction,
                projectile_damage,
            } => {
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                    && mob_query.contains(*mob_entity)
                    && *projectile_damage > 0
                {
                    // play sound effect
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BulletDing,
                    });

                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_entity,
                    });

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
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                    && mob_segment_query.contains(*mob_segment_entity)
                    && *projectile_damage > 0
                {
                    // play sound effect
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::BulletDing,
                    });

                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_segment_entity,
                    });

                    continue;
                }
            }
            _ => {}
        }
    }
}

fn deal_damage_on_intersection_system(
    mut collision_events: EventReader<SortedCollisionEvent>,
    projectile_query: Query<&DealDamageOnIntersection, With<ProjectileComponent>>,
    player_query: Query<&PlayerComponent>,
    mob_query: Query<&MobComponent>,
    mob_segment_query: Query<&MobSegmentComponent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    mut damage_dealt_event_writer: EventWriter<DamageDealtEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileIntersection {
                player_entity,
                projectile_entity,
                projectile_faction,
                projectile_damage,
            } => {
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !matches!(projectile_faction, Faction::Ally)
                    && player_query.contains(*player_entity)
                    && *projectile_damage > 0
                {
                    // play sound effect
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::PlayerHit,
                    });

                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *player_entity,
                    });

                    continue;
                }
            }
            SortedCollisionEvent::MobToProjectileIntersection {
                projectile_source: _,
                mob_entity,
                projectile_entity,
                projectile_faction,
                mob_faction,
                projectile_damage,
            } => {
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                    && mob_query.contains(*mob_entity)
                    && *projectile_damage > 0
                {
                    info!("Mob to projectile intersection damage");
                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_entity,
                    });

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
                // checks to make sure that the projectile should deal damage
                if projectile_query.get(*projectile_entity).is_ok()
                    && !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                    && mob_segment_query.contains(*mob_segment_entity)
                    && *projectile_damage > 0
                {
                    // send a damage dealt event
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *projectile_damage,
                        target: *mob_segment_entity,
                    });

                    continue;
                }
            }
            _ => {}
        }
    }
}

fn explode_on_intersection_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    projectile_query: Query<(&ExplodeOnIntersection, &Transform), With<ProjectileComponent>>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToProjectileIntersection {
                player_entity: _,
                projectile_entity,
                projectile_faction,
                projectile_damage: _,
            } => {
                // checks to make sure that the projectile should deal damage
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    if !matches!(projectile_faction, Faction::Ally) {
                        // spawn explosion
                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::EnemyBlastExplosion,
                            transform: Transform {
                                translation: projectile_transform.translation,
                                scale: projectile_transform.scale,
                                ..Default::default()
                            },
                            ..default()
                        });

                        // despawn blas
                        commands.entity(*projectile_entity).despawn_recursive();

                        continue;
                    }
                }
            }
            SortedCollisionEvent::MobToProjectileIntersection {
                projectile_source: _,
                mob_entity: _,
                projectile_entity,
                projectile_faction,
                mob_faction,
                projectile_damage: _,
            } => {
                // checks to make sure that the projectile should deal damage
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    if !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    } {
                        sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::MobHit,
                        });

                        match projectile_faction {
                            Faction::Ally => {
                                // spawn explosion
                                spawn_effect_event_writer.send(SpawnEffectEvent {
                                    effect_type: EffectType::AllyBlastExplosion,
                                    transform: Transform {
                                        translation: projectile_transform.translation,
                                        scale: projectile_transform.scale,
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
                                        translation: projectile_transform.translation,
                                        scale: projectile_transform.scale,
                                        ..Default::default()
                                    },
                                    ..default()
                                });
                            }
                            Faction::Neutral => {}
                        }

                        // despawn blast
                        commands.entity(*projectile_entity).despawn_recursive();

                        continue;
                    }
                }
            }
            SortedCollisionEvent::MobSegmentToProjectileIntersection {
                mob_segment_entity: _,
                projectile_entity,
                mob_segment_faction,
                projectile_faction,
                projectile_damage: _,
            } => {
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    if !match mob_segment_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    } {
                        sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::MobHit,
                        });
                        match projectile_faction {
                            Faction::Ally => {
                                // spawn explosion
                                spawn_effect_event_writer.send(SpawnEffectEvent {
                                    effect_type: EffectType::AllyBlastExplosion,
                                    transform: Transform {
                                        translation: projectile_transform.translation,
                                        scale: projectile_transform.scale,
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
                                        translation: projectile_transform.translation,
                                        scale: projectile_transform.scale,
                                        ..Default::default()
                                    },
                                    ..default()
                                });
                            }
                            Faction::Neutral => {}
                        }

                        // despawn blast
                        commands.entity(*projectile_entity).despawn_recursive();
                        continue;
                    }
                }
            }
            _ => {}
        }
    }
}

fn explode_on_contact_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    projectile_query: Query<(&ExplodeOnContact, &Transform), With<ProjectileComponent>>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
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
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    if !matches!(projectile_faction.clone(), Faction::Ally) {
                        // spawn explosion
                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::EnemyBulletExplosion,
                            transform: Transform {
                                translation: projectile_transform.translation,
                                scale: projectile_transform.scale,
                                ..Default::default()
                            },
                            ..default()
                        });

                        // despawn blast
                        commands.entity(*projectile_entity).despawn_recursive();

                        continue;
                    }
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
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
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
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(*projectile_entity).despawn_recursive();
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
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity) {
                    sound_effect_event_writer.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::MobHit,
                    });
                    match projectile_faction {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
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
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(*projectile_entity).despawn_recursive();
                    continue;
                }
            }

            SortedCollisionEvent::ProjectileToProjectileContact {
                projectile_entity_1,
                projectile_faction_1,
                projectile_entity_2: _,
                projectile_faction_2: _,
            } => {
                if let Ok((_, projectile_transform)) = projectile_query.get(*projectile_entity_1) {
                    match projectile_faction_1 {
                        Faction::Ally => {
                            // spawn explosion
                            spawn_effect_event_writer.send(SpawnEffectEvent {
                                effect_type: EffectType::AllyBulletExplosion,
                                transform: Transform {
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
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
                                    translation: projectile_transform.translation,
                                    scale: projectile_transform.scale,
                                    ..Default::default()
                                },
                                ..default()
                            });
                        }
                        Faction::Neutral => {}
                    }

                    // despawn blast
                    commands.entity(*projectile_entity_1).despawn_recursive();
                    continue;
                }
            }
            _ => {}
        }
    }
}

fn timed_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<
        (Entity, &ProjectileComponent, &Transform, &mut TimedDespawn),
        With<ProjectileComponent>,
    >,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    for (projectile_entity, projectile_component, projectile_transform, mut timed_despawn) in
        projectile_query.iter_mut()
    {
        timed_despawn.0.tick(time.delta());
        if timed_despawn.0.just_finished() {
            commands.entity(projectile_entity).despawn_recursive();

            match &projectile_component.projectile_type {
                ProjectileType::Blast(faction) => match faction {
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
                    _ => {}
                },
                ProjectileType::Bullet(faction) => match faction {
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
                    _ => {}
                },
                ProjectileType::Beam(faction) => {}
            }
        }
    }
}

fn follow_source_system(
    mut projectile_query: Query<(&FollowSource, &mut Transform), With<ProjectileComponent>>,
    player_query: Query<(&Transform, &PlayerComponent), Without<ProjectileComponent>>,
) {
    for (follow_source, mut projectile_transform) in projectile_query.iter_mut() {
        if let Ok((player_transform, player_component)) = player_query.get(follow_source.source) {
            // get the base spawn point for a projectile
            let player_offset_pos =
                player_transform.translation.xy() + player_component.projectile_offset_position;

            // current difference between the projectile position and the base spawn position for projectile
            let current_pos_diff = projectile_transform.translation.xy() - player_offset_pos;

            // update the projectile transform by adding the difference between the follow position and the current difference in position
            //so that the projectile consistently follows the source
            projectile_transform.translation +=
                (follow_source.pos_vec - current_pos_diff).extend(0.0);
        }
    }
}
