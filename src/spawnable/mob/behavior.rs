use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

use crate::{
    collision::CollisionEvent,
    game::GameParametersResource,
    spawnable::{
        spawn_projectile, EffectType, InitialMotion, MobType, PlayerComponent, ProjectileResource,
        ProjectileType, SpawnEffectEvent, SpawnableComponent,
    },
};

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum MobBehavior {
    PeriodicFire(PeriodicFireBehaviorData),
    SpawnMob(SpawnMobBehaviorData),
    ExplodeOnImpact,
    DealDamageToPlayerOnImpact,
    ReceiveDamageOnImpact,
    DieAtZeroHealth,
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
    mut collision_events: EventReader<CollisionEvent>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    time: Res<Time>,
    mob_resource: Res<super::MobsResource>,
    projectile_resource: Res<ProjectileResource>,
    mut mob_query: Query<(
        Entity,
        &mut SpawnableComponent,
        &mut super::MobComponent,
        &RigidBodyPositionComponent,
        &RigidBodyVelocityComponent,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
) {
    // Get all contact events first (can't be read more than once within a system)
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    // Iterate through all spawnable entities and execute their behavior
    for (entity, mut spawnable_component, mut mob_component, rb_pos, rb_vel) in mob_query.iter_mut()
    {
        let behaviors = mob_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                MobBehavior::PeriodicFire(data) => {
                    if mob_component.weapon_timer.is_none() {
                        mob_component.weapon_timer = Some(Timer::from_seconds(data.period, true));
                    } else if let Some(timer) = &mut mob_component.weapon_timer {
                        timer.tick(time.delta());
                        if timer.just_finished() {
                            // spawn blast
                            let position = Vec2::new(
                                rb_pos.position.translation.x + data.offset_position.x,
                                rb_pos.position.translation.y + data.offset_position.y,
                            );

                            // add mob velocity to initial blast velocity
                            let mut modified_initial_motion = data.initial_motion.clone();

                            if let Some(linvel) = &mut modified_initial_motion.linvel {
                                linvel.x += rb_vel.linvel.x;
                                linvel.y += rb_vel.linvel.y;
                            }

                            //spawn_blast
                            spawn_projectile(
                                &data.projectile_type,
                                &projectile_resource,
                                position,
                                mob_component.attack_damage,
                                data.despawn_time,
                                modified_initial_motion,
                                &mut commands,
                                &rapier_config,
                                &game_parameters,
                            );
                        }
                    }
                }
                MobBehavior::SpawnMob(data) => {
                    // if mob component does not have a timer initialize timer
                    // otherwise tick timer and spawn mob on completion
                    if mob_component.mob_spawn_timer.is_none() {
                        mob_component.mob_spawn_timer =
                            Some(Timer::from_seconds(data.period, true));
                    } else if let Some(timer) = &mut mob_component.mob_spawn_timer {
                        timer.tick(time.delta());
                        if timer.just_finished() {
                            // spawn mob
                            let position = Vec2::new(
                                rb_pos.position.translation.x + data.offset_position.x,
                                rb_pos.position.translation.y + data.offset_position.y,
                            );

                            super::spawn_mob(
                                &data.mob_type,
                                &mob_resource,
                                position,
                                &mut commands,
                                &rapier_config,
                                &game_parameters,
                            )
                        }
                    }
                }
                MobBehavior::ExplodeOnImpact => {
                    explode_on_impact(
                        entity,
                        &mut spawnable_component,
                        &collision_events_vec,
                        &mut spawn_effect_event_writer,
                        &rb_pos,
                    );
                }
                MobBehavior::DealDamageToPlayerOnImpact => {
                    deal_damage_to_player_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut player_query,
                    );
                }
                MobBehavior::ReceiveDamageOnImpact => {
                    receive_damage_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut mob_component,
                        &mut player_query,
                    );
                }
                MobBehavior::DieAtZeroHealth => {
                    if mob_component.health.is_dead() {
                        spawnable_component.should_despawn = true;
                        // spawn mob explosion
                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::MobExplosion,
                            position: rb_pos.position.translation.into(),
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
    collision_events: &[&CollisionEvent],
    mob_component: &mut super::MobComponent,
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact {
                player_entity,
                mob_entity,
                mob_faction: _,
                player_damage,
                mob_damage: _,
            } => {
                if entity == *mob_entity {
                    for (player_entity_q, mut _player_component) in player_query.iter_mut() {
                        if player_entity_q == *player_entity {
                            mob_component.health.take_damage(*player_damage);
                        }
                    }
                }
            }
            CollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1: _,
                mob_damage_1: _,
                mob_entity_2: _,
                mob_faction_2: _,
                mob_damage_2,
            } => {
                if entity == *mob_entity_1 {
                    mob_component.health.take_damage(*mob_damage_2);
                }
            }

            _ => {}
        }
    }
}

/// Deal damage to colliding entity on impact
fn deal_damage_to_player_on_impact(
    entity: Entity,
    collision_events: &[&CollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        if let CollisionEvent::PlayerToMobContact {
            player_entity,
            mob_entity,
            mob_faction: _,
            player_damage: _,
            mob_damage,
        } = collision_event
        {
            if entity == *mob_entity {
                // deal damage to player
                for (player_entity_q, mut player_component) in player_query.iter_mut() {
                    if player_entity_q == *player_entity {
                        player_component.health.take_damage(*mob_damage);
                    }
                }
            }
        }
    }
}

/// Explode spawnable on impact
fn explode_on_impact(
    entity: Entity,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&CollisionEvent],
    spawn_effect_event_writer: &mut EventWriter<SpawnEffectEvent>,
    rb_pos: &RigidBodyPosition,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact {
                player_entity: _,
                mob_entity,
                mob_faction: _,
                player_damage: _,
                mob_damage: _,
            } => {
                // remove faction check to allow allied mobs to harm players
                if entity == *mob_entity {
                    // despawn mob
                    spawnable_component.should_despawn = true;
                    // spawn mob explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::MobExplosion,
                        position: rb_pos.position.translation.into(),
                    });
                    continue;
                }
            }
            CollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1: _,
                mob_damage_1: _,
                mob_entity_2: _,
                mob_faction_2: _,
                mob_damage_2: _,
            } => {
                if entity == *mob_entity_1 {
                    // despawn mob
                    spawnable_component.should_despawn = true;
                    // spawn mob explosion
                    spawn_effect_event_writer.send(SpawnEffectEvent {
                        effect_type: EffectType::MobExplosion,
                        position: rb_pos.position.translation.into(),
                    });
                    continue;
                }
            }
            _ => {}
        }
    }
}
