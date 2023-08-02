use serde::Deserialize;
use strum_macros::Display;

/// Type that encompasses all spawnable enemy mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum EnemyMobType {
    Pawn,
    Drone,
    StraferRight,
    StraferLeft,
    MissileLauncher,
    Missile,
    CrustlingRight,
    CrustlingLeft,
    Repeater,
    Shelly,
}

/// Type that encompasses all spawnable entities
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SpawnableType {
    Projectile(ProjectileType),
    Consumable(ConsumableType),
    Item(ItemType),
    Effect(EffectType),
    Mob(MobType),
    MobSegment(MobSegmentType),
}

/// Type that encompasses all weapon projectiles
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ProjectileType {
    Blast(Faction),
    Bullet(Faction),
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
    Enemy(EnemyMobType),
    Ally(AllyMobType),
    Neutral(NeutralMobType),
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum MobSegmentType {
    Neutral(NeutralMobSegmentType),
    Enemy(EnemyMobSegmentType),
}

/// Type that encompasses all spawnable ally mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum AllyMobType {
    Hauler2,
    Hauler3,
}

/// Type that encompasses all spawnable ally mob segments
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralMobSegmentType {
    HaulerBack,
    HaulerMiddle,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum EnemyMobSegmentType {
    CrustlingTentacle1,
    CrustlingTentacle2,
    CrustlingTentacle3,
    RepeaterBody,
    RepeaterRightShoulder,
    RepeaterLeftShoulder,
    RepeaterRightArm,
    RepeaterLeftArm,
    RepeaterRightClaw,
    RepeaterLeftClaw,
}

/// Type that encompasses all spawnable neutral mobs
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralMobType {
    MoneyAsteroid,
}

/// Type that encompasses all spawnable consumables
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ConsumableType {
    DefenseWrench,
    Money1,
    Money3,
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
    AllyBlastDespawn,
    MobExplosion,
    ConsumableDespawn,
    EnemyBlastExplosion,
    EnemyBlastDespawn,
    EnemyBulletExplosion,
    BarrierGlow,
    AllyBulletDespawn,
    EnemyBulletDespawn,
    AllyBulletExplosion,
    //PoisonBlastExplosion,
    //CriticalBlastExplosion,
    //MobExplosion,
    //Giblets(MobType),
}
