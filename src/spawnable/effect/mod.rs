use crate::{
    animation::{AnimationComponent, AnimationData},
    assets::EffectAssets,
    states::GameCleanup,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::{collections::HashMap, ops::Range};
use thetawave_interface::spawnable::{EffectType, TextEffectType};

use super::InitialMotion;

mod behavior;

pub use behavior::{
    despawn_after_animation_effect_behavior_system,
    fade_out_despawn_after_animation_effect_behavior_system,
    fade_out_sprite_effect_behavior_system, fade_out_text_effect_behavior_system,
};

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
    /// Text of the effect
    pub text: String,
    /// Color of the text
    pub text_color: Color,
    /// Font size pf the text
    pub font_size: f32,
    /// X position range (randomly chosen)
    pub translation_x: Range<f32>,
    /// Y position range (randomly chosen)
    pub translation_y: Range<f32>,
    /// Scale of the text
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
    /// Maps text effect types to data
    pub text_effects: HashMap<TextEffectType, TextEffectData>,
}

/// Event for spawning effect
#[derive(Event, Default)]
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
            spawn_text_effect(
                event.text.clone(),
                text_effect_type,
                event.transform,
                &mut commands,
                &asset_server,
                &text_effects_resource,
                &effects_resource,
            );
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

// spawn a text effect
fn spawn_text_effect(
    effect_text: Option<String>,
    text_effect_type: &TextEffectType,
    transform: Transform,
    commands: &mut Commands,
    asset_server: &AssetServer,
    text_effects_resource: &TextEffectsResource,
    effects_resource: &EffectsResource,
) {
    let mut rng = rand::thread_rng();

    // get data specific to the text effect
    let effect_data = &effects_resource.effects[&EffectType::Text(text_effect_type.clone())];
    let text_effect_data: &TextEffectData = &text_effects_resource.text_effects[text_effect_type];

    // create text
    let text = Text::from_section(
        match text_effect_type {
            TextEffectType::DamageDealt => effect_text.unwrap_or("0".to_string()),

            TextEffectType::ConsumableCollected(_) => text_effect_data.text.clone(),
        },
        TextStyle {
            font: asset_server.load("fonts/wibletown-regular.otf"),
            font_size: text_effect_data.font_size,
            color: text_effect_data.text_color,
        },
    );

    // spawn text effect entity
    commands
        .spawn(Text2dBundle { text, ..default() })
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
                text_effect_type.clone(),
            )),
            ..default()
        })
        .insert(EffectComponent {
            effect_type: EffectType::Text(text_effect_type.clone()),
            behaviors: effect_data.effect_behaviors.clone(),
        })
        .insert(GameCleanup);
}

/// Spawn a non-text effect from effect type
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
            texture_atlas: effect_assets.get_asset(effect_type).unwrap_or_default(),
            sprite: TextureAtlasSprite {
                color: effect_assets.get_color(effect_type),
                ..Default::default()
            },
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
            ..default()
        })
        .insert(LockedAxes::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::from(initial_motion))
        .insert(effect_transform)
        .insert(GameCleanup)
        .insert(Name::new(effect_data.effect_type.to_string()));
}
