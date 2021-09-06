use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    game::GameParametersResource,
    spawnable::{BehaviorType, MobType, SpawnableComponent, SpawnableType},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
}

#[derive(Deserialize)]
pub struct MobData {
    pub mob_type: MobType,
    pub behaviors: Vec<BehaviorType>,
    pub acceleration: Vec2,
    pub deceleration: Vec2,
    pub speed: Vec2,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub angular_speed: f32,
    pub collider_dimensions: Vec2,
    pub texture: TextureData,
    pub thruster: Option<ThrusterData>,
}

#[derive(Deserialize)]
pub struct ThrusterData {
    pub y_offset: f32,
    pub texture: TextureData,
}

#[derive(Deserialize)]
pub struct TextureData {
    pub path: String,
    pub dimensions: Vec2,
    pub cols: usize,
    pub rows: usize,
}

pub struct MobsResource {
    pub mobs: HashMap<MobType, MobData>,
    pub texture_atlas_handle:
        HashMap<MobType, (Handle<TextureAtlas>, Option<Handle<TextureAtlas>>)>,
}

pub fn spawn_mob(
    mob_type: &MobType,
    mob_resource: &MobsResource,
    position: Vec2,
    commands: &mut Commands,
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
) {
    let mob_data = &mob_resource.mobs[mob_type];
    let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type].0.clone_weak();

    // scale collider to align with the sprite
    let collider_size_hx =
        mob_data.collider_dimensions.x * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy =
        mob_data.collider_dimensions.y * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    let transform = Transform::from_scale(Vec3::new(
        game_parameters.sprite_scale,
        game_parameters.sprite_scale,
        1.0,
    ));

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
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
}
