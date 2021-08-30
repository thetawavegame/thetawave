use crate::game::GameParametersResource;
use crate::player::PlayerComponent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

mod formation;
mod mob;

pub use self::formation::{spawn_formation_system, SpawnerResource, SpawnerTimer};
pub use self::mob::{spawn_mob, MobComponent, MobData, MobsResource};

pub struct SpawnableComponent {
    pub spawnable_type: SpawnableType,
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub angular_speed: f32,
    pub behaviors: Vec<BehaviorType>,
    pub should_despawn: bool,
}

#[derive(Deserialize, Clone)]
pub enum BehaviorType {
    RotateToTarget(Option<Vec2>),
    MoveForward,
    MoveDown,
    MoveRight,
    MoveLeft,
    BrakeHorizontal,
    ExplodeOnImpact,
}

/// Type that encompasses all spawnable entities
// TODO: add projectiles (blast)
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

/// Type that encompasses all spawnable mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobType {
    Enemy(EnemyType),
    Ally(AllyType),
    Neutral(NeutralType),
}

/// Type that encompasses all spawnable enemy mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum EnemyType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    MissileLauncher,
    Missile,
    RepeaterBody,
    RepeaterHead,
    RepeaterLeftShoulder,
    RepeaterRightShoulder,
    RepeaterLeftArm,
    RepeaterRightArm,
}

/// Type that encompasses all spawnable ally mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum AllyType {
    Hauler,
}

/// Type that encompasses all spawnable neutral mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralType {
    MoneyAsteroid,
}

/// Type that encompasses all spawnable consumables
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

/// Type that encompasses all spawnable items
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ItemType {
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    EnhancedPlating,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
    TractorBeam,
    BlastRepeller,
}

/// Type that encompasses all spawnable effects
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    MobExplosion,
    Giblets(MobType),
}
pub fn spawnable_execute_behavior_system(
    mut contact_events: EventReader<ContactEvent>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(
        Entity,
        &mut SpawnableComponent,
        &mut RigidBodyVelocity,
        &Transform,
    )>,
) {
    let mut contact_events_vec = vec![];
    for contact_event in contact_events.iter() {
        contact_events_vec.push(*contact_event);
    }
    for (entity, mut spawnable_component, mut rb_vel, spawnable_transform) in
        spawnable_query.iter_mut()
    {
        let behaviors = spawnable_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                BehaviorType::MoveDown => {
                    move_down(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                BehaviorType::MoveRight => {
                    move_right(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                BehaviorType::MoveLeft => {
                    move_left(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                BehaviorType::RotateToTarget(target_position) => {
                    rotate_to_target(
                        spawnable_transform,
                        target_position.unwrap(),
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                BehaviorType::MoveForward => {
                    move_forward(
                        &rapier_config,
                        spawnable_transform,
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                BehaviorType::BrakeHorizontal => {
                    brake_horizontal(
                        &rapier_config,
                        &game_parameters,
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                BehaviorType::ExplodeOnImpact => {
                    explode_on_impact(entity, &mut spawnable_component, &contact_events_vec);
                }
            }
        }
    }
}

fn explode_on_impact(
    entity: Entity,
    spawnable_component: &mut SpawnableComponent,
    contact_events: &[ContactEvent],
) {
    for contact_event in contact_events {
        //checks for collision between spawnable and other
        if let ContactEvent::Stopped(h1, h2) = contact_event {
            if h1.entity() == entity || h2.entity() == entity {
                spawnable_component.should_despawn = true;
            }
        }
    }
}

pub fn despawn_spawnable_system(
    mut commands: Commands,
    spawnable_query: Query<(Entity, &SpawnableComponent)>,
) {
    for (entity, spawnable_component) in spawnable_query.iter() {
        if spawnable_component.should_despawn {
            commands.entity(entity).despawn();
        }
    }
}

pub fn spawnable_set_target_behavior_system(
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut spawnable_query: Query<(&mut SpawnableComponent, &Transform)>,
) {
    for (mut spawnable_component, _) in spawnable_query.iter_mut() {
        for behavior in spawnable_component.behaviors.iter_mut() {
            if let BehaviorType::RotateToTarget(_) = behavior {
                *behavior = BehaviorType::RotateToTarget(None);
            }
        }
    }

    for player_transform in player_query.iter() {
        for (mut spawnable_component, spawnable_transform) in spawnable_query.iter_mut() {
            match &spawnable_component.spawnable_type {
                SpawnableType::Mob(mob_type) => match mob_type {
                    MobType::Enemy(enemy_type) => match enemy_type {
                        EnemyType::Missile => {
                            // set target to closest player
                            for behavior in spawnable_component.behaviors.iter_mut() {
                                *behavior = match behavior {
                                    BehaviorType::RotateToTarget(target) => {
                                        let spawnable_position_vec2: Vec2 =
                                            spawnable_transform.translation.into();
                                        let player_position_vec2: Vec2 =
                                            player_transform.translation.into();
                                        if target.is_none()
                                            || spawnable_position_vec2
                                                .distance(player_position_vec2)
                                                < spawnable_position_vec2.distance(target.unwrap())
                                        {
                                            BehaviorType::RotateToTarget(Some(player_position_vec2))
                                        } else {
                                            behavior.clone()
                                        }
                                    }
                                    _ => behavior.clone(),
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

pub fn spawnable_set_contact_behavior_system(
    mut contact_events: EventReader<ContactEvent>,
    mut spawnable_query: Query<(Entity, &mut SpawnableComponent)>,
) {
    // set behaviors based on contact events
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            let collider1_entity = h1.entity();
            let collider2_entity = h2.entity();
            for (spawnable_entity, mut spawnable_component) in spawnable_query.iter_mut() {
                let spawnable_entity = if spawnable_entity == collider1_entity {
                    Some(collider1_entity)
                } else if spawnable_entity == collider2_entity {
                    Some(collider2_entity)
                } else {
                    None
                };
                if spawnable_entity.is_some() {
                    match &spawnable_component.spawnable_type {
                        SpawnableType::Mob(mob_type) => match mob_type {
                            MobType::Enemy(enemy_type) => match enemy_type {
                                EnemyType::StraferRight | EnemyType::StraferLeft => {
                                    for behavior in spawnable_component.behaviors.iter_mut() {
                                        *behavior = match behavior {
                                            BehaviorType::MoveRight => BehaviorType::MoveLeft,
                                            BehaviorType::MoveLeft => BehaviorType::MoveRight,
                                            _ => behavior.clone(),
                                        }
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}

pub fn signed_modulo(a: f32, n: f32) -> f32 {
    a - (a / n).floor() * n
}

fn rotate_to_target(
    transform: &Transform,
    target_position: Vec2,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    let mut target_angle = ((transform.translation.y - target_position.y)
        .atan2(transform.translation.x - target_position.x))
    .to_degrees()
        + 90.0;

    if target_angle < 0.0 {
        target_angle += 360.0;
    }

    let current_angle = (transform.rotation.to_axis_angle().1.to_degrees()
        * transform.rotation.to_axis_angle().0.z)
        + 180.0;

    let mut smallest_angle = signed_modulo(target_angle - current_angle, 360.0);
    if smallest_angle > 180.0 {
        smallest_angle = -(360.0 - smallest_angle);
    }

    if smallest_angle < 0.0 {
        if rb_vel.angvel > -spawnable_component.angular_speed {
            rb_vel.angvel -= spawnable_component.angular_acceleration;
        }
    } else if rb_vel.angvel < spawnable_component.angular_speed {
        rb_vel.angvel += spawnable_component.angular_acceleration;
    }
}

fn move_forward(
    rapier_config: &RapierConfiguration,
    transform: &Transform,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    let angle = (transform.rotation.to_axis_angle().1 * transform.rotation.to_axis_angle().0.z)
        - (std::f32::consts::FRAC_PI_2);

    let max_speed_x = (spawnable_component.speed.x * angle.cos() * rapier_config.scale).abs();
    let max_speed_y = (spawnable_component.speed.y * angle.sin() * rapier_config.scale).abs();

    if rb_vel.linvel.x > max_speed_x {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    } else if rb_vel.linvel.x < -max_speed_x {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x += spawnable_component.acceleration.x * angle.cos() * rapier_config.scale;
    }

    if rb_vel.linvel.y > max_speed_y {
        rb_vel.linvel.y -= spawnable_component.deceleration.y * rapier_config.scale;
    } else if rb_vel.linvel.y < -max_speed_y {
        rb_vel.linvel.y += spawnable_component.deceleration.y * rapier_config.scale;
    } else {
        rb_vel.linvel.y += spawnable_component.acceleration.x * angle.sin() * rapier_config.scale;
    }
}

fn move_down(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    //move down
    if rb_vel.linvel.y > spawnable_component.speed.y * rapier_config.scale * -1.0 {
        rb_vel.linvel.y -= spawnable_component.acceleration.y * rapier_config.scale;
    } else {
        rb_vel.linvel.y += spawnable_component.deceleration.y * rapier_config.scale;
    }
}

fn move_right(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    if rb_vel.linvel.x < spawnable_component.speed.x * rapier_config.scale {
        rb_vel.linvel.x += spawnable_component.acceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    }
}

fn move_left(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    if rb_vel.linvel.x > spawnable_component.speed.x * rapier_config.scale * -1.0 {
        rb_vel.linvel.x -= spawnable_component.acceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    }
}

fn brake_horizontal(
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    // decelerate in x direction
    if rb_vel.linvel.x > game_parameters.stop_threshold {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    } else if rb_vel.linvel.x < game_parameters.stop_threshold * -1.0 {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x = 0.0;
    }
}
