use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::{hash_map::Entry, HashMap};
use thetawave_interface::{
    audio::CollisionSoundType,
    health::HealthComponent,
    objective::DefenseInteraction,
    spawnable::{MobSegmentType, SpawnableType},
    states::GameCleanup,
    weapon::{WeaponData, WeaponsComponent},
};

use crate::collision::{
    HORIZONTAL_BARRIER_COLLIDER_GROUP, MOB_COLLIDER_GROUP, SPAWNABLE_COLLIDER_GROUP,
};
use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::MobAssets,
    game::GameParametersResource,
    loot::DropListType,
    spawnable::SpawnableComponent,
};

mod behavior;
pub use self::behavior::*;

use super::{
    ColliderData, CompoundColliderData, JointType, MobSegmentAnchorPointData, MobSpawner,
    MobSpawnerData,
};

#[derive(Resource, Deserialize)]
pub struct MobSegmentsResource {
    /// Mob types mapped to mob data
    pub mob_segments: HashMap<MobSegmentType, MobSegmentData>,
}

#[derive(Component)]
// additional segment of mob that is jointed to a mob
pub struct MobSegmentComponent {
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: usize,
    pub collision_sound: CollisionSoundType,
    pub defense_interaction: Option<DefenseInteraction>,
    pub consumable_drops: DropListType,
    pub behaviors: Vec<behavior::MobSegmentBehavior>,
    pub mob_spawners: HashMap<String, Vec<MobSpawner>>,
}

impl From<&MobSegmentData> for MobSegmentComponent {
    fn from(mob_segment_data: &MobSegmentData) -> Self {
        let mut mob_spawners: HashMap<String, Vec<MobSpawner>> = HashMap::new();

        if let Some(spawners_map) = mob_segment_data.mob_spawners.clone() {
            for (spawner_name, spawners) in spawners_map.iter() {
                for spawner in spawners.iter() {
                    match mob_spawners.entry(spawner_name.clone()) {
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(MobSpawner::from(spawner.clone()));
                        }
                        Entry::Vacant(e) => {
                            e.insert(vec![MobSpawner::from(spawner.clone())]);
                        }
                    }
                }
            }
        }

        MobSegmentComponent {
            mob_segment_type: mob_segment_data.mob_segment_type.clone(),
            collision_damage: mob_segment_data.collision_damage,
            collision_sound: mob_segment_data.collision_sound.clone(),
            defense_interaction: mob_segment_data.defense_interaction.clone(),
            consumable_drops: mob_segment_data.consumable_drops.clone(),
            behaviors: mob_segment_data.behaviors.clone(),
            mob_spawners,
        }
    }
}

#[derive(Deserialize)]
pub struct MobSegmentData {
    pub animation: AnimationData,
    pub colliders: Vec<ColliderData>,
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: usize,
    #[serde(default)]
    pub collision_sound: CollisionSoundType,
    #[serde(default)]
    pub defense_interaction: Option<DefenseInteraction>,
    pub health: usize,
    pub consumable_drops: DropListType,
    pub z_level: f32,
    pub anchor_point: Vec2,
    pub mob_segment_anchor_points: Option<Vec<MobSegmentAnchorPointData>>,
    pub behaviors: Vec<MobSegmentBehavior>,
    pub disconnected_behaviors: Option<Vec<MobSegmentBehavior>>,
    pub mob_spawners: Option<HashMap<String, Vec<MobSpawnerData>>>,
    #[serde(default)]
    pub weapons: Option<Vec<WeaponData>>,
    #[serde(default = "default_mob_segment_density")]
    pub density: f32,
}

fn default_mob_segment_density() -> f32 {
    1.0
}

impl From<&MobSegmentData> for HealthComponent {
    fn from(mob_segment_data: &MobSegmentData) -> Self {
        HealthComponent::new(mob_segment_data.health, 0, 0.0)
    }
}

impl MobSegmentData {
    pub fn get_weapon_component(&self) -> Option<WeaponsComponent> {
        self.weapons.clone().map(WeaponsComponent::from)
    }
}

/// Spawn a mob segment
#[allow(clippy::too_many_arguments)]
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

    let mut mob_segment = commands.spawn_empty();

    let new_position = Vec2::new(
        position.x + parent_anchor_point.x - mob_segment_data.anchor_point.x,
        position.y + parent_anchor_point.y - mob_segment_data.anchor_point.y,
    );

    mob_segment
        .insert(ImpulseJoint::new(joint_parent_entity, *joint))
        .insert(SpriteSheetBundle {
            atlas: mob_assets
                .get_mob_segment_texture_atlas_layout(mob_segment_type)
                .into(),
            texture: mob_assets.get_mob_segment_image(mob_segment_type),
            transform: Transform {
                translation: new_position.extend(mob_segment_data.z_level),
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
        .insert(Collider::compound(
            mob_segment_data
                .colliders
                .iter()
                .map(|collider_data| collider_data.clone().into())
                .collect::<Vec<CompoundColliderData>>(),
        ))
        .insert(Friction::new(1.0))
        .insert(Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        })
        .insert(CollisionGroups {
            memberships: SPAWNABLE_COLLIDER_GROUP | MOB_COLLIDER_GROUP,
            filters: Group::ALL ^ HORIZONTAL_BARRIER_COLLIDER_GROUP,
        })
        .insert(MobSegmentComponent::from(mob_segment_data))
        .insert(HealthComponent::from(mob_segment_data))
        .insert(SpawnableComponent::new(SpawnableType::MobSegment(
            mob_segment_type.clone(),
        )))
        .insert(ColliderMassProperties::Density(mob_segment_data.density))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GameCleanup)
        .insert(Velocity::default())
        .insert(Name::new(mob_segment_data.mob_segment_type.to_string()));

    if let Some(weapon_component) = mob_segment_data.get_weapon_component() {
        mob_segment.insert(weapon_component);
    }

    let mob_segment_entity = mob_segment.id();

    if let Some(mob_segment_anchor_points) = mob_segment_data.mob_segment_anchor_points.clone() {
        for mob_segment_anchor_point in mob_segment_anchor_points.iter() {
            let new_mob_segment_data =
                &mob_segments_resource.mob_segments[&mob_segment_anchor_point.mob_segment_type];

            // create joint
            let joint = match &mob_segment_anchor_point.joint {
                JointType::Revolute => RevoluteJointBuilder::new()
                    .local_anchor1(mob_segment_anchor_point.position)
                    .local_anchor2(new_mob_segment_data.anchor_point)
                    .motor_position(
                        mob_segment_anchor_point.target_pos,
                        mob_segment_anchor_point.stiffness,
                        mob_segment_anchor_point.damping,
                    ),
            };

            spawn_mob_segment(
                &new_mob_segment_data.mob_segment_type,
                mob_segment_entity,
                &joint,
                mob_segments_resource,
                mob_assets,
                new_position,
                mob_segment_anchor_point.position,
                commands,
                game_parameters,
            )
        }
    }
}
