use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::EffectAssets,
    states::GameCleanup,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use super::{EffectType, InitialMotion};

mod behavior;
pub use self::behavior::effect_execute_behavior_system;

/// Core component of effect
#[derive(Component)]
pub struct EffectComponent {
    /// Type of the effect
    pub effect_type: super::EffectType,
    /// Behaviors specific to effects
    pub behaviors: Vec<behavior::EffectBehavior>,
}

/// Data describing attributes of effects
#[derive(Deserialize)]
pub struct EffectData {
    /// Type of the effect
    pub effect_type: super::EffectType,
    /// Sprite texture
    pub animation: AnimationData,
    /// Behaviors specific to effects
    pub effect_behaviors: Vec<behavior::EffectBehavior>,
    /// Z level of transform
    pub z_level: f32,
}

/// Resource to store data and textures of effects
#[derive(Resource)]
pub struct EffectsResource {
    /// Maps effect types to data
    pub effects: HashMap<EffectType, EffectData>,
}

/// Event for spawning effect
#[derive(Event)]
pub struct SpawnEffectEvent {
    /// Type of the effect
    pub effect_type: EffectType,
    /// Position of the effect to spawn
    pub transform: Transform,
    /// Initial motion of the effect
    pub initial_motion: InitialMotion,
}

/// Handles spawning of effects from events
pub fn spawn_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    effects_resource: Res<EffectsResource>,
    effect_assets: Res<EffectAssets>,
) {
    for event in event_reader.iter() {
        spawn_effect(
            &event.effect_type,
            &effects_resource,
            &effect_assets,
            event.transform,
            event.initial_motion.clone(),
            &mut commands,
        );
    }
}

/// Spawn effect from effect type
pub fn spawn_effect(
    effect_type: &EffectType,
    effects_resource: &EffectsResource,
    effect_assets: &EffectAssets,
    transform: Transform,
    initial_motion: InitialMotion,
    commands: &mut Commands,
) {
    // Get data from effect resource
    let effect_data = &effects_resource.effects[effect_type];

    // spawn the effect
    let mut effect = commands.spawn_empty();

    let mut effect_transform = transform;
    effect_transform.translation.z = effect_data.z_level;

    effect
        .insert(SpriteSheetBundle {
            texture_atlas: effect_assets.get_asset(effect_type),
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(effect_data.animation.frame_duration, TimerMode::Repeating),
            direction: effect_data.animation.direction.clone(),
        })
        .insert(EffectComponent {
            effect_type: effect_data.effect_type.clone(),
            behaviors: effect_data.effect_behaviors.clone(),
        })
        .insert(super::SpawnableComponent {
            spawnable_type: super::SpawnableType::Effect(effect_data.effect_type.clone()),
            acceleration: Vec2::new(0.0, 0.0),
            deceleration: Vec2::new(0.0, 0.0),
            speed: Vec2::new(0.0, 0.0),
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: vec![],
        })
        .insert(LockedAxes::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::from(initial_motion))
        .insert(effect_transform)
        .insert(GameCleanup)
        .insert(Name::new(effect_data.effect_type.to_string()));
}
