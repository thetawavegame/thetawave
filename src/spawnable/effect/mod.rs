use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::EffectAssets,
    states::GameCleanup,
};
use bevy::{prelude::*, sprite::Anchor};
use bevy_rapier2d::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, ops::Range};
use thetawave_interface::spawnable::{EffectType, TextEffectType};

use super::InitialMotion;

mod behavior;
pub use self::behavior::effect_execute_behavior_system;
use self::behavior::EffectBehavior;

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
    pub effect_type: EffectType,
    /// Sprite texture
    pub animation: AnimationData,
    /// Behaviors specific to effects
    pub effect_behaviors: Vec<behavior::EffectBehavior>,
    /// Z level of transform
    pub z_level: f32,
}

#[derive(Deserialize, Debug)]
pub struct TextEffectData {
    pub text: String,
    pub text_color: Color,
    pub font_size: f32,
    pub translation_x: Range<f32>,
    pub translation_y: Range<f32>,
    pub scale: f32,
}

/// Resource to store data and textures of effects
#[derive(Resource)]
pub struct EffectsResource {
    /// Maps effect types to data
    pub effects: HashMap<EffectType, EffectData>,
}

#[derive(Resource)]
pub struct TextEffectsResource {
    pub text_effects: HashMap<TextEffectType, TextEffectData>,
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
    /// Send optional text to be used in some effects
    pub text: Option<String>,
}

/// Handles spawning of effects from events
pub fn spawn_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    asset_server: Res<AssetServer>,
    effects_resource: Res<EffectsResource>,
    text_effects_resource: Res<TextEffectsResource>,
    effect_assets: Res<EffectAssets>,
) {
    for event in event_reader.iter() {
        if let EffectType::Text(text_effect_type) = &event.effect_type {
            match text_effect_type {
                TextEffectType::DamageDealt => {
                    spawn_damage_text_effect(
                        event.text.clone().unwrap(),
                        event.transform,
                        &mut commands,
                        &asset_server,
                        &text_effects_resource,
                        &effects_resource,
                    );
                }
                TextEffectType::ConsumableCollected(consumable_type) => todo!(),
            }
        } else {
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
}

fn spawn_damage_text_effect(
    damage_text: String,
    transform: Transform,
    commands: &mut Commands,
    asset_server: &AssetServer,
    text_effects_resource: &TextEffectsResource,
    effects_resource: &EffectsResource,
) {
    let font = asset_server.load("fonts/wibletown-regular.otf");

    let mut rng = rand::thread_rng();

    let effect_data = &effects_resource.effects[&EffectType::Text(TextEffectType::DamageDealt)];

    // Get data from effect resource
    let text_effect_data = &text_effects_resource.text_effects[&TextEffectType::DamageDealt];

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                damage_text.clone(),
                TextStyle {
                    font: font.clone(),
                    font_size: text_effect_data.font_size,
                    color: text_effect_data.text_color,
                },
            ),
            ..default()
        })
        .insert(
            transform
                .with_translation(
                    transform.translation
                        + Vec3::new(
                            rng.gen_range(text_effect_data.translation_x.clone()),
                            rng.gen_range(text_effect_data.translation_y.clone()),
                            effect_data.z_level,
                        ),
                )
                .with_scale(Vec3 {
                    x: text_effect_data.scale,
                    y: text_effect_data.scale,
                    z: 0.0,
                }),
        )
        .insert(super::SpawnableComponent {
            spawnable_type: super::SpawnableType::Effect(EffectType::Text(
                TextEffectType::DamageDealt,
            )),
            acceleration: Vec2::new(0.0, 0.0),
            deceleration: Vec2::new(0.0, 0.0),
            speed: Vec2::new(0.0, 0.0),
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: vec![],
        })
        .insert(EffectComponent {
            effect_type: EffectType::Text(TextEffectType::DamageDealt),
            behaviors: effect_data.effect_behaviors.clone(),
        })
        .insert(GameCleanup);
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
            texture_atlas: effect_assets.get_asset(effect_type).unwrap(),
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
