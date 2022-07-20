use crate::{
    animation::{AnimationComponent, TextureData},
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

#[derive(Component)]
pub struct EffectComponent {
    pub effect_type: super::EffectType,
    pub behaviors: Vec<behavior::EffectBehavior>,
}

#[derive(Deserialize)]
pub struct EffectData {
    pub effect_type: super::EffectType,
    pub texture: TextureData,
    pub effect_behaviors: Vec<behavior::EffectBehavior>,
    pub z_level: f32,
}

pub struct EffectsResource {
    pub effects: HashMap<EffectType, EffectData>,
    pub texture_atlas_handle: HashMap<EffectType, Handle<TextureAtlas>>,
}

pub struct SpawnEffectEvent {
    pub effect_type: EffectType,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

pub fn spawn_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    effects_resource: Res<EffectsResource>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        spawn_effect(
            &event.effect_type,
            &effects_resource,
            event.position,
            event.scale,
            event.rotation,
            &mut commands,
            &game_parameters,
        );
    }
}

pub fn spawn_effect(
    effect_type: &EffectType,
    effects_resource: &EffectsResource,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
    // Get data from effect resource
    let effect_data = &effects_resource.effects[effect_type];
    let texture_atlas_handle = effects_resource.texture_atlas_handle[effect_type].clone_weak();

    let mut effect = commands.spawn();

    effect
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            ..Default::default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(effect_data.texture.frame_duration, true),
            direction: effect_data.texture.animation_direction.clone(),
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
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::from_transform(Transform {
            translation: position.extend(effect_data.z_level),
            rotation: Quat::from_rotation_z(rotation),
            scale: Vec3::new(
                game_parameters.sprite_scale + scale.x,
                game_parameters.sprite_scale + scale.y,
                1.0,
            ),
            ..Default::default()
        }))
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new(effect_data.effect_type.to_string()));
}
