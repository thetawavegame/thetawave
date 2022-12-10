use crate::{
    animation::{AnimationComponent, AnimationData, TextureData},
    assets::EffectAssets,
    game::GameParametersResource,
    states::{AppStateComponent, AppStates},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use super::EffectType;

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
    /// Maps effect types to textures
    pub texture_atlas_handle: HashMap<EffectType, Handle<TextureAtlas>>,
}

/// Event for spawning effect
pub struct SpawnEffectEvent {
    /// Type of the effect
    pub effect_type: EffectType,
    /// Position of the effect to spawn
    pub position: Vec2,
    /// Scale of the effect to spawn
    pub scale: Vec2,
    /// Rotation of the effect to spawn
    pub rotation: f32,
}

/// Handles spawning of effects from events
pub fn spawn_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    effects_resource: Res<EffectsResource>,
    effect_assets: Res<EffectAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_effect(
            &event.effect_type,
            &effects_resource,
            &effect_assets,
            event.position,
            event.scale,
            event.rotation,
            &mut commands,
            &game_parameters,
        );
    }
}

/// Spawn effect from effect type
pub fn spawn_effect(
    effect_type: &EffectType,
    effects_resource: &EffectsResource,
    effect_assets: &EffectAssets,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from effect resource
    let effect_data = &effects_resource.effects[effect_type];

    // spawn the effect
    let mut effect = commands.spawn_empty();

    effect
        .insert(SpriteSheetBundle {
            texture_atlas: effect_assets.get_asset(effect_type).clone(),
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
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from_transform(Transform {
            translation: position.extend(effect_data.z_level),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::new(
                game_parameters.sprite_scale + scale.x,
                game_parameters.sprite_scale + scale.y,
                1.0,
            ),
        }))
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new(effect_data.effect_type.to_string()));
}
