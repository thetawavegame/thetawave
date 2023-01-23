use crate::player::PlayerComponent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use strum_macros::Display;

mod behavior;
mod behavior_sequence;
mod consumable;
mod effect;
mod mob;
mod projectile;

pub use self::mob::*;
pub use self::projectile::{
    projectile_execute_behavior_system, spawn_projectile, spawn_projectile_system,
    ProjectileComponent, ProjectileData, ProjectileResource, SpawnProjectileEvent,
};

pub use self::behavior::{
    spawnable_execute_behavior_system, spawnable_set_target_behavior_system, SpawnableBehavior,
};

pub use self::behavior_sequence::{
    mob_behavior_sequence_tracker_system, mob_behavior_sequence_update_system,
    BehaviorSequenceResource, MobBehaviorUpdateEvent,
};

pub use self::effect::{
    effect_execute_behavior_system, spawn_effect, spawn_effect_system, EffectData, EffectsResource,
    SpawnEffectEvent,
};

pub use self::consumable::{
    consumable_execute_behavior_system, spawn_consumable, spawn_consumable_system,
    ConsumableComponent, ConsumableData, ConsumableResource, SpawnConsumableEvent,
};

/// Core component of spawnable entities
#[derive(Component)]
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
}

impl From<&MobData> for SpawnableComponent {
    fn from(mob_data: &MobData) -> Self {
        SpawnableComponent {
            spawnable_type: SpawnableType::Mob(mob_data.mob_type.clone()),
            acceleration: mob_data.acceleration,
            deceleration: mob_data.deceleration,
            speed: mob_data.speed,
            angular_acceleration: mob_data.angular_acceleration,
            angular_deceleration: mob_data.angular_deceleration,
            angular_speed: mob_data.angular_speed,
            behaviors: mob_data.spawnable_behaviors.clone(),
        }
    }
}

impl SpawnableComponent {
    fn new(spawnable_type: SpawnableType) -> Self {
        SpawnableComponent {
            spawnable_type,
            acceleration: Vec2::ZERO,
            deceleration: Vec2::ZERO,
            speed: Vec2::ZERO,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: vec![],
        }
    }
}

/// Initial motion that entity is spawned in with
#[derive(Deserialize, Clone, Default)]
pub struct InitialMotion {
    /// Optional angular velocity
    pub angvel: Option<f32>,
    /// Optional random range of angular velocity
    pub random_angvel: Option<(f32, f32)>,
    /// Optional linear velocity
    pub linvel: Option<Vec2>,
    /// Optional random range of linear velocity
    pub random_linvel: Option<(Vec2, Vec2)>,
}

impl From<InitialMotion> for Velocity {
    fn from(im: InitialMotion) -> Self {
        let random_linvel = if let Some((lower, upper)) = im.random_linvel {
            let x = thread_rng().gen_range(lower.x..=upper.x);
            let y = thread_rng().gen_range(lower.y..=upper.y);
            Vec2::new(x, y)
        } else {
            Vec2::ZERO
        };

        let random_angvel = if let Some((lower, upper)) = im.random_angvel {
            thread_rng().gen_range(lower..=upper)
        } else {
            0.0
        };

        Velocity {
            linvel: im.linvel.unwrap_or_default() + random_linvel,
            angvel: im.angvel.unwrap_or_default() + random_angvel,
        }
    }
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
    BarrierGlow,
    EnemyBulletDespawn,
    EnemyBulletExplosion,
    //PoisonBlastExplosion,
    //CriticalBlastExplosion,
    //MobExplosion,
    //Giblets(MobType),
}

/// Component that despawns entity after amount of time has passed

#[derive(Component)]
pub struct DespawnTimerComponent {
    despawn_timer: Timer,
}

/// Manages despawn timer components
pub fn despawn_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn_timer_query: Query<(Entity, &mut DespawnTimerComponent)>,
) {
    for (entity, mut despawn_timer) in despawn_timer_query.iter_mut() {
        despawn_timer.despawn_timer.tick(time.delta());
        if despawn_timer.despawn_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
