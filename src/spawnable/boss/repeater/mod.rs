use std::collections::HashMap;

use bevy::prelude::*;
use bevy_rapier2d::{na::Translation, prelude::*};
use rand::{thread_rng, Rng};

use serde::Deserialize;
use strum_macros::Display;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum RepeaterPart {
    Head,
    Body,
    RightShoulder,
    LeftShoulder,
    RightArm,
    LeftArm,
}

use crate::{
    animation::{AnimationComponent, TextureData},
    game::GameParametersResource,
    misc::Health,
    spawnable::{BossPartType, EnemyType, MobType, SpawnableComponent, SpawnableType},
    states::{AppStateComponent, AppStates},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

use super::{BossType, MobComponent, MobsResource};

pub struct RepeaterResource {
    pub repeater_parts: RepeaterPartsData,
    pub texture_atlas_handle: HashMap<RepeaterPart, Handle<TextureAtlas>>,
}

#[derive(Deserialize)]
pub struct RepeaterPartsData {
    pub body: RepeaterBodyData,
    pub head: RepeaterHeadData,
    pub rshould: RepeaterShoulderData,
    pub lshould: RepeaterShoulderData,
    pub rarm: RepeaterArmData,
    pub larm: RepeaterArmData,
}

#[derive(Deserialize)]
pub struct RepeaterBodyData {
    pub boss_part_type: BossPartType,
    pub acceleration: Vec2,
    pub deceleration: Vec2,
    pub speed: Vec2,
    pub collider_dimensions: Vec2,
    pub texture: TextureData,
    pub collision_damage: f32,
    pub health: Health,
    pub z_level: f32,
}

#[derive(Deserialize)]
pub struct RepeaterHeadData {
    pub boss_part_type: BossPartType,
    pub collider_dimensions: Vec2,
    pub texture: TextureData,
    pub collision_damage: f32,
    pub health: Health,
    pub z_level: f32,
}

#[derive(Deserialize)]
pub struct RepeaterShoulderData {
    pub boss_part_type: BossPartType,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub angular_speed: f32,
    pub collider_dimensions: Vec2,
    pub texture: TextureData,
    pub collision_damage: f32,
    pub health: Health,
    pub z_level: f32,
}

#[derive(Deserialize)]
pub struct RepeaterArmData {
    pub boss_part_type: BossPartType,
    pub angular_acceleration: f32,
    pub angular_deceleration: f32,
    pub angular_speed: f32,
    pub collider_dimensions: Vec2,
    pub texture: TextureData,
    pub attack_damage: f32,
    pub collision_damage: f32,
    pub health: Health,
    pub z_level: f32,
}

// TODO: create function specific to spawning repeater
pub fn spawn_repeater(
    mob_resource: &MobsResource,
    position: Vec2,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
}

// TODO: use boss type to spawn different kinds of bosses`
/// Spawn boss from give boss type
pub fn spawn_boss(
    boss_type: &BossType,
    repeater_resource: &RepeaterResource,
    position: Vec2,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from mob resource
    let body_data = &repeater_resource.repeater_parts.body;
    let body_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::Body].clone_weak();

    let head_data = &repeater_resource.repeater_parts.head;
    let head_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::Head].clone_weak();

    let rshould_data = &repeater_resource.repeater_parts.rshould;
    let rshould_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::RightShoulder].clone_weak();

    let lshould_data = &repeater_resource.repeater_parts.lshould;
    let lshould_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::LeftShoulder].clone_weak();

    let rarm_data = &repeater_resource.repeater_parts.rarm;
    let rarm_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::RightArm].clone_weak();

    let larm_data = &repeater_resource.repeater_parts.larm;
    let larm_texture_atlas_handle =
        repeater_resource.texture_atlas_handle[&RepeaterPart::LeftArm].clone_weak();

    // scale collider to align with the sprite
    let body_collider_size_hx =
        body_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let body_collider_size_hy =
        body_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let head_collider_size_hx =
        head_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let head_collider_size_hy =
        head_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let rshould_collider_size_hx =
        rshould_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let rshould_collider_size_hy =
        rshould_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let lshould_collider_size_hx =
        lshould_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let lshould_collider_size_hy =
        lshould_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let rarm_collider_size_hx =
        rarm_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let rarm_collider_size_hy =
        rarm_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let larm_collider_size_hx =
        larm_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let larm_collider_size_hy =
        larm_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // right shoulder joint
    let right_shoulder_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(80.0, 115.0))
        .local_anchor2(Vec2::new(-60.0, 50.0))
        .motor_velocity(0.0, 1.0);

    let right_elbow_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(62.0, -78.0))
        .local_anchor2(Vec2::new(30.0, 110.0))
        .motor_velocity(0.0, 1.0);

    let left_shoulder_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-80.0, 115.0))
        .local_anchor2(Vec2::new(60.0, 50.0))
        .motor_velocity(0.0, 1.0);

    let left_elbow_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(-62.0, -78.0))
        .local_anchor2(Vec2::new(-30.0, 110.0))
        .motor_velocity(0.0, 1.0);

    // create mob entity
    //let mut mob = commands.spawn();

    let mob = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle {
            local: Transform {
                translation: Vec3::new(position.x, position.y, 0.0),
                ..Default::default()
            },
            ..default()
        })
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(AppStateComponent(AppStates::Game))
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Repeater Core"))
        .with_children(|parent| {
            // head
            parent
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: head_texture_atlas_handle,
                    transform: Transform {
                        //translation: position.extend(head_data.z_level),
                        translation: Vec3::new(0.0, 0.0, head_data.z_level),
                        scale: Vec3::new(
                            game_parameters.sprite_scale,
                            game_parameters.sprite_scale,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(head_data.texture.frame_duration, true),
                    direction: head_data.texture.animation_direction.clone(),
                })
                .insert(Collider::cuboid(
                    head_collider_size_hx,
                    head_collider_size_hy,
                ))
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(head_data.boss_part_type.clone()),
                    acceleration: Vec2::ZERO,
                    deceleration: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    angular_acceleration: 0.0,
                    angular_deceleration: 0.0,
                    angular_speed: 0.0,
                    behaviors: [].to_vec(),
                })
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Name::new(head_data.boss_part_type.to_string()));

            // body
            parent
                .spawn()
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: body_texture_atlas_handle,
                    transform: Transform {
                        //translation: position.extend(head_data.z_level),
                        translation: Vec3::new(0.0, 110.0, body_data.z_level),
                        scale: Vec3::new(
                            game_parameters.sprite_scale,
                            game_parameters.sprite_scale,
                            1.0,
                        ),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(body_data.texture.frame_duration, true),
                    direction: body_data.texture.animation_direction.clone(),
                })
                .insert(Collider::cuboid(
                    body_collider_size_hx,
                    body_collider_size_hy,
                ))
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(body_data.boss_part_type.clone()),
                    acceleration: body_data.acceleration,
                    deceleration: body_data.deceleration,
                    speed: body_data.speed,
                    angular_acceleration: 0.0,
                    angular_deceleration: 0.0,
                    angular_speed: 0.0,
                    behaviors: [].to_vec(),
                })
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Name::new(body_data.boss_part_type.to_string()));
        })
        .id();

    //right shoulder

    let upper_right_arm = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(AppStateComponent(AppStates::Game))
        .insert(ImpulseJoint::new(mob, right_shoulder_joint))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: rshould_texture_atlas_handle,
            transform: Transform {
                //translation: position.extend(head_data.z_level),
                translation: Vec3::new(0.0, 0.0, rshould_data.z_level),
                scale: Vec3::new(
                    game_parameters.sprite_scale,
                    game_parameters.sprite_scale,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(rshould_data.texture.frame_duration, true),
            direction: rshould_data.texture.animation_direction.clone(),
        })
        //.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Name::new(rshould_data.boss_part_type.to_string()))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(
                    rshould_collider_size_hx,
                    rshould_collider_size_hy,
                ))
                .insert_bundle(TransformBundle {
                    local: Transform {
                        rotation: Quat::from_rotation_z(0.78),
                        translation: Vec3::new(10.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(rshould_data.boss_part_type.clone()),
                    acceleration: Vec2::ZERO,
                    deceleration: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    angular_acceleration: rshould_data.angular_acceleration,
                    angular_deceleration: rshould_data.angular_deceleration,
                    angular_speed: rshould_data.angular_speed,
                    behaviors: [].to_vec(),
                });
        })
        .id();

    // right arm
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(AppStateComponent(AppStates::Game))
        .insert(ImpulseJoint::new(upper_right_arm, right_elbow_joint))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: rarm_texture_atlas_handle,
            transform: Transform {
                //translation: position.extend(head_data.z_level),
                translation: Vec3::new(0.0, 0.0, rarm_data.z_level),
                scale: Vec3::new(
                    game_parameters.sprite_scale,
                    game_parameters.sprite_scale,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(rarm_data.texture.frame_duration, true),
            direction: rarm_data.texture.animation_direction.clone(),
        })
        //.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Name::new(rarm_data.boss_part_type.to_string()))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(
                    rarm_collider_size_hx,
                    rarm_collider_size_hy,
                ))
                .insert_bundle(TransformBundle {
                    local: Transform {
                        rotation: Quat::from_rotation_z(0.3),
                        translation: Vec3::new(53.0, 37.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(rarm_data.boss_part_type.clone()),
                    acceleration: Vec2::ZERO,
                    deceleration: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    angular_acceleration: rarm_data.angular_acceleration,
                    angular_deceleration: rarm_data.angular_deceleration,
                    angular_speed: rarm_data.angular_speed,
                    behaviors: [].to_vec(),
                });
        });

    //left shoulder

    let upper_left_arm = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(AppStateComponent(AppStates::Game))
        .insert(ImpulseJoint::new(mob, left_shoulder_joint))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: lshould_texture_atlas_handle,
            transform: Transform {
                //translation: position.extend(head_data.z_level),
                translation: Vec3::new(0.0, 0.0, lshould_data.z_level),
                scale: Vec3::new(
                    game_parameters.sprite_scale,
                    game_parameters.sprite_scale,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(lshould_data.texture.frame_duration, true),
            direction: lshould_data.texture.animation_direction.clone(),
        })
        //.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Name::new(lshould_data.boss_part_type.to_string()))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(
                    lshould_collider_size_hx,
                    lshould_collider_size_hy,
                ))
                .insert_bundle(TransformBundle {
                    local: Transform {
                        rotation: Quat::from_rotation_z(-0.78),
                        translation: Vec3::new(-10.0, 0.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(lshould_data.boss_part_type.clone()),
                    acceleration: Vec2::ZERO,
                    deceleration: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    angular_acceleration: lshould_data.angular_acceleration,
                    angular_deceleration: lshould_data.angular_deceleration,
                    angular_speed: lshould_data.angular_speed,
                    behaviors: [].to_vec(),
                });
        })
        .id();

    // left arm
    commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(AppStateComponent(AppStates::Game))
        .insert(ImpulseJoint::new(upper_left_arm, left_elbow_joint))
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: larm_texture_atlas_handle,
            transform: Transform {
                //translation: position.extend(head_data.z_level),
                translation: Vec3::new(0.0, 0.0, larm_data.z_level),
                scale: Vec3::new(
                    game_parameters.sprite_scale,
                    game_parameters.sprite_scale,
                    1.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(larm_data.texture.frame_duration, true),
            direction: larm_data.texture.animation_direction.clone(),
        })
        //.insert(LockedAxes::ROTATION_LOCKED)
        .insert(Name::new(larm_data.boss_part_type.to_string()))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(Collider::cuboid(
                    larm_collider_size_hx,
                    larm_collider_size_hy,
                ))
                .insert_bundle(TransformBundle {
                    local: Transform {
                        rotation: Quat::from_rotation_z(-0.3),
                        translation: Vec3::new(-53.0, 37.0, 0.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Friction::new(1.0))
                .insert(Restitution {
                    coefficient: 1.0,
                    combine_rule: CoefficientCombineRule::Max,
                })
                .insert(CollisionGroups {
                    memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    filters: u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                })
                .insert(SpawnableComponent {
                    spawnable_type: SpawnableType::BossPart(larm_data.boss_part_type.clone()),
                    acceleration: Vec2::ZERO,
                    deceleration: Vec2::ZERO,
                    speed: Vec2::ZERO,
                    angular_acceleration: larm_data.angular_acceleration,
                    angular_deceleration: larm_data.angular_deceleration,
                    angular_speed: larm_data.angular_speed,
                    behaviors: [].to_vec(),
                });
        });
}
