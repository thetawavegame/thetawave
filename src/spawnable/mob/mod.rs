use bevy::prelude::*;
use bevy_rapier2d::{
    geometry::ColliderMassProperties,
    prelude::{
        ActiveEvents, CoefficientCombineRule, Collider, CollisionGroups, Friction, Group,
        LockedAxes, Restitution, RevoluteJointBuilder, RigidBody, Velocity,
    },
};
use serde::Deserialize;
use std::collections::{hash_map::Entry, HashMap};

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::MobAssets,
    game::GameParametersResource,
    loot::DropListType,
    spawnable::{SpawnableBehavior, SpawnableComponent},
};

mod behavior;
mod mob_segment;
pub(crate) use self::{behavior::*, mob_segment::*};

use super::{behavior_sequence::MobBehaviorSequenceType, InitialMotion};
use crate::collision::{
    HORIZONTAL_BARRIER_COLLIDER_GROUP, MOB_COLLIDER_GROUP, SPAWNABLE_COLLIDER_GROUP,
};
use thetawave_interface::{
    audio::CollisionSoundType,
    game::options::GameOptions,
    health::HealthComponent,
    objective::DefenseInteraction,
    spawnable::{MobDestroyedEvent, MobSegmentType, MobType, SpawnMobEvent, SpawnPosition},
    states::GameCleanup,
    weapon::{WeaponData, WeaponsComponent},
};

/// Core component for mobs
#[derive(Component)]
pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
    /// Mob specific behaviors
    pub behaviors: Vec<behavior::MobBehavior>,
    /// behaviors that mob segments attached to the mob will perform, given the mob's control behaviors
    pub mob_segment_behaviors: Option<
        HashMap<MobSegmentControlBehavior, HashMap<MobSegmentType, Vec<MobSegmentBehavior>>>,
    >,
    /// Control behaviors currently in use
    pub control_behaviors: Vec<MobSegmentControlBehavior>,
    /// Behavior sequence that the mob is using
    pub behavior_sequence: Option<MobBehaviorSequenceType>,
    /// Tracks the behavior sequence of the mob
    pub behavior_sequence_tracker: Option<BehaviorSequenceTracker>,
    /// Tracks available mob spawning patterns for the mob
    pub mob_spawners: HashMap<String, Vec<MobSpawner>>,
    /// Damage dealt to other factions on collision
    pub collision_damage: usize,
    pub collision_sound: CollisionSoundType,
    /// Damage dealt to defense objective, after reaching bottom of arena
    pub defense_interaction: Option<DefenseInteraction>,
    /// List of consumable drops
    pub loot_drops: DropListType,
}

impl From<&MobData> for MobComponent {
    fn from(mob_data: &MobData) -> Self {
        let mut mob_spawners: HashMap<String, Vec<MobSpawner>> = HashMap::new();

        for (spawner_name, spawners) in mob_data.mob_spawners.iter() {
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

        MobComponent {
            mob_type: mob_data.mob_type.clone(),
            behaviors: mob_data.mob_behaviors.clone(),
            behavior_sequence: mob_data.behavior_sequence_type.clone(),
            mob_segment_behaviors: mob_data.mob_segment_behaviors.clone(),
            control_behaviors: mob_data.control_behaviors.clone(),
            behavior_sequence_tracker: None,
            mob_spawners,
            collision_damage: mob_data.collision_damage,
            collision_sound: mob_data.collision_sound.clone(),
            defense_interaction: mob_data.defense_interaction.clone(),
            loot_drops: mob_data.consumable_drops.clone(),
        }
    }
}

#[derive(Component)]
pub struct BossComponent;

#[derive(Deserialize, Clone, Debug)]
pub struct MobSpawner {
    pub mob_type: MobType,
    pub timer: Timer,
    pub position: SpawnPosition,
}

impl From<MobSpawnerData> for MobSpawner {
    fn from(value: MobSpawnerData) -> Self {
        MobSpawner {
            mob_type: value.mob_type.clone(),
            position: value.position.clone(),
            timer: Timer::from_seconds(value.period, TimerMode::Repeating),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct MobSpawnerData {
    pub mob_type: MobType,
    pub period: f32,
    pub position: SpawnPosition,
}

pub struct BehaviorSequenceTracker {
    pub timer: Timer,
    pub index: usize,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of spawnable behaviors that are performed
    #[serde(default)]
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// Behavior sequence type
    pub behavior_sequence_type: Option<MobBehaviorSequenceType>,
    /// List of mob behaviors that are performed
    #[serde(default)]
    pub mob_behaviors: Vec<MobBehavior>,
    /// behaviors used to control attached mob segments
    #[serde(default)]
    pub control_behaviors: Vec<MobSegmentControlBehavior>,
    /// behaviors that mob segments attached to the mob will perform, given the mobs current behavior
    pub mob_segment_behaviors: Option<
        HashMap<MobSegmentControlBehavior, HashMap<MobSegmentType, Vec<MobSegmentBehavior>>>,
    >,
    /// Whether the mob can rotate on its z axis
    #[serde(default)]
    pub can_rotate: bool,
    /// Acceleration stat
    #[serde(default)]
    pub acceleration: Vec2,
    /// Deceleration stat
    #[serde(default)]
    pub deceleration: Vec2,
    /// Maximum speed that can be accelerated to
    #[serde(default)]
    pub speed: Vec2,
    /// Angular acceleration stat
    #[serde(default)]
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    #[serde(default)]
    pub angular_deceleration: f32,
    /// Maximum angular speed that can be accelerated to
    #[serde(default)]
    pub angular_speed: f32,
    /// Motion that the mob initializes with
    #[serde(default)]
    pub initial_motion: InitialMotion,
    /// Dimensions of the mob's hitbox
    pub colliders: Vec<ColliderData>,
    /// Texture
    pub animation: AnimationData,
    /// Optional data describing the thruster
    pub thruster: Option<ThrusterData>,
    /// Damage dealt to other factions through attacks
    #[serde(default)]
    pub collision_damage: usize,
    /// Damage dealt to defense objective, after reaching bottom of arena
    #[serde(default)]
    pub collision_sound: CollisionSoundType,
    pub defense_interaction: Option<DefenseInteraction>,
    /// Health of the mob
    pub health: usize,
    /// List of consumable drops
    #[serde(default)]
    pub consumable_drops: DropListType,
    /// Z level of the mobs transform
    pub z_level: f32,
    /// anchor points for other mob segments
    #[serde(default)]
    pub mob_segment_anchor_points: Vec<MobSegmentAnchorPointData>,
    /// mob spawners that the mob can use
    #[serde(default)]
    pub mob_spawners: HashMap<String, Vec<MobSpawnerData>>,
    /// projectile spawners that the mob can use
    #[serde(default)]
    pub weapons: Option<Vec<WeaponData>>,
    #[serde(default = "default_mob_density")]
    pub density: f32,
}

fn default_mob_density() -> f32 {
    1.0
}

impl From<&MobData> for HealthComponent {
    fn from(mob_data: &MobData) -> Self {
        HealthComponent::new(mob_data.health, 0, 0.0)
    }
}

impl MobData {
    pub fn get_weapon_component(&self) -> Option<WeaponsComponent> {
        self.weapons.clone().map(WeaponsComponent::from)
    }
}

#[derive(Deserialize, Clone)]
pub struct ColliderData {
    pub dimensions: Vec2,
    pub rotation: f32,
    pub position: Vec2,
}

pub type CompoundColliderData = (Vec2, f32, Collider);

impl From<ColliderData> for CompoundColliderData {
    fn from(val: ColliderData) -> Self {
        (
            val.position,
            val.rotation,
            Collider::cuboid(val.dimensions.x, val.dimensions.y),
        )
    }
}

#[derive(Deserialize, Clone)]
pub struct MobSegmentAnchorPointData {
    pub position: Vec2,
    pub mob_segment_type: MobSegmentType,
    pub joint: JointType,
    pub target_pos: f32,
    pub stiffness: f32,
    pub damping: f32,
}

#[derive(Deserialize, Clone)]
pub enum JointType {
    Revolute,
}

/// Spawns mobs from events
pub fn spawn_mob_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnMobEvent>,
    mob_resource: Res<MobsResource>,
    mob_segments_resource: Res<MobSegmentsResource>,
    mob_assets: Res<MobAssets>,
    game_parameters: Res<GameParametersResource>,
    game_options: Res<GameOptions>,
) {
    for event in event_reader.read() {
        spawn_mob(
            &event.mob_type,
            &mob_resource,
            &mob_segments_resource,
            &mob_assets,
            event.position,
            event.rotation,
            event.boss,
            &mut commands,
            &game_parameters,
            &game_options,
        );
    }
}

/// Data describing thrusters
#[derive(Deserialize)]
pub struct ThrusterData {
    /// Y offset from center of entity
    pub y_offset: f32,
    /// Texture
    pub animation: AnimationData,
    /// Color for bloom effect
    pub bloom_color: Color,
}

impl ThrusterData {
    /// Color for bloom effect, multiplied by the bloom intensity value
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
pub struct MobsResource {
    /// Mob types mapped to mob data
    pub mobs: HashMap<MobType, MobData>,
}

/// Spawn a mob entity
#[allow(clippy::too_many_arguments)]
pub fn spawn_mob(
    mob_type: &MobType,
    mob_resource: &MobsResource,
    mob_segments_resource: &MobSegmentsResource,
    mob_assets: &MobAssets,
    position: Vec2,
    rotation: Quat,
    boss: bool,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
    game_options: &GameOptions,
) {
    // Get data from mob resource
    let mob_data = &mob_resource.mobs[mob_type];

    // create mob entity
    let mut mob = commands.spawn_empty();

    mob.insert(SpriteSheetBundle {
        atlas: mob_assets.get_mob_texture_atlas_layout(mob_type).into(),
        texture: mob_assets.get_mob_image(mob_type),
        transform: Transform {
            translation: position.extend(mob_data.z_level),
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            rotation,
        },
        ..Default::default()
    })
    .insert(AnimationComponent {
        timer: Timer::from_seconds(mob_data.animation.frame_duration, TimerMode::Repeating),
        direction: mob_data.animation.direction.clone(),
    })
    .insert(RigidBody::Dynamic)
    .insert(Velocity::from(mob_data.initial_motion.clone()))
    .insert(Collider::compound(
        mob_data
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
    .insert(MobComponent::from(mob_data))
    .insert(HealthComponent::from(mob_data))
    .insert(SpawnableComponent::from(mob_data))
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(GameCleanup)
    .insert(ColliderMassProperties::Density(mob_data.density))
    .insert(Name::new(mob_data.mob_type.to_string()));

    if boss {
        mob.insert(BossComponent);
    }

    if !mob_data.can_rotate {
        mob.insert(LockedAxes::ROTATION_LOCKED);
    }

    if let Some(weapon_component) = mob_data.get_weapon_component() {
        mob.insert(weapon_component);
    }

    // spawn thruster as child if mob has thruster
    if let Some(thruster) = &mob_data.thruster {
        mob.with_children(|parent| {
            parent
                .spawn(SpriteSheetBundle {
                    atlas: mob_assets
                        .get_thruster_texture_atlas_layout(mob_type)
                        .unwrap()
                        .into(),
                    texture: mob_assets.get_thruster_image(mob_type).unwrap().into(),
                    transform: Transform::from_xyz(0.0, thruster.y_offset, -1.0),
                    sprite: Sprite {
                        color: thruster.affine_bloom_transformation(game_options.bloom_intensity),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(
                        thruster.animation.frame_duration,
                        TimerMode::Repeating,
                    ),
                    direction: thruster.animation.direction.clone(),
                })
                .insert(Name::new("Thruster"));
        });
    }

    let mob_entity = mob.id();
    // add mob segment if anchor point
    for anchor_point in mob_data.mob_segment_anchor_points.iter() {
        // spawn mob_segment

        let mob_segment_data = &mob_segments_resource.mob_segments[&anchor_point.mob_segment_type];

        // create joint
        let joint = match &anchor_point.joint {
            JointType::Revolute => RevoluteJointBuilder::new()
                .local_anchor1(anchor_point.position)
                .local_anchor2(mob_segment_data.anchor_point)
                .motor_position(
                    anchor_point.target_pos,
                    anchor_point.stiffness,
                    anchor_point.damping,
                ),
        };

        spawn_mob_segment(
            &mob_segment_data.mob_segment_type,
            mob_entity,
            &joint,
            mob_segments_resource,
            mob_assets,
            position,
            anchor_point.position,
            commands,
            game_parameters,
        )
    }
}

#[derive(Event)]
pub struct BossesDestroyedEvent;

pub fn check_boss_mobs_system(
    boss_mobs_query: Query<&BossComponent>,
    mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>,
    mut bosses_destroyed_event_writer: EventWriter<BossesDestroyedEvent>,
) {
    for event in mob_destroyed_event_reader.read() {
        // check if the mob that was destroyed was a boss, and check that there are no remaining boss mobs
        if event.is_boss && boss_mobs_query.get_single().is_err() {
            info!("Sending bosses destroyed event");
            bosses_destroyed_event_writer.send(BossesDestroyedEvent);
        }
    }
}
