use std::default::Default;

use bevy_ecs::{entity::Entity, event::Event};
use bevy_ecs_macros::Component;
use bevy_math::{Quat, Vec2};
use serde::Deserialize;
use strum_macros::{Display, EnumString};

/// Type that encompasses all spawnable enemy mobs
#[derive(Deserialize, EnumString, Display, Debug, Hash, PartialEq, Eq, Clone, Copy)]
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

impl Default for SpawnableType {
    /// Money1 is default so that SpawnableComponent can derive default
    fn default() -> Self {
        SpawnableType::Consumable(ConsumableType::Money1)
    }
}

/// Type that encompasses all weapon projectiles
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ProjectileType {
    Blast(Faction),
    Bullet(Faction),
}

impl ProjectileType {
    pub fn get_faction(&self) -> Faction {
        match self {
            ProjectileType::Blast(faction) => faction.clone(),
            ProjectileType::Bullet(faction) => faction.clone(),
        }
    }
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

impl MobType {
    pub fn get_name(&self) -> String {
        match self {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => "Pawn",
                EnemyMobType::Drone => "Drone",
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => "Strafer",
                EnemyMobType::MissileLauncher => "Missile Launcher",
                EnemyMobType::Missile => "Missile",
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => "Crustling",
                EnemyMobType::Repeater => "Repeater",
                EnemyMobType::Shelly => "Shelly",
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::Hauler2 => "Hauler",
                AllyMobType::Hauler3 => "Hauler",
                AllyMobType::TutorialHauler2 => "Hauler",
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => "Money Asteroid",
                NeutralMobType::TutorialDrone => "Tutorial Drone",
            },
        }
        .to_string()
    }
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
    TutorialHauler2,
}

/// Type that encompasses all spawnable ally mob segments
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum NeutralMobSegmentType {
    HaulerBack,
    HaulerMiddle,
    TutorialHaulerBack,
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
    TutorialDrone,
}

/// Type that encompasses all spawnable consumables
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ConsumableType {
    Money1,
    Money3,
    HealthWrench,
    Armor,
    GainProjectiles,
}

/// Type that encompasses all spawnable items
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ItemType {
    EnhancedPlating,
    /*
    SteelBarrel,
    PlasmaBlasts,
    HazardousReactor,
    WarpThruster,
    Tentaclover,
    DefenseSatellite,
    DoubleBarrel,
    YithianPlague,
    Spice,
    StructureReinforcement,
    BlasterSizeEnhancer,
    FrequencyAugmentor,
    TractorBeam,
    BlastRepeller,
    */
}

/// Type that encompasses all spawnable effects
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum EffectType {
    AllyBlastExplosion,
    AllyBlastDespawn,
    #[default]
    MobExplosion, // defaults to mob explosion
    ConsumableDespawn,
    EnemyBlastExplosion,
    EnemyBlastDespawn,
    EnemyBulletExplosion,
    BarrierGlow,
    AllyBulletDespawn,
    EnemyBulletDespawn,
    AllyBulletExplosion,
    Text(TextEffectType),
}

/// Subtype of effect for text effects
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum TextEffectType {
    DamageDealt,
    ConsumableCollected(ConsumableType),
}

#[derive(Event)]
pub struct MobDestroyedEvent {
    pub mob_type: MobType,
    pub entity: Entity,
}

#[derive(Event)]
pub struct MobSegmentDestroyedEvent {
    pub mob_segment_type: MobSegmentType,
    pub entity: Entity,
}

/// Event for spawning mobs
#[derive(Event)]
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Position to spawn mob
    pub position: Vec2,

    pub rotation: Quat,

    pub boss: bool,
}

#[derive(Component)]
pub struct ItemComponent {
    pub item_type: ItemType,
}

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub position: Vec2,
}

/// Tag for applying an in-game thing to the closest player based on the player's "gravity" params.
#[derive(Component)]
pub struct AttractToClosestPlayerComponent;

#[derive(Deserialize, Clone, Debug)]
pub enum SpawnPosition {
    Global(Vec2),
    Local(Vec2),
}
