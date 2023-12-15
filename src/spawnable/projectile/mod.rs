use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};
use thetawave_interface::{
    spawnable::{Faction, ProjectileType, SpawnableType},
    states::GameCleanup,
};

use crate::collision::{
    ALLY_PROJECTILE_COLLIDER_GROUP, ENEMY_PROJECTILE_COLLIDER_GROUP,
    HORIZONTAL_BARRIER_COLLIDER_GROUP, NEUTRAL_PROJECTILE_COLLIDER_GROUP, SPAWNABLE_COLLIDER_GROUP,
};
use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::ProjectileAssets,
    game::GameParametersResource,
    spawnable::InitialMotion,
    spawnable::{SpawnableBehavior, SpawnableComponent},
};

mod behavior;

pub use self::behavior::{projectile_execute_behavior_system, ProjectileBehavior};

use super::ColliderData;

/// Event for spawning projectiles
#[derive(Event, Clone)]
pub struct SpawnProjectileEvent {
    /// Type of projectile to spawn
    pub projectile_type: ProjectileType,
    pub projectile_count: usize,
    pub projectile_direction: f32,
    /// Position to spawn
    pub transform: Transform,
    /// Damage of projectile
    pub damage: usize,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Initial motion of the projectile
    pub initial_motion: InitialMotion,
    pub source: Entity,
    pub speed: f32,
}

/// Core component for projectiles
#[derive(Component)]
pub struct ProjectileComponent {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// Projectile specific behaviors
    pub behaviors: Vec<ProjectileBehavior>,
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
    for event in event_reader.read() {
        spawn_projectile(
            &event.projectile_type,
            &projectile_resource,
            &projectile_assets,
            event.transform,
            event.projectile_count,
            event.damage,
            event.projectile_direction,
            event.speed,
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
    projectile_count: usize,
    damage: usize,
    direction: f32,
    speed: f32,
    despawn_time: f32, // time before despawning
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
    source: Entity,
) {
    // Get data from projectile resource
    let projectile_data = &projectile_resource.projectiles[projectile_type];

    let mut projectile_behaviors = projectile_data.projectile_behaviors.clone();
    projectile_behaviors.push(ProjectileBehavior::TimedDespawn { despawn_time });

    let mut projectile_transform = transform;
    projectile_transform.translation.z = projectile_data.z_level;
    projectile_transform.scale.x *= game_parameters.sprite_scale;
    projectile_transform.scale.y *= game_parameters.sprite_scale;
    projectile_transform.scale.z = 1.0;

    // the percentage of the total number of projectiles that the player has acquired
    let total_projectiles_percent =
        (projectile_count as f32 - 1.) / (game_parameters.max_player_projectiles - 1.);
    // indicates the angle between the first and last projectile
    let spread_arc = game_parameters
        .max_spread_arc
        .min(total_projectiles_percent * game_parameters.projectile_gap);
    // indicates the angle between each projectile
    let spread_angle_segment = spread_arc / (projectile_count as f32 - 1.).max(1.);

    for p in 0..projectile_count {
        let new_initial_motion =
            if let Some(mut initial_motion_linvel) = initial_motion.clone().linvel {
                // Calculate the angle for the current projectile.
                // The first projectile is spread_angle_segment/2 radians to the left of the direction,
                // and the last projectile is spread_angle_segment/2 radians to the right.
                let angle_offset =
                    (p as f32 - (projectile_count as f32 - 1.) / 2.) * spread_angle_segment;
                let projectile_angle = direction + angle_offset;

                let weights = Vec2::new(1.0, 2.0); //TODO: move to a weapon struct

                // Convert the angle to a velocity vector
                initial_motion_linvel += Vec2::from_angle(projectile_angle) * speed * weights;

                InitialMotion {
                    linvel: Some(initial_motion_linvel),
                    ..initial_motion.clone()
                }
            } else {
                initial_motion.clone()
            };

        // create projectile entity
        let mut projectile = commands.spawn_empty();
        let projectile_colider_group = get_projectile_collider_group(projectile_type.get_faction());

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
            .insert(Velocity::from(new_initial_motion))
            .insert(projectile_transform)
            .insert(Collider::cuboid(
                projectile_data.collider.dimensions.x,
                projectile_data.collider.dimensions.y,
            ))
            .insert(ProjectileComponent {
                projectile_type: projectile_data.projectile_type.clone(),
                behaviors: projectile_behaviors.clone(),
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
                memberships: SPAWNABLE_COLLIDER_GROUP | projectile_colider_group,
                filters: Group::ALL
                    ^ (HORIZONTAL_BARRIER_COLLIDER_GROUP
                        | SPAWNABLE_COLLIDER_GROUP
                        | projectile_colider_group),
            })
            .insert(GameCleanup)
            .insert(Name::new(projectile_data.projectile_type.to_string()));

        if !projectile_data.is_solid {
            projectile.insert(Sensor);
        }
    }
}

fn get_projectile_collider_group(faction: Faction) -> Group {
    match faction {
        Faction::Ally => ALLY_PROJECTILE_COLLIDER_GROUP,
        Faction::Enemy => ENEMY_PROJECTILE_COLLIDER_GROUP,
        Faction::Neutral => NEUTRAL_PROJECTILE_COLLIDER_GROUP,
    }
}
