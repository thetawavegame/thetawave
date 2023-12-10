use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    health::{DamageDealtEvent, HealthComponent},
    player::PlayerComponent,
    spawnable::{
        EffectType, MobDestroyedEvent, MobType, ProjectileType, SpawnItemEvent, SpawnMobEvent,
    },
};

use crate::{
    collision::SortedCollisionEvent,
    game::GameParametersResource,
    loot::LootDropsResource,
    spawnable::{InitialMotion, SpawnConsumableEvent, SpawnEffectEvent, SpawnProjectileEvent},
};

use super::{MobComponent, SpawnPosition};

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum MobBehavior {
    PeriodicFire(String),
    SpawnMob(String),
    ExplodeOnImpact,
    DealDamageToPlayerOnImpact,
    ReceiveDamageOnImpact,
    DieAtZeroHealth,
}

#[derive(Deserialize, Hash, PartialEq, Eq, Clone)]
pub enum MobSegmentControlBehavior {
    RepeaterProtectHead,
    RepeaterAttack,
}

/// Data used to periodically spawn mobs
#[derive(Deserialize, Clone)]
pub struct SpawnMobBehaviorData {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Offset from center of source entity
    pub offset_position: Vec2,
    /// Period between spawnings
    pub period: f32,
}

/// Data used to periodically spawn mobs
#[derive(Deserialize, Clone)]
pub struct PeriodicFireBehaviorData {
    /// Type of mob to spawn
    pub projectile_type: ProjectileType,
    /// Offset from center of source entity
    pub offset_position: Vec2,
    /// Initial motion of soawned projectile
    pub initial_motion: InitialMotion,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Period between spawnings
    pub period: f32,
}

#[allow(clippy::too_many_arguments)]
/// Manages excuteing behaviors of mobs
pub fn mob_execute_behavior_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    time: Res<Time>,
    mut mob_query: Query<(
        Entity,
        &mut MobComponent,
        &Transform,
        &Velocity,
        &HealthComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut spawn_consumable_event_writer: EventWriter<SpawnConsumableEvent>,
    mut spawn_item_event_writer: EventWriter<SpawnItemEvent>,
    mut spawn_projectile_event_writer: EventWriter<SpawnProjectileEvent>,
    mut spawn_mob_event_writer: EventWriter<SpawnMobEvent>,
    mut mob_destroyed_event_writer: EventWriter<MobDestroyedEvent>,
    mut damage_dealt_event_writer: EventWriter<DamageDealtEvent>,
    loot_drops_resource: Res<LootDropsResource>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    game_parameters: Res<GameParametersResource>,
) {
    // Get all contact events first (can't be read more than once within a system)
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.read() {
        collision_events_vec.push(collision_event);
    }

    // Iterate through all spawnable entities and execute their behavior
    for (entity, mut mob_component, mob_transform, mob_velocity, mob_health) in mob_query.iter_mut()
    {
        let behaviors = mob_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                MobBehavior::PeriodicFire(projectile_spawner_key) => {
                    let attack_damage = mob_component.attack_damage;

                    // get all the mob spawners under the given key
                    let projectile_spawners = mob_component
                        .projectile_spawners
                        .get_mut(&projectile_spawner_key)
                        .unwrap();

                    for projectile_spawner in projectile_spawners.iter_mut() {
                        projectile_spawner.timer.tick(time.delta());

                        if projectile_spawner.timer.just_finished() {
                            let projectile_transform = Transform {
                                translation: match projectile_spawner.position {
                                    SpawnPosition::Global(coords) => coords.extend(1.0),
                                    SpawnPosition::Local(coords) => {
                                        (mob_transform.translation.xy()
                                            + mob_transform.local_x().xy() * coords.x
                                            + mob_transform.local_y().xy() * coords.y)
                                            .extend(1.0)
                                    }
                                },
                                ..Default::default()
                            };

                            // add mob velocity to initial blast velocity
                            let mut modified_initial_motion =
                                projectile_spawner.initial_motion.clone();

                            if let Some(linvel) = &mut modified_initial_motion.linvel {
                                linvel.x += mob_velocity.linvel.x;
                                linvel.y += mob_velocity.linvel.y;
                            }

                            //spawn_blast
                            sound_effect_event_writer.send(PlaySoundEffectEvent {
                                sound_effect_type: SoundEffectType::EnemyFireBlast,
                            });

                            spawn_projectile_event_writer.send(SpawnProjectileEvent {
                                projectile_type: projectile_spawner.projectile_type.clone(),
                                transform: projectile_transform,
                                damage: attack_damage,
                                despawn_time: projectile_spawner.despawn_time,
                                initial_motion: modified_initial_motion,
                                source: entity,
                                projectile_direction: 4.71239,
                                projectile_count: 1, // TODO: get stat from mobs
                            });
                        }
                    }
                }
                MobBehavior::SpawnMob(mob_spawner_key) => {
                    // get data

                    // if mob component does not have a timer initialize timer
                    // otherwise tick timer and spawn mob on completion

                    // get all the mob spawners under the given key
                    let mob_spawners = mob_component
                        .mob_spawners
                        .get_mut(&mob_spawner_key)
                        .unwrap();

                    for mob_spawner in mob_spawners.iter_mut() {
                        mob_spawner.timer.tick(time.delta());

                        if mob_spawner.timer.just_finished() {
                            // spawn mob
                            let position = match mob_spawner.position {
                                SpawnPosition::Global(coords) => coords,
                                SpawnPosition::Local(coords) => {
                                    mob_transform.translation.xy()
                                        + mob_transform.local_x().xy() * coords.x
                                        + mob_transform.local_y().xy() * coords.y
                                }
                            };

                            spawn_mob_event_writer.send(SpawnMobEvent {
                                mob_type: mob_spawner.mob_type.clone(),
                                position,
                                rotation: mob_transform.rotation, // passed rotation of the parent mob
                                boss: false,
                            });
                        }
                    }
                }
                MobBehavior::ExplodeOnImpact => {
                    explode_on_impact(
                        &mut commands,
                        entity,
                        &collision_events_vec,
                        &mut spawn_effect_event_writer,
                        mob_transform,
                        &game_parameters,
                        &mut sound_effect_event_writer,
                    );
                }
                MobBehavior::DealDamageToPlayerOnImpact => {
                    deal_damage_to_player_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut player_query,
                        &mut damage_dealt_event_writer,
                    );
                }
                MobBehavior::ReceiveDamageOnImpact => {
                    receive_damage_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut player_query,
                        &mut damage_dealt_event_writer,
                    );
                }
                MobBehavior::DieAtZeroHealth => {
                    if mob_health.is_dead() {
                        sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::MobExplosion,
                        });

                        // spawn mob explosion
                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::MobExplosion,
                            transform: Transform {
                                translation: mob_transform.translation,
                                scale: Vec3::new(
                                    game_parameters.sprite_scale,
                                    game_parameters.sprite_scale,
                                    1.0,
                                ),
                                ..Default::default()
                            },
                            ..default()
                        });

                        // drop loot
                        loot_drops_resource.spawn_loot_drops(
                            &mob_component.loot_drops,
                            &mut spawn_consumable_event_writer,
                            &mut spawn_item_event_writer,
                            mob_transform.translation.xy(),
                        );

                        // despawn mob
                        commands.entity(entity).despawn_recursive();

                        // apply disconnected behaviors to connected mob segments
                        mob_destroyed_event_writer.send(MobDestroyedEvent {
                            entity,
                            mob_type: mob_component.mob_type.clone(),
                        });
                    }
                }
            }
        }
    }
}

/// Take damage from colliding entity on impact
fn receive_damage_on_impact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    damage_dealt_event_writer: &mut EventWriter<DamageDealtEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToMobContact {
                player_entity,
                mob_entity,
                mob_faction: _,
                player_damage,
                mob_damage: _,
            } => {
                if entity == *mob_entity {
                    for (player_entity_q, mut _player_component) in player_query.iter_mut() {
                        if player_entity_q == *player_entity && *player_damage > 0 {
                            damage_dealt_event_writer.send(DamageDealtEvent {
                                damage: *player_damage,
                                target: *mob_entity,
                            });
                        }
                    }
                }
            }
            SortedCollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1: _,
                mob_damage_1: _,
                mob_entity_2: _,
                mob_faction_2: _,
                mob_damage_2,
            } => {
                if entity == *mob_entity_1 && *mob_damage_2 > 0 {
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *mob_damage_2,
                        target: *mob_entity_1,
                    });
                }
            }
            SortedCollisionEvent::MobToMobSegmentContact {
                mob_entity,
                mob_faction: _,
                mob_damage: _,
                mob_segment_entity: _,
                mob_segment_faction: _,
                mob_segment_damage,
            } => {
                if entity == *mob_entity && *mob_segment_damage > 0 {
                    damage_dealt_event_writer.send(DamageDealtEvent {
                        damage: *mob_segment_damage,
                        target: *mob_entity,
                    });
                }
            }

            _ => {}
        }
    }
}

/// Deal damage to colliding entity on impact
fn deal_damage_to_player_on_impact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    damage_dealt_event_writer: &mut EventWriter<DamageDealtEvent>,
) {
    for collision_event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToMobContact {
            player_entity,
            mob_entity,
            mob_faction: _,
            player_damage: _,
            mob_damage,
        } = collision_event
        {
            if entity == *mob_entity {
                // deal damage to player
                for (player_entity_q, player_component) in player_query.iter_mut() {
                    let damage = (player_component.incoming_damage_multiplier * *mob_damage as f32)
                        .round() as usize;
                    if player_entity_q == *player_entity && damage > 0 {
                        damage_dealt_event_writer.send(DamageDealtEvent {
                            damage,
                            target: player_entity_q,
                        })
                    }
                }
            }
        }
    }
}

/// Explode spawnable on impact
#[allow(clippy::too_many_arguments)]
fn explode_on_impact(
    commands: &mut Commands,
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    transform: &Transform,
    game_parameters: &GameParametersResource,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToMobContact {
                player_entity: _,
                mob_entity,
                mob_faction: _,
                player_damage: _,
                mob_damage: _,
            } => {
                sound_effect_event_writer.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::MobExplosion,
                });
                // remove faction check to allow allied mobs to harm players
                if entity == *mob_entity {
                    // spawn mob explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::MobExplosion,
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::new(
                                game_parameters.sprite_scale,
                                game_parameters.sprite_scale,
                                1.0,
                            ),
                            ..Default::default()
                        },
                        ..default()
                    });
                    // despawn mob
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            SortedCollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1: _,
                mob_damage_1: _,
                mob_entity_2: _,
                mob_faction_2: _,
                mob_damage_2: _,
            } => {
                sound_effect_event_writer.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::MobExplosion,
                });
                if entity == *mob_entity_1 {
                    // spawn mob explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::MobExplosion,
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::new(
                                game_parameters.sprite_scale,
                                game_parameters.sprite_scale,
                                1.0,
                            ),
                            ..Default::default()
                        },
                        ..default()
                    });
                    // despawn mob
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            SortedCollisionEvent::MobToMobSegmentContact {
                mob_entity,
                mob_faction: _,
                mob_damage: _,
                mob_segment_entity: _,
                mob_segment_faction: _,
                mob_segment_damage: _,
            } => {
                sound_effect_event_writer.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::MobExplosion,
                });
                if entity == *mob_entity {
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::MobExplosion,
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::new(
                                game_parameters.sprite_scale,
                                game_parameters.sprite_scale,
                                1.0,
                            ),
                            ..Default::default()
                        },
                        ..default()
                    });
                    commands.entity(entity).despawn_recursive();
                    continue;
                }
            }
            _ => {}
        }
    }
}
