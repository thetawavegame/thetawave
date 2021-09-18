use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    game::GameParametersResource,
    spawnable::InitialMotion,
    spawnable::{BehaviorType, MobType, SpawnableComponent, SpawnableType},
    visual::{AnimationComponent, AnimationDirection},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};

/// Core component for mobs
pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of behaviors the mob performs
    pub behaviors: Vec<BehaviorType>,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// Maximum speed that can be accelerated to
    pub speed: Vec2,
    /// Angular acceleration stat
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    pub angular_deceleration: f32,
    /// Maximum angular speed that can be accelerated to
    pub angular_speed: f32,
    /// Motion that the mob initializes with
    pub initial_motion: InitialMotion,
    /// Dimensions of the mob's hitbox
    pub collider_dimensions: Vec2,
    /// Texture
    pub texture: TextureData,
    /// Optional data describing the thruster
    pub thruster: Option<ThrusterData>,
}

/// Data describing thrusters
#[derive(Deserialize)]
pub struct ThrusterData {
    /// Y offset from center of entity
    pub y_offset: f32,
    /// Texture
    pub texture: TextureData,
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

/// Stores data about mob entities
pub struct MobsResource {
    /// Mob types mapped to mob data
    pub mobs: HashMap<MobType, MobData>,
    /// Mob types mapped to their texture and optional thruster texture
    pub texture_atlas_handle:
        HashMap<MobType, (Handle<TextureAtlas>, Option<Handle<TextureAtlas>>)>,
}

/// Spawn a mob entity
pub fn spawn_mob(
    mob_type: &MobType,
    mob_resource: &MobsResource,
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
) {
    /// Get data from mob resource
    let mob_data = &mob_resource.mobs[mob_type];
    let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type].0.clone_weak();

    // scale collider to align with the sprite
    let collider_size_hx =
        mob_data.collider_dimensions.x * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy =
        mob_data.collider_dimensions.y * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    // create mob entity
    let mut mob = commands.spawn();

    mob.insert_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::new(
            game_parameters.sprite_scale,
            game_parameters.sprite_scale,
            1.0,
        )),
        ..Default::default()
    })
    .insert(AnimationComponent {
        timer: Timer::from_seconds(mob_data.texture.frame_duration, true),
        direction: mob_data.texture.animation_direction.clone(),
    })
    .insert_bundle(RigidBodyBundle {
        body_type: RigidBodyType::Dynamic,
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        velocity: RigidBodyVelocity {
            angvel: if let Some(random_angvel) = mob_data.initial_motion.random_angvel {
                thread_rng().gen_range(random_angvel.0..=random_angvel.1)
            } else {
                0.0
            },
            ..Default::default()
        },
        position: position.into(),
        ..Default::default()
    })
    .insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
        material: ColliderMaterial {
            friction: 1.0,
            restitution: 1.0,
            restitution_combine_rule: CoefficientCombineRule::Max,
            ..Default::default()
        },
        flags: ColliderFlags {
            collision_groups: InteractionGroups::new(
                SPAWNABLE_COL_GROUP_MEMBERSHIP,
                u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
            ),
            active_events: ActiveEvents::CONTACT_EVENTS,
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(ColliderPositionSync::Discrete)
    .insert(MobComponent {
        mob_type: mob_data.mob_type.clone(),
    })
    .insert(SpawnableComponent {
        spawnable_type: SpawnableType::Mob(mob_data.mob_type.clone()),
        acceleration: mob_data.acceleration,
        deceleration: mob_data.deceleration,
        speed: mob_data.speed,
        angular_acceleration: mob_data.angular_acceleration,
        angular_deceleration: mob_data.angular_deceleration,
        angular_speed: mob_data.angular_speed,
        behaviors: mob_data.behaviors.clone(),
        should_despawn: false,
    })
    .insert(Name::new(mob_data.mob_type.to_string()));

    // spawn thruster as child if mob has thruster
    if let Some(thruster) = &mob_data.thruster {
        let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type]
            .1
            .as_ref()
            .unwrap()
            .clone_weak();

        mob.with_children(|parent| {
            parent
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_xyz(0.0, thruster.y_offset, 0.0),
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(thruster.texture.frame_duration, true),
                    direction: thruster.texture.animation_direction.clone(),
                })
                .insert(Name::new("Thruster"));
        });
    }
}
