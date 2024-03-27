use bevy::{
    prelude::{
        Commands, Component, Entity, Event, EventReader, EventWriter, Name, Quat, Res, Resource,
        Sprite, SpriteSheetBundle, Timer, TimerMode, Transform, Vec2, Vec3Swizzles,
    },
    render::color::Color,
};
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionGroups, Group, LockedAxes, RigidBody, Sensor, Velocity,
};
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::{
    audio::PlaySoundEffectEvent,
    game::options::GameOptions,
    spawnable::{Faction, ProjectileType, SpawnableType},
    states::GameCleanup,
    weapon::WeaponProjectileData,
};

use crate::collision::{
    ALLY_PROJECTILE_COLLIDER_GROUP, ENEMY_PROJECTILE_COLLIDER_GROUP,
    HORIZONTAL_BARRIER_COLLIDER_GROUP, NEUTRAL_PROJECTILE_COLLIDER_GROUP, SPAWNABLE_COLLIDER_GROUP,
};
use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::ProjectileAssets,
    game::GameParametersResource,
    spawnable::{SpawnableBehavior, SpawnableComponent},
    weapon::WeaponProjectileInitialVelocitiesExt,
};

mod behavior;

pub use self::behavior::{projectile_execute_behavior_system, ProjectileBehavior};

use super::{ColliderData, InitialMotion};

#[derive(Event, Clone)]
pub struct FireWeaponEvent {
    pub weapon_projectile_data: WeaponProjectileData,
    pub source_transform: Transform,
    pub source_entity: Entity,
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
    /// Color for bloom effect
    pub bloom_color: Color,
}

impl ProjectileData {
    pub fn affine_bloom_transformation(&self, bloom_intensity: f32) -> Color {
        Color::rgb(
            1.0 + self.bloom_color.r() * bloom_intensity,
            1.0 + self.bloom_color.g() * bloom_intensity,
            1.0 + self.bloom_color.b() * bloom_intensity,
        )
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
    mut fire_weapon_event_reader: EventReader<FireWeaponEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    projectile_resource: Res<ProjectileResource>,
    projectile_assets: Res<ProjectileAssets>,
    game_parameters: Res<GameParametersResource>,
    game_options: Res<GameOptions>,
) {
    for event in fire_weapon_event_reader.read() {
        spawn_projectile_from_weapon(
            &mut commands,
            &mut sound_effect_event_writer,
            event.weapon_projectile_data.clone(),
            event.initial_motion.clone(),
            event.source_entity,
            event.source_transform,
            &projectile_resource,
            &projectile_assets,
            &game_parameters,
            &game_options,
        );
    }
}

pub fn spawn_projectile_from_weapon(
    commands: &mut Commands,
    sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    weapon_projectile_data: WeaponProjectileData,
    initial_motion: InitialMotion,
    source_entity: Entity,
    source_transform: Transform,
    projectile_resource: &ProjectileResource,
    projectile_assets: &ProjectileAssets,
    game_parameters: &GameParametersResource,
    game_options: &GameOptions,
) {
    // Play the sound effect for the projectiles firing
    sound_effect_event_writer.send(PlaySoundEffectEvent {
        sound_effect_type: weapon_projectile_data.sound.clone(),
    });

    // Get data for the type of ammunition being spawned
    let projectile_data = &projectile_resource.projectiles[&weapon_projectile_data.ammunition];

    // Get the behaviors for the type of ammunition being spawned and add a despawn behavior
    let mut projectile_behaviors = projectile_data.projectile_behaviors.clone();
    projectile_behaviors.push(ProjectileBehavior::TimedDespawn {
        despawn_time: weapon_projectile_data.despawn_time,
    });

    // Create the transform for spawned projectiles
    let projectile_transform = Transform {
        translation: match weapon_projectile_data.position {
            thetawave_interface::spawnable::SpawnPosition::Global(pos) => pos,
            thetawave_interface::spawnable::SpawnPosition::Local(pos) => {
                source_transform.translation.xy() + pos
            }
        }
        .extend(projectile_data.z_level),
        scale: Vec2::splat(game_parameters.sprite_scale * weapon_projectile_data.size).extend(1.0),
        rotation: Quat::from_rotation_z(weapon_projectile_data.direction),
    };

    // Set the correct collider group for the ammunition based on the faction
    let projectile_colider_group =
        get_projectile_collider_group(weapon_projectile_data.ammunition.get_faction());

    // Get a vec of linvels to create the spread pattern
    let spread_linvels = weapon_projectile_data.get_linvels(game_parameters.max_player_projectiles);

    for linvel in spread_linvels {
        let new_initial_motion =
            if let Some(mut initial_motion_linvel) = initial_motion.clone().linvel {
                // Convert the angle to a velocity vector
                initial_motion_linvel += linvel;

                InitialMotion {
                    linvel: Some(initial_motion_linvel),
                    ..initial_motion.clone()
                }
            } else {
                initial_motion.clone()
            };

        // create projectile entity
        let mut projectile = commands.spawn_empty();

        projectile
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(SpriteSheetBundle {
                atlas: projectile_assets
                    .get_texture_atlas_layout(&weapon_projectile_data.ammunition)
                    .into(),
                texture: projectile_assets.get_image(&weapon_projectile_data.ammunition),
                sprite: Sprite {
                    color: projectile_data
                        .affine_bloom_transformation(game_options.bloom_intensity),
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
                damage: weapon_projectile_data.damage,
                time_alive: 0.0,
                source: source_entity,
            })
            .insert(SpawnableComponent {
                spawnable_type: SpawnableType::Projectile(projectile_data.projectile_type.clone()),
                acceleration: Vec2::ZERO,
                deceleration: Vec2::ZERO,
                speed: [game_parameters.max_speed, game_parameters.max_speed].into(), // highest possible speed
                angular_acceleration: 0.0,
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
