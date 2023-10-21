use bevy::{
    math::Vec3Swizzles,
    prelude::{
        in_state, BuildChildren, Commands, Entity, EventReader, IntoSystemConfigs, Name, Plugin,
        Res, Transform, Update, Vec2,
    },
    sprite::{SpriteSheetBundle, TextureAtlasSprite},
    time::{Timer, TimerMode},
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Group, LockedAxes, RigidBody, Sensor, Velocity,
};
use thetawave_interface::{
    spawnable::{ProjectileType, SpawnableType},
    states::{AppStates, GameCleanup, GameStates},
};

use crate::{
    animation::AnimationComponent,
    assets::ProjectileAssets,
    game::GameParametersResource,
    spawnable::{InitialMotion, SpawnableComponent},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

use super::{
    behavior::{
        DealDamageOnContact, DealDamageOnIntersection, ExplodeOnContact, ExplodeOnIntersection,
        FollowSource, TimedDespawn,
    },
    ProjectileBehavior, ProjectileComponent, ProjectileResource, SpawnProjectileEvent,
};

pub struct SpawnProjectilePlugin;

impl Plugin for SpawnProjectilePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            spawn_projectile_system
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );
    }
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

    if matches!(projectile_type, ProjectileType::Beam(..)) {
        projectile_transform.scale.y *= 200.0;
        projectile_transform.translation.y += projectile_transform.scale.y / (2.0);
    }

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
        .insert(if matches!(projectile_type, ProjectileType::Beam(..)) {
            Velocity::default()
        } else {
            Velocity::from(initial_motion)
        })
        .insert(projectile_transform)
        .insert(Collider::cuboid(
            projectile_data.collider.dimensions.x / 2.0,
            projectile_data.collider.dimensions.y / 2.0,
        ))
        .insert(ProjectileComponent {
            projectile_type: projectile_data.projectile_type.clone(),
            damage,
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
            ProjectileBehavior::FollowSource => projectile.insert(FollowSource {
                source,
                pos_vec: projectile_transform.translation.xy() - transform.translation.xy(),
            }),
        };
    }
}
