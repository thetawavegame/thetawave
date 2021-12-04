use crate::{
    player::PlayerComponent, spawnable::projectile::ProjectileComponent, visual::AnimationDirection,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

mod behavior;
mod mob;
mod projectile;
mod spawn;

pub use self::mob::{
    health::Health, mob_execute_behavior_system, spawn_mob, MobBehavior, MobComponent, MobData,
    MobsResource,
};
pub use self::projectile::{
    projectile_execute_behavior_system, spawn_projectile, ProjectileData, ProjectileResource,
};
pub use self::spawn::{spawner_system, SpawnerResource, SpawnerResourceData};

pub use self::behavior::{
    spawnable_execute_behavior_system, spawnable_set_contact_behavior_system,
    spawnable_set_target_behavior_system, SpawnableBehavior,
};

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

/// Component that despawns entity after amount of time has passed
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

// Manages damage dealing on collision
pub fn collision_damage_system(
    mut intersection_events: EventReader<IntersectionEvent>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    projectile_query: Query<(Entity, &ProjectileComponent)>,
) {
    for intersection_event in intersection_events.iter() {
        let mut colliding_entities = vec![];
        let collider1_entity = intersection_event.collider1.entity();
        let collider2_entity = intersection_event.collider2.entity();
        colliding_entities.push(collider1_entity);
        colliding_entities.push(collider2_entity);

        // check if player was collided with
        for (player_entity, mut player_component) in player_query.iter_mut() {
            if player_entity == collider1_entity {
                for (projectile_entity, projectile_component) in projectile_query.iter() {
                    if projectile_entity == collider2_entity
                        && (match projectile_component.projectile_type.clone() {
                            ProjectileType::Blast(faction) => match faction {
                                Faction::Ally => false,
                                Faction::Enemy => true,
                                Faction::Neutral => true,
                            },
                        })
                    {
                        player_component
                            .health
                            .take_damage(projectile_component.damage);
                    }
                }
            } else if player_entity == collider2_entity {
                for (projectile_entity, projectile_component) in projectile_query.iter() {
                    if projectile_entity == collider1_entity
                        && (match projectile_component.projectile_type.clone() {
                            ProjectileType::Blast(faction) => match faction {
                                Faction::Ally => false,
                                Faction::Enemy => true,
                                Faction::Neutral => true,
                            },
                        })
                    {
                        player_component
                            .health
                            .take_damage(projectile_component.damage);
                    }
                }
            }
        }
    }
}
