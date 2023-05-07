use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::ProjectileAssets,
    game::GameParametersResource,
    misc::Health,
    spawnable::InitialMotion,
    spawnable::{ProjectileType, SpawnableBehavior, SpawnableComponent, SpawnableType},
    states::GameCleanup,
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

mod behavior;

pub use self::behavior::{projectile_execute_behavior_system, ProjectileBehavior};

use super::ColliderData;

/// Event for spawning projectiles
pub struct SpawnProjectileEvent {
    /// Type of projectile to spawn
    pub projectile_type: ProjectileType,
    /// Position to spawn
    pub transform: Transform,
    /// Damage of projectile
    pub damage: f32,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Initial motion of the projectile
    pub initial_motion: InitialMotion,
    /// Optional health of the projectile
    pub health: Option<Health>,
}

/// Core component for projectiles
#[derive(Component)]
pub struct ProjectileComponent {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// Projectile specific behaviors
    pub behaviors: Vec<ProjectileBehavior>,
    /// Damage dealt to target
    pub damage: f32,
    /// Time the projectile has existed
    pub time_alive: f32,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct ProjectileData {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// List of spawnable behaviors that are performed
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// List of projectile behaviors that are performed
    pub projectile_behaviors: Vec<ProjectileBehavior>,
    /// Animation (currently loops single animation in specified direction)
    pub animation: AnimationData,
    /// Z level of transform of projectile
    pub z_level: f32,
    /// Collider
    pub collider: ColliderData,
    /// If it has a contact collider
    pub is_solid: bool,
}

/// Stores data about mob entities
#[derive(Resource)]
pub struct ProjectileResource {
    /// Projectile types mapped to projectile data
    pub projectiles: HashMap<ProjectileType, ProjectileData>,
}

/// Spawns projectiles from events
pub fn spawn_projectile_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnProjectileEvent>,
    projectile_resource: Res<ProjectileResource>,
    projectile_assets: Res<ProjectileAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_projectile(
            &event.projectile_type,
            &projectile_resource,
            &projectile_assets,
            event.transform,
            event.damage,
            event.health.clone(),
            event.despawn_time,
            event.initial_motion.clone(),
            &mut commands,
            &game_parameters,
        );
    }
}

/// Spawn a projectile entity
#[allow(clippy::too_many_arguments)]
pub fn spawn_projectile(
    projectile_type: &ProjectileType,
    projectile_resource: &ProjectileResource,
    projectile_assets: &ProjectileAssets,
    transform: Transform,
    damage: f32,
    health: Option<Health>,
    despawn_time: f32, // time before despawning
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from projectile resource
    let projectile_data = &projectile_resource.projectiles[projectile_type];

    // create projectile entity
    let mut projectile = commands.spawn_empty();

    let mut projectile_behaviors = projectile_data.projectile_behaviors.clone();
    projectile_behaviors.push(ProjectileBehavior::TimedDespawn { despawn_time });

    let mut projectile_transform = transform;
    projectile_transform.translation.z = projectile_data.z_level;
    projectile_transform.scale.x *= game_parameters.sprite_scale;
    projectile_transform.scale.y *= game_parameters.sprite_scale;
    projectile_transform.scale.z = 1.0;

    projectile
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(SpriteSheetBundle {
            texture_atlas: projectile_assets.get_asset(projectile_type),
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(
                projectile_data.animation.frame_duration,
                TimerMode::Repeating,
            ),
            direction: projectile_data.animation.direction.clone(),
        })
        .insert(RigidBody::Dynamic)
        .insert(Velocity::from(initial_motion))
        .insert(projectile_transform)
        .insert(Collider::cuboid(
            projectile_data.collider.dimensions.x,
            projectile_data.collider.dimensions.y,
        ))
        .insert(ProjectileComponent {
            projectile_type: projectile_data.projectile_type.clone(),
            behaviors: projectile_behaviors,
            damage,
            time_alive: 0.0,
        })
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Projectile(projectile_data.projectile_type.clone()),
            acceleration: Vec2::ZERO,
            deceleration: Vec2::ZERO,
            speed: [game_parameters.max_speed, game_parameters.max_speed].into(), // highest possible speed
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: game_parameters.max_speed,
            behaviors: projectile_data.spawnable_behaviors.clone(),
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(CollisionGroups {
            memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
            filters: Group::ALL ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
        })
        .insert(GameCleanup)
        .insert(Name::new(projectile_data.projectile_type.to_string()));

    if !projectile_data.is_solid {
        projectile.insert(Sensor);
    }
}
