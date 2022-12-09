use bevy::prelude::*;
use bevy_rapier2d::geometry::Group;
use bevy_rapier2d::prelude::*;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::{collections::HashMap, string::ToString};

use crate::{
    animation::{AnimationComponent, TextureData},
    game::GameParametersResource,
    loot::ConsumableDropListType,
    misc::Health,
    spawnable::{InitialMotion, MobType, SpawnableBehavior, SpawnableComponent, SpawnableType},
    states::{AppStateComponent, AppStates},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};

mod behavior;
pub use self::behavior::{mob_execute_behavior_system, MobBehavior};

/// Core component for mobs
#[derive(Component)]
pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
    /// Mob specific behaviors
    pub behaviors: Vec<behavior::MobBehavior>,
    /// Optional mob spawn timer
    pub mob_spawn_timer: Option<Timer>,
    /// Optional weapon timer
    pub weapon_timer: Option<Timer>,
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    /// Damage dealt to defense objective, after reaching bottom of arena
    pub defense_damage: f32,
    /// Health of the mob
    pub health: Health,
    /// List of consumable drops
    pub consumable_drops: ConsumableDropListType,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct MobData {
    /// Type of mob
    pub mob_type: MobType,
    /// List of spawnable behaviors that are performed
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// List of mob behaviors that are performed
    pub mob_behaviors: Vec<behavior::MobBehavior>,
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
    /// Damage dealt to other factions through attacks
    pub attack_damage: f32,
    /// Damage dealt to other factions on collision
    pub collision_damage: f32,
    /// Damage dealt to defense objective, after reaching bottom of arena
    pub defense_damage: f32,
    /// Health of the mob
    pub health: Health,
    /// List of consumable drops
    pub consumable_drops: ConsumableDropListType,
    /// Z level of the mobs transform
    pub z_level: f32,
}

/// Event for spawning mobs
pub struct SpawnMobEvent {
    /// Type of mob to spawn
    pub mob_type: MobType,
    /// Position to spawn mob
    pub position: Vec2,
}

/// Spawns mobs from events
pub fn spawn_mob_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnMobEvent>,
    mob_resource: Res<MobsResource>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_mob(
            &event.mob_type,
            &mob_resource,
            event.position,
            &mut commands,
            &game_parameters,
        );
    }
}

/// Data describing thrusters
#[derive(Deserialize)]
pub struct ThrusterData {
    /// Y offset from center of entity
    pub y_offset: f32,
    /// Texture
    pub texture: TextureData,
}

/// Stores data about mob entities
#[derive(Resource)]
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
    game_parameters: &GameParametersResource,
) {
    // Get data from mob resource
    let mob_data = &mob_resource.mobs[mob_type];
    let texture_atlas_handle = mob_resource.texture_atlas_handle[mob_type].0.clone_weak();

    // scale collider to align with the sprite
    let collider_size_hx = mob_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy = mob_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // create mob entity
    let mut mob = commands.spawn_empty();

    mob.insert(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: position.extend(mob_data.z_level),
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
        timer: Timer::from_seconds(mob_data.texture.frame_duration, TimerMode::Repeating),
        direction: mob_data.texture.animation_direction.clone(),
    })
    .insert(RigidBody::Dynamic)
    .insert(LockedAxes::ROTATION_LOCKED)
    .insert(Velocity {
        angvel: if let Some(random_angvel) = mob_data.initial_motion.random_angvel {
            thread_rng().gen_range(random_angvel.0..=random_angvel.1)
        } else {
            0.0
        },
        ..Default::default()
    })
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
    .insert(MobComponent {
        mob_type: mob_data.mob_type.clone(),
        behaviors: mob_data.mob_behaviors.clone(),
        mob_spawn_timer: None,
        weapon_timer: None,
        attack_damage: mob_data.attack_damage,
        collision_damage: mob_data.collision_damage,
        defense_damage: mob_data.defense_damage,
        health: mob_data.health.clone(),
        consumable_drops: mob_data.consumable_drops.clone(),
    })
    .insert(SpawnableComponent {
        spawnable_type: SpawnableType::Mob(mob_data.mob_type.clone()),
        acceleration: mob_data.acceleration,
        deceleration: mob_data.deceleration,
        speed: mob_data.speed,
        angular_acceleration: mob_data.angular_acceleration,
        angular_deceleration: mob_data.angular_deceleration,
        angular_speed: mob_data.angular_speed,
        behaviors: mob_data.spawnable_behaviors.clone(),
    })
    .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(AppStateComponent(AppStates::Game))
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
                .spawn(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    transform: Transform::from_xyz(0.0, thruster.y_offset, 0.0),
                    ..Default::default()
                })
                .insert(AnimationComponent {
                    timer: Timer::from_seconds(
                        thruster.texture.frame_duration,
                        TimerMode::Repeating,
                    ),
                    direction: thruster.texture.animation_direction.clone(),
                })
                .insert(Name::new("Thruster"));
        });
    }
}
