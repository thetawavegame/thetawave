use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    animation::{AnimationComponent, AnimationData, AnimationDirection, TextureData},
    assets::ProjectileAssets,
    game::GameParametersResource,
    spawnable::InitialMotion,
    spawnable::{ProjectileType, SpawnableBehavior, SpawnableComponent, SpawnableType},
    states::{AppStateComponent, AppStates},
};

mod behavior;

pub use self::behavior::{projectile_execute_behavior_system, ProjectileBehavior};

/// Event for spawning projectiles
pub struct SpawnProjectileEvent {
    /// Type of projectile to spawn
    pub projectile_type: ProjectileType,
    /// Position to spawn
    pub position: Vec2,
    /// Damage of projectile
    pub damage: f32,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Initial motion of the projectile
    pub initial_motion: InitialMotion,
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
    /// Dimensions of the projectile's hitbox
    pub collider_dimensions: Vec2,
    /// Animation (currently loops single animation in specified direction)
    pub animation: AnimationData,
    /// Z level of transform of projectile
    pub z_level: f32,
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
            event.position,
            event.damage,
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
    position: Vec2,
    damage: f32,
    despawn_time: f32, // time before despawning
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from projectile resource
    let projectile_data = &projectile_resource.projectiles[projectile_type];

    // scale collider to align with the sprite
    let collider_size_hx =
        projectile_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy =
        projectile_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // create projectile entity
    let mut projectile = commands.spawn_empty();

    let mut projectile_behaviors = projectile_data.projectile_behaviors.clone();
    projectile_behaviors.push(ProjectileBehavior::TimedDespawn {
        despawn_time,
        current_time: 0.0,
    });

    projectile
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
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity {
            angvel: if let Some(random_angvel) = initial_motion.random_angvel {
                thread_rng().gen_range(random_angvel.0..=random_angvel.1)
            } else {
                0.0
            },
            linvel: if let Some(linvel) = initial_motion.linvel {
                linvel
            } else {
                Vec2::ZERO
            },
        })
        .insert(Transform {
            translation: position.extend(projectile_data.z_level),
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Sensor)
        .insert(ProjectileComponent {
            projectile_type: projectile_data.projectile_type.clone(),
            behaviors: projectile_behaviors,
            damage,
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new(projectile_data.projectile_type.to_string()));
}
