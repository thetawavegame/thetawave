use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::MobAssets,
    game::GameParametersResource,
    loot::ConsumableDropListType,
    misc::Health,
    spawnable::{MobSegmentType, SpawnableComponent, SpawnableType},
    states::{AppStateComponent, AppStates},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

mod behavior;

#[derive(Resource)]
pub struct MobSegmentsResource {
    /// Mob types mapped to mob data
    pub mob_segments: HashMap<MobSegmentType, MobSegmentData>,
}

#[derive(Component)]
// additional segment of mob that is jointed to a mob
pub struct MobSegmentComponent {
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: f32,
    pub defense_damage: f32,
    pub health: Health,
    pub consumable_drops: ConsumableDropListType,
    pub behaviors: Vec<behavior::MobSegmentBehavior>,
}

impl From<&MobSegmentData> for MobSegmentComponent {
    fn from(mob_segment_data: &MobSegmentData) -> Self {
        MobSegmentComponent {
            mob_segment_type: mob_segment_data.mob_segment_type.clone(),
            collision_damage: mob_segment_data.collision_damage,
            defense_damage: mob_segment_data.defense_damage,
            health: mob_segment_data.health.clone(),
            consumable_drops: mob_segment_data.consumable_drops.clone(),
            behaviors: mob_segment_data.behaviors.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct MobSegmentData {
    pub animation: AnimationData,
    pub collider_dimensions: Vec2,
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: f32,
    pub defense_damage: f32,
    pub health: Health,
    pub consumable_drops: ConsumableDropListType,
    pub z_level: f32,
    pub anchor_point: Vec2,
    pub behaviors: Vec<behavior::MobSegmentBehavior>,
}

pub fn spawn_mob_segment(
    mob_segment_type: &MobSegmentType,
    joint_parent_entity: Entity,
    joint: &RevoluteJointBuilder,
    mob_segments_resource: &MobSegmentsResource,
    mob_assets: &MobAssets,
    position: Vec2,
    parent_anchor_point: Vec2,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    let mob_segment_data = &mob_segments_resource.mob_segments[mob_segment_type];

    // scale collider to align with the sprite
    let collider_size_hx =
        mob_segment_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy =
        mob_segment_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    let mut mob_segment = commands.spawn_empty();

    mob_segment
        .insert(ImpulseJoint::new(joint_parent_entity, *joint))
        .insert(SpriteSheetBundle {
            texture_atlas: mob_assets.get_mob_segment_asset(mob_segment_type),
            transform: Transform {
                translation: Vec3::new(
                    position.x + parent_anchor_point.x - mob_segment_data.anchor_point.x,
                    position.y + parent_anchor_point.y - mob_segment_data.anchor_point.y,
                    mob_segment_data.z_level,
                ),
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
            timer: Timer::from_seconds(
                mob_segment_data.animation.frame_duration,
                TimerMode::Repeating,
            ),
            direction: mob_segment_data.animation.direction.clone(),
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Friction::new(1.0))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(CollisionGroups {
            memberships: SPAWNABLE_COL_GROUP_MEMBERSHIP,
            filters: Group::ALL ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
        })
        .insert(MobSegmentComponent::from(mob_segment_data))
        .insert(SpawnableComponent::new(SpawnableType::MobSegment(
            mob_segment_type.clone(),
        )))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new(mob_segment_data.mob_segment_type.to_string()));
}
