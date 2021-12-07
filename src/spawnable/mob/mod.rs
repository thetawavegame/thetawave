use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

pub mod health;

use crate::{
    game::GameParametersResource,
    spawnable::{
        spawn_projectile, CollisionEvent, Faction, Health, InitialMotion, MobType, PlayerComponent,
        ProjectileResource, ProjectileType, SpawnableBehavior, SpawnableComponent, SpawnableType,
        TextureData,
    },
    visual::AnimationComponent,
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

/// Core component for mobs
pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
    /// Mob specific behaviors
    pub behaviors: Vec<MobBehavior>,
    /// Optional mob spawn timer
    pub mob_spawn_timer: Option<Timer>,
    /// Optional weapon timer
    pub weapon_timer: Option<Timer>,
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    pub health: Health,
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

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of spawnable behaviors that are performed
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// List of mob behaviors that are performed
    pub mob_behaviors: Vec<MobBehavior>,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// Maximum speed that can be accelerated to
    pub speed: Vec2,
    /// Angular acceleration stat
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    pub angular_deceleration: f32,
    /// Maximum angular speed that can be accelerated to
    pub angular_speed: f32,
    /// Motion that the mob initializes with
    pub initial_motion: InitialMotion,
    /// Dimensions of the mob's hitbox
    pub collider_dimensions: Vec2,
    /// Texture
    pub texture: TextureData,
    /// Optional data describing the thruster
    pub thruster: Option<ThrusterData>,
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    pub health: Health,
}

/// Data describing thrusters
#[derive(Deserialize)]
pub struct ThrusterData {
    /// Y offset from center of entity
    pub y_offset: f32,
    /// Texture
    pub texture: TextureData,
}

/// Stores data about mob entities
pub struct MobsResource {
    /// Mob types mapped to mob data
    pub mobs: HashMap<MobType, MobData>,
    /// Mob types mapped to their texture and optional thruster texture
    pub texture_atlas_handle:
        HashMap<MobType, (Handle<TextureAtlas>, Option<Handle<TextureAtlas>>)>,
}

/// Spawn a mob entity
pub fn spawn_mob(
    mob_type: &MobType,
    mob_resource: &MobsResource,
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
) {
    // Get data from mob resource
    let mob_data = &mob_resource.mobs[mob_type];
    let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type].0.clone_weak();

    // scale collider to align with the sprite
    let collider_size_hx =
        mob_data.collider_dimensions.x * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy =
        mob_data.collider_dimensions.y * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    // create mob entity
    let mut mob = commands.spawn();

    mob.insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::new(
            game_parameters.sprite_scale,
            game_parameters.sprite_scale,
            1.0,
        )),
        ..Default::default()
    })
    .insert(AnimationComponent {
        timer: Timer::from_seconds(mob_data.texture.frame_duration, true),
        direction: mob_data.texture.animation_direction.clone(),
    })
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        velocity: RigidBodyVelocity {
            angvel: if let Some(random_angvel) = mob_data.initial_motion.random_angvel {
                thread_rng().gen_range(random_angvel.0..=random_angvel.1)
            } else {
                0.0
            },
            ..Default::default()
        },
        position: position.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
        material: ColliderMaterial {
            friction: 1.0,
            restitution: 1.0,
            restitution_combine_rule: CoefficientCombineRule::Max,
            ..Default::default()
        },
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(
                SPAWNABLE_COL_GROUP_MEMBERSHIP,
                u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
            ),
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(MobComponent {
        mob_type: mob_data.mob_type.clone(),
        behaviors: mob_data.mob_behaviors.clone(),
        mob_spawn_timer: None,
        weapon_timer: None,
        attack_damage: mob_data.attack_damage,
        collision_damage: mob_data.collision_damage,
        health: mob_data.health.clone(),
    })
    .insert(SpawnableComponent {
        spawnable_type: SpawnableType::Mob(mob_data.mob_type.clone()),
        acceleration: mob_data.acceleration,
        deceleration: mob_data.deceleration,
        speed: mob_data.speed,
        angular_acceleration: mob_data.angular_acceleration,
        angular_deceleration: mob_data.angular_deceleration,
        angular_speed: mob_data.angular_speed,
        behaviors: mob_data.spawnable_behaviors.clone(),
        should_despawn: false,
    })
    .insert(Name::new(mob_data.mob_type.to_string()));

    // spawn thruster as child if mob has thruster
    if let Some(thruster) = &mob_data.thruster {
        let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type]
            .1
            .as_ref()
            .unwrap()
            .clone_weak();

        mob.with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_xyz(0.0, thruster.y_offset, 0.0),
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(thruster.texture.frame_duration, true),
                    direction: thruster.texture.animation_direction.clone(),
                })
                .insert(Name::new("Thruster"));
        });
    }
}

/// Manages excuteing behaviors of mobs
pub fn mob_execute_behavior_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    time: Res<Time>,
    mob_resource: Res<MobsResource>,
    projectile_resource: Res<ProjectileResource>,
    mut mob_query: Query<(
        Entity,
        &mut SpawnableComponent,
        &mut MobComponent,
        &RigidBodyPosition,
        &RigidBodyVelocity,
    )>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
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

                            spawn_mob(
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
                        &mut player_query,
                    );
                }
                MobBehavior::DealDamageToPlayerOnImpact => {
                    deal_damage_to_player_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut mob_component,
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
                    }
                }
            }
        }
    }
}

fn receive_damage_on_impact(
    entity: Entity,
    collision_events: &[&CollisionEvent],
    mob_component: &mut MobComponent,
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact {
                player_entity,
                mob_entity,
                mob_faction,
                player_damage,
                mob_damage,
            } => {
                if entity == *mob_entity {
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if player_entity_q == *player_entity {
                            mob_component.health.take_damage(*player_damage);
                        }
                    }
                }
            }
            CollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1,
                mob_damage_1,
                mob_entity_2,
                mob_faction_2,
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

fn deal_damage_to_player_on_impact(
    entity: Entity,
    collision_events: &[&CollisionEvent],
    mob_component: &mut MobComponent,
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact {
                player_entity,
                mob_entity,
                mob_faction,
                player_damage,
                mob_damage,
            } => {
                if entity == *mob_entity {
                    // deal damage to player
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if player_entity_q == *player_entity {
                            player_component.health.take_damage(*mob_damage);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Explode spawnable on impact
fn explode_on_impact(
    entity: Entity,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&CollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact {
                player_entity,
                mob_entity,
                mob_faction,
                player_damage,
                mob_damage,
            } => {
                // remove faction check to allow allied mobs to harm players
                if entity == *mob_entity {
                    // despawn mob
                    spawnable_component.should_despawn = true;
                    // TODO: spawn explosion
                    continue;
                }
            }
            CollisionEvent::MobToMobContact {
                mob_entity_1,
                mob_faction_1,
                mob_damage_1,
                mob_entity_2,
                mob_faction_2,
                mob_damage_2,
            } => {
                if entity == *mob_entity_1 {
                    // despawn mob
                    spawnable_component.should_despawn = true;
                    // TODO: spawn explosion
                    continue;
                }
            }
            _ => {}
        }
    }
}
