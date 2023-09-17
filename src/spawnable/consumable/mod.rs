use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::{
    spawnable::{ConsumableType, SpawnableType},
    states::GameCleanup,
};

use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::ConsumableAssets,
    game::GameParametersResource,
    spawnable::{InitialMotion, SpawnableBehavior, SpawnableComponent},
};

mod behavior;

pub use self::behavior::{consumable_execute_behavior_system, ConsumableBehavior};

/// All the different consumable effects
#[derive(Deserialize, Clone)]
pub enum ConsumableEffect {
    GainHealth(usize),
    GainArmor(usize),
    GainMoney(usize),
}

/// Core component for a consumable
#[derive(Component)]
pub struct ConsumableComponent {
    /// Type of the consumable
    pub consumable_type: ConsumableType,
    /// Collection of the consumable effects
    pub consumable_effects: Vec<ConsumableEffect>,
    /// Consumable specific behaviors
    pub behaviors: Vec<ConsumableBehavior>,
}

/// Event for spawning a consumable
#[derive(Event)]
pub struct SpawnConsumableEvent {
    /// Type of the consumable to spawn
    pub consumable_type: ConsumableType,
    /// Position of the consumable to spawn
    pub position: Vec2,
}

/// Handles spawning of consumables according to read events
pub fn spawn_consumable_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnConsumableEvent>,
    consumables_resource: Res<ConsumableResource>,
    consumable_assets: Res<ConsumableAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_consumable(
            &event.consumable_type,
            &consumables_resource,
            &consumable_assets,
            event.position,
            &mut commands,
            &game_parameters,
        );
    }
}

/// Data describing consumables
#[derive(Deserialize)]
pub struct ConsumableData {
    /// Type of the consumable
    pub consumable_type: ConsumableType,
    /// Dimensions of the collider
    pub collider_dimensions: Vec2,
    /// Spawnable generic behaviors
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// Texture of the consumable
    pub animation: AnimationData,
    /// Initial motion of the consuimable
    pub initial_motion: InitialMotion,
    /// Effects of picking up the consumable
    pub consumable_effects: Vec<ConsumableEffect>,
    /// Consumable specific behaviors
    pub consumable_behaviors: Vec<ConsumableBehavior>,
    /// Maximum speed
    pub speed: Vec2,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// z value of the transform
    pub z_level: f32,
}

/// Consumable resource stores data about all consumables
#[derive(Resource)]
pub struct ConsumableResource {
    /// Maps consumable types to data
    pub consumables: HashMap<ConsumableType, ConsumableData>,
}

/// Spawn a consumable by type
pub fn spawn_consumable(
    consumable_type: &ConsumableType,
    consumable_resource: &ConsumableResource,
    consumable_assets: &ConsumableAssets,
    position: Vec2,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    //Get data from the consumable resource
    let consumable_data = &consumable_resource.consumables[consumable_type];

    // Scale collider to align with the sprite
    let collider_size_hx =
        consumable_data.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy =
        consumable_data.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // Create consumable entity
    let mut consumable = commands.spawn_empty();

    // spawn the consumable
    consumable
        .insert(SpriteSheetBundle {
            texture_atlas: consumable_assets.get_asset(consumable_type),
            sprite: TextureAtlasSprite {
                color: consumable_assets.get_color(consumable_type),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(
                consumable_data.animation.frame_duration,
                TimerMode::Repeating,
            ),
            direction: consumable_data.animation.direction.clone(),
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Velocity::from(consumable_data.initial_motion.clone()))
        .insert(Transform {
            translation: position.extend(consumable_data.z_level),
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Sensor)
        .insert(ConsumableComponent {
            consumable_type: consumable_data.consumable_type.clone(),
            consumable_effects: consumable_data.consumable_effects.clone(),
            behaviors: consumable_data.consumable_behaviors.clone(),
        })
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Consumable(consumable_data.consumable_type.clone()),
            acceleration: consumable_data.acceleration,
            deceleration: consumable_data.deceleration,
            speed: consumable_data.speed,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: consumable_data.spawnable_behaviors.clone(),
        })
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(GameCleanup)
        .insert(Name::new(consumable_data.consumable_type.to_string()));
}
