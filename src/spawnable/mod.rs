use crate::{
    game::GameParametersResource, player::PlayerComponent, tools::signed_modulo,
    visual::AnimationDirection,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

mod mob;
mod projectile;
mod spawner;

pub use self::mob::{
    mob_execute_behavior_system, spawn_mob, MobBehavior, MobComponent, MobData, MobsResource,
};
pub use self::projectile::{
    projectile_execute_behavior_system, spawn_projectile, ProjectileData, ProjectileResource,
};
pub use self::spawner::{spawner_system, SpawnerResource, SpawnerResourceData};

pub struct SpawnableComponent {
    /// Type of spawnable
    pub spawnable_type: SpawnableType,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// Maximum speed stat
    pub speed: Vec2,
    /// Angular acceleration stat
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    pub angular_deceleration: f32,
    /// Maximum angular speed stat
    pub angular_speed: f32,
    /// List of behaviors that are performed
    pub behaviors: Vec<SpawnableBehavior>,
    /// Flag to despawn next frame
    pub should_despawn: bool,
}

/// Data describing texture
#[derive(Deserialize)]
pub struct TextureData {
    /// Path to the texture
    pub path: String,
    /// Dimensions of the texture (single frame)
    pub dimensions: Vec2,
    /// Columns in the spritesheet
    pub cols: usize,
    /// Rows in the spritesheet
    pub rows: usize,
    /// Duration of a frame of animation
    pub frame_duration: f32,
    /// How the animation switches frames
    pub animation_direction: AnimationDirection,
}

/// Initial motion that entity is spawned in with
#[derive(Deserialize, Clone)]
pub struct InitialMotion {
    /// Optional random range of angular velocity
    pub random_angvel: Option<(f32, f32)>,
    /// Optional linear velocity
    pub linvel: Option<Vec2>,
}

/// Types of behaviors that can be performed by spawnables
#[derive(Deserialize, Clone)]
pub enum SpawnableBehavior {
    RotateToTarget(Option<Vec2>),
    MoveForward,
    MoveDown,
    MoveRight,
    MoveLeft,
    BrakeHorizontal,
}

/// Type that encompasses all spawnable entities
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Projectile(ProjectileType),
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

/// Type that encompasses all weapon projectiles
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ProjectileType {
    Blast(Faction),
}

/// Factions
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum Faction {
    Ally,
    Enemy,
    Neutral,
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

/// Manages excuting behaviors of spawnables
pub fn spawnable_execute_behavior_system(
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(&SpawnableComponent, &mut RigidBodyVelocity, &Transform)>,
) {
    // Iterate through all spawnable entities and execute their behavior
    for (spawnable_component, mut rb_vel, spawnable_transform) in spawnable_query.iter_mut() {
        let behaviors = spawnable_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                SpawnableBehavior::MoveDown => {
                    move_down(&rapier_config, spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveRight => {
                    move_right(&rapier_config, spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveLeft => {
                    move_left(&rapier_config, spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::RotateToTarget(target_position) => {
                    rotate_to_target(
                        spawnable_transform,
                        target_position.unwrap(),
                        spawnable_component,
                        &mut rb_vel,
                    );
                }
                SpawnableBehavior::MoveForward => {
                    move_forward(
                        &rapier_config,
                        spawnable_transform,
                        spawnable_component,
                        &mut rb_vel,
                    );
                }
                SpawnableBehavior::BrakeHorizontal => {
                    brake_horizontal(
                        &rapier_config,
                        &game_parameters,
                        spawnable_component,
                        &mut rb_vel,
                    );
                }
            }
        }
    }
}

/// Despawn spawnables that are flagged with 'should_despawn'
pub fn despawn_spawnable_system(
    mut commands: Commands,
    spawnable_query: Query<(Entity, &SpawnableComponent)>,
) {
    for (entity, spawnable_component) in spawnable_query.iter() {
        if spawnable_component.should_despawn {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Manages setting targeting of spawnables
pub fn spawnable_set_target_behavior_system(
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut spawnable_query: Query<(&mut SpawnableComponent, &Transform)>,
) {
    // Sets targetting to None
    for (mut spawnable_component, _) in spawnable_query.iter_mut() {
        for behavior in spawnable_component.behaviors.iter_mut() {
            if let SpawnableBehavior::RotateToTarget(_) = behavior {
                *behavior = SpawnableBehavior::RotateToTarget(None);
            }
        }
    }

    // Recalculates what the target should be
    for player_transform in player_query.iter() {
        for (mut spawnable_component, spawnable_transform) in spawnable_query.iter_mut() {
            match &spawnable_component.spawnable_type {
                SpawnableType::Mob(mob_type) => match mob_type {
                    MobType::Enemy(enemy_type) => match enemy_type {
                        EnemyType::Missile => {
                            // set target to closest player
                            for behavior in spawnable_component.behaviors.iter_mut() {
                                *behavior = match behavior {
                                    SpawnableBehavior::RotateToTarget(target) => {
                                        let spawnable_position_vec2: Vec2 =
                                            spawnable_transform.translation.into();
                                        let player_position_vec2: Vec2 =
                                            player_transform.translation.into();
                                        if target.is_none()
                                            || spawnable_position_vec2
                                                .distance(player_position_vec2)
                                                < spawnable_position_vec2.distance(target.unwrap())
                                        {
                                            SpawnableBehavior::RotateToTarget(Some(
                                                player_position_vec2,
                                            ))
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

/// Manages setting behaviors due to contact events
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
                                            SpawnableBehavior::MoveRight => {
                                                SpawnableBehavior::MoveLeft
                                            }
                                            SpawnableBehavior::MoveLeft => {
                                                SpawnableBehavior::MoveRight
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
    }
}

/// Rotates entity to face target
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

/// Move entity forward along it's axis
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

/// Moves entity down
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

/// Moves entity right
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

/// Moves entity left
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

/// Decelerates to 0 horizontal movement
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
