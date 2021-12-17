use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    animation::{AnimationComponent, TextureData},
    game::GameParametersResource,
    spawnable::InitialMotion,
    spawnable::{
        DespawnTimerComponent, ProjectileType, SpawnableBehavior, SpawnableComponent, SpawnableType,
    },
};

mod behavior;

pub use self::behavior::{projectile_execute_behavior_system, ProjectileBehavior};

/// Core component for projectiles
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
    /// Texture
    pub texture: TextureData,
}

/// Stores data about mob entities
pub struct ProjectileResource {
    /// Projectile types mapped to projectile data
    pub projectiles: HashMap<ProjectileType, ProjectileData>,
    /// Mob types mapped to their texture and optional thruster texture
    pub texture_atlas_handle: HashMap<ProjectileType, Handle<TextureAtlas>>,
}

#[allow(clippy::too_many_arguments)]
/// Spawn a mob entity
pub fn spawn_projectile(
    projectile_type: &ProjectileType,
    projectile_resource: &ProjectileResource,
    position: Vec2,
    damage: f32,
    despawn_time: f32, // time before despawning
    initial_motion: InitialMotion,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
) {
    // Get data from mob resource
    let projectile_data = &projectile_resource.projectiles[projectile_type];
    let texture_atlas_handle =
        projectile_resource.texture_atlas_handle[projectile_type].clone_weak();

    // scale collider to align with the sprite
    let collider_size_hx = projectile_data.collider_dimensions.x * game_parameters.sprite_scale
        / rapier_config.scale
        / 2.0;
    let collider_size_hy = projectile_data.collider_dimensions.y * game_parameters.sprite_scale
        / rapier_config.scale
        / 2.0;

    // create mob entity
    let mut projectile = commands.spawn();

    let mut projectile_behaviors = projectile_data.projectile_behaviors.clone();
    projectile_behaviors.push(ProjectileBehavior::TimedDespawn {
        despawn_time,
        current_time: 0.0,
    });

    projectile
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            )),
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(projectile_data.texture.frame_duration, true),
            direction: projectile_data.texture.animation_direction.clone(),
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            velocity: RigidBodyVelocity {
                angvel: if let Some(random_angvel) = initial_motion.random_angvel {
                    thread_rng().gen_range(random_angvel.0..=random_angvel.1)
                } else {
                    0.0
                },
                linvel: if let Some(linvel) = initial_motion.linvel {
                    linvel.into()
                } else {
                    Vec2::ZERO.into()
                },
            },
            position: position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
            collider_type: ColliderType::Sensor,
            flags: ColliderFlags {
                // TODO: filter out others of same faction
                //collision_groups: InteractionGroups::new(PROJECTILE_GROUP_MEMBERSHIP, filter)
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
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
            should_despawn: false,
        })
        /*
        .insert(DespawnTimerComponent {
            despawn_timer: Timer::from_seconds(despawn_time, false),
        })
        */
        .insert(Name::new(projectile_data.projectile_type.to_string()));
}
