use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};
use thetawave_interface::{
    spawnable::{ProjectileType, SpawnableType},
    states::GameCleanup,
};

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::ProjectileAssets,
    game::GameParametersResource,
    spawnable::InitialMotion,
    spawnable::{SpawnableBehavior, SpawnableComponent},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

mod behavior;

pub use self::behavior::ProjectileBehavior;
use self::behavior::{
    DealDamageOnContact, DealDamageOnIntersection, ExplodeOnContact, ExplodeOnIntersection,
    TimedDespawn,
};

use super::ColliderData;

/// Event for spawning projectiles
#[derive(Event, Clone)]
pub struct SpawnProjectileEvent {
    /// Type of projectile to spawn
    pub projectile_type: ProjectileType,
    /// Position to spawn
    pub transform: Transform,
    /// Damage of projectile
    pub damage: usize,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Initial motion of the projectile
    pub initial_motion: InitialMotion,
    pub source: Entity,
}

/// Core component for projectiles
#[derive(Component)]
pub struct ProjectileComponent {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// Damage dealt to target
    pub damage: usize,
    /// Time the projectile has existed
    pub time_alive: f32,
    /// Entity that fired the projectile
    pub source: Entity,
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

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(behavior::ProjectileBehaviorPlugin);
    }
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
            event.despawn_time,
            event.initial_motion.clone(),
            &mut commands,
            &game_parameters,
            event.source,
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
    damage: usize,
    despawn_time: f32, // time before despawning
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
    source: Entity,
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
            sprite: TextureAtlasSprite {
                color: projectile_assets.get_color(projectile_type),
                ..Default::default()
            },
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
            damage,
            time_alive: 0.0,
            source,
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

    add_projectile_behavior_components(projectile_behaviors, projectile);
}

fn add_projectile_behavior_components(
    projectile_behaviors: Vec<ProjectileBehavior>,
    mut projectile: bevy::ecs::system::EntityCommands<'_, '_, '_>,
) {
    for behavior in projectile_behaviors.iter() {
        match behavior {
            ProjectileBehavior::ExplodeOnContact => projectile.insert(ExplodeOnContact),
            ProjectileBehavior::DealDamageOnIntersection => {
                projectile.insert(DealDamageOnIntersection)
            }
            ProjectileBehavior::DealDamageOnContact => projectile.insert(DealDamageOnContact),
            ProjectileBehavior::TimedDespawn { despawn_time } => projectile.insert(TimedDespawn(
                Timer::from_seconds(*despawn_time, TimerMode::Once),
            )),
            ProjectileBehavior::ExplodeOnIntersection => projectile.insert(ExplodeOnIntersection),
        };
    }
}
