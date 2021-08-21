use crate::game::GameParametersResource;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

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
    pub behaviors: Vec<BehaviorType>,
}

#[derive(Deserialize, Clone)]
pub enum BehaviorType {
    Move(BehaviorDirection),
    Brake(BehaviorDirection),
}

#[derive(Deserialize, Clone)]
pub enum BehaviorDirection {
    Up,
    Down,
    Right,
    Left,
    Horizontal,
    Vertical,
}

/// Type that encompasses all spawnable entities
// TODO: add projectiles (blast)
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SpawnableType {
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
}

/// Type that encompasses all spawnable mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MobType {
    Enemy(EnemyType),
    Ally(AllyType),
    Neutral(NeutralType),
}

/// Type that encompasses all spawnable enemy mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
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
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AllyType {
    Hauler,
}

/// Type that encompasses all spawnable neutral mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum NeutralType {
    MoneyAsteroid,
}

/// Type that encompasses all spawnable consumables
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money5,
    HealthWrench,
    Armor,
}

/// Type that encompasses all spawnable items
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
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
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum EffectType {
    AllyBlastExplosion,
    EnemyBlastExplosion,
    PoisonBlastExplosion,
    CriticalBlastExplosion,
    MobExplosion,
    Giblets(MobType),
}
pub fn spawnable_execute_behavior_system(
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(&SpawnableComponent, &mut RigidBodyVelocity)>,
) {
    for (spawnable_component, mut rb_vel) in spawnable_query.iter_mut() {
        for behavior in spawnable_component.behaviors.iter() {
            match behavior {
                BehaviorType::Move(behavior_direction) => match behavior_direction {
                    BehaviorDirection::Down => {
                        move_down(&rapier_config, &spawnable_component, &mut rb_vel);
                    }
                    BehaviorDirection::Right => {
                        move_right(&rapier_config, &spawnable_component, &mut rb_vel);
                    }
                    BehaviorDirection::Left => {
                        move_left(&rapier_config, &spawnable_component, &mut rb_vel);
                    }
                    _ => {}
                },
                BehaviorType::Brake(behavior_direction) => match behavior_direction {
                    BehaviorDirection::Horizontal => {
                        brake_horizontal(
                            &rapier_config,
                            &game_parameters,
                            &spawnable_component,
                            &mut rb_vel,
                        );
                    }
                    _ => {}
                },
            }
        }
    }
}

pub fn spawnable_set_behavior_system(
    mut contact_events: EventReader<ContactEvent>,
    mut mob_query: Query<(Entity, &mut SpawnableComponent)>,
) {
    for contact_event in contact_events.iter() {
        if let ContactEvent::Started(h1, h2) = contact_event {
            let collider1_entity = h1.entity();
            let collider2_entity = h2.entity();
            for (spawnable_entity, mut spawnable_component) in mob_query.iter_mut() {
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
                                            BehaviorType::Move(BehaviorDirection::Right) => {
                                                BehaviorType::Move(BehaviorDirection::Left)
                                            }
                                            BehaviorType::Move(BehaviorDirection::Left) => {
                                                BehaviorType::Move(BehaviorDirection::Right)
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
                    /*
                    match mob_component.mob_type {
                        MobType::Enemy(EnemyType::StraferRight)
                        | MobType::Enemy(EnemyType::StraferLeft) => {
                            for behavior in spawnable_component.behaviors.iter_mut() {
                                *behavior = match behavior {
                                    BehaviorType::Move(BehaviorDirection::Right) => {
                                        BehaviorType::Move(BehaviorDirection::Left)
                                    }
                                    BehaviorType::Move(BehaviorDirection::Left) => {
                                        BehaviorType::Move(BehaviorDirection::Right)
                                    }
                                    _ => behavior.clone(),
                                }
                            }
                        }
                        _ => {}
                    }
                    */
                }

                /*
                        match &spawnable_component.spawnable_type {
                            SpawnableType::Mob(mob_type) => match mob_type {
                                MobType::Enemy(enemy_type) => match enemy_type {
                                    EnemyType::StraferRight | EnemyType::StraferLeft => {
                                        println!("strafer behavior");
                                        for behavior in spawnable_component.behaviors.iter_mut() {
                                            *behavior = match behavior {
                                                BehaviorType::Move(BehaviorDirection::Right) => {
                                                    BehaviorType::Move(BehaviorDirection::Left)
                                                }
                                                BehaviorType::Move(BehaviorDirection::Left) => {
                                                    BehaviorType::Move(BehaviorDirection::Right)
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
                */
            }
        }
        //println!("{:?}", contact_event);
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
