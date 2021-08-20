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
pub fn spawnable_movement_system(
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(&SpawnableComponent, &mut RigidBodyVelocity)>,
) {
    for (spawnable_component, mut rb_vel) in spawnable_query.iter_mut() {
        match spawnable_component.spawnable_type {
            SpawnableType::Mob(MobType::Ally(AllyType::Hauler))
            | SpawnableType::Mob(MobType::Enemy(EnemyType::Drone))
            | SpawnableType::Mob(MobType::Enemy(EnemyType::Pawn))
            | SpawnableType::Mob(MobType::Enemy(EnemyType::MissileLauncher)) => {
                move_down(
                    &rapier_config,
                    &game_parameters,
                    &spawnable_component,
                    &mut rb_vel,
                );
            }
            _ => {}
        }
    }
}

fn move_down(
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    //move down
    if rb_vel.linvel.y > spawnable_component.speed.y * rapier_config.scale * -1.0 {
        rb_vel.linvel.y -= spawnable_component.acceleration.y * rapier_config.scale;
    } else {
        rb_vel.linvel.y += spawnable_component.deceleration.y * rapier_config.scale;
    }

    // decelerate in x direction
    if rb_vel.linvel.x > game_parameters.stop_threshold {
        rb_vel.linvel.x -= spawnable_component.deceleration.x;
    } else if rb_vel.linvel.x < game_parameters.stop_threshold * -1.0 {
        rb_vel.linvel.x += spawnable_component.deceleration.x;
    } else {
        rb_vel.linvel.x = 0.0;
    }
}
