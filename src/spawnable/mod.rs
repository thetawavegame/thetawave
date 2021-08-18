use serde::Deserialize;

mod formation;
mod mob;

pub use self::formation::{spawn_formation_system, SpawnerResource, SpawnerTimer};
pub use self::mob::{mob_movement_system, spawn_mob, spawn_mob_system, MobData, MobsResource};

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
