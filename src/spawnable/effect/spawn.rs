use crate::animation::AnimationComponent;
use crate::assets::{EffectAssets, UiAssets};
use crate::spawnable::effect::{EffectComponent, TextEffectData, TextEffectsResource};
use crate::spawnable::{EffectsResource, InitialMotion, SpawnEffectEvent, SpawnableComponent};
use bevy::color::Color;
use bevy::prelude::{
    in_state, App, Commands, EventReader, IntoSystemConfigs, Name, Plugin, Res, Sprite, Text,
    Text2dBundle, TextStyle, Timer, TimerMode, Transform, Update, Vec3,
};
use bevy::sprite::{SpriteBundle, TextureAtlas};
use bevy::utils::default;
use bevy_rapier2d::prelude::{LockedAxes, RigidBody, Velocity};
use rand::Rng;
use thetawave_interface::game::options::GameOptions;
use thetawave_interface::spawnable::{EffectType, SpawnableType, TextEffectType};
use thetawave_interface::states;
use thetawave_interface::states::GameCleanup;
/// `EffectSpawnPlugin` manages the spawning of in-game effects.
///
/// This plugin is responsible for adding the system that handles the spawning of effects during the game.
pub struct EffectSpawnPlugin;

impl Plugin for EffectSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_effect_system, spawn_text_effect_system)
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Handles the spawning of visual effects based on `SpawnEffectEvent` events.
///
/// This system iterates through each `SpawnEffectEvent`, and if the event specifies a non-text effect,
/// it triggers the spawning of the corresponding effect.
fn spawn_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    effects_resource: Res<EffectsResource>,
    effect_assets: Res<EffectAssets>,
    game_options: Res<GameOptions>,
) {
    for event in event_reader.read() {
        if !matches!(event.effect_type, EffectType::Text(..)) {
            spawn_effect(
                &event.effect_type,
                &effects_resource,
                &effect_assets,
                event.transform,
                event.initial_motion.clone(),
                &mut commands,
                &game_options,
            );
        }
    }
}

/// Handles the spawning of text effects based on `SpawnEffectEvent` events.
///
/// This system iterates through each `SpawnEffectEvent`, and if the event specifies a text effect,
/// it triggers the spawning of the corresponding text effect.
fn spawn_text_effect_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnEffectEvent>,
    effects_resource: Res<EffectsResource>,
    text_effects_resource: Res<TextEffectsResource>,
    ui_assets: Res<UiAssets>,
) {
    for event in event_reader.read() {
        if let EffectType::Text(text_effect_type) = &event.effect_type {
            spawn_text_effect(
                event.text.clone(),
                text_effect_type,
                event.transform,
                &mut commands,
                &text_effects_resource,
                &effects_resource,
                &ui_assets,
            );
        }
    }
}

/// Creates and spawns a text effect entity based on the provided parameters.
///
/// This function constructs a text effect entity with the specified text, effect type, and transform,
/// and adds it to the ECS world.
fn spawn_text_effect(
    effect_text: Option<String>,
    text_effect_type: &TextEffectType,
    transform: Transform,
    commands: &mut Commands,
    text_effects_resource: &TextEffectsResource,
    effects_resource: &EffectsResource,
    ui_assets: &UiAssets,
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
            font: ui_assets.lunchds_font.clone(),
            font_size: text_effect_data.font_size,
            color: Color::Srgba(text_effect_data.text_color),
        },
    );

    // spawn text effect entity
    commands
        .spawn(Text2dBundle {
            text,
            ..Default::default()
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
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Effect(EffectType::Text(text_effect_type.clone())),
            ..Default::default()
        })
        .insert(EffectComponent::from(effect_data))
        .insert(GameCleanup);
}

/// Creates and spawns a non-text effect entity based on the provided parameters.
///
/// This function constructs a non-text effect entity with the specified effect type, transform, and initial motion,
/// and adds it to the ECS world.
fn spawn_effect(
    effect_type: &EffectType,
    effects_resource: &EffectsResource,
    effect_assets: &EffectAssets,
    transform: Transform,
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_options: &GameOptions,
) {
    // Get data from effect resource
    let effect_data = &effects_resource.effects[effect_type];

    // spawn the effect
    let mut effect = commands.spawn_empty();

    let mut effect_transform = transform;
    effect_transform.translation.z = effect_data.z_level;

    effect
        .insert(SpriteBundle {
            texture: effect_assets.get_image(effect_type).unwrap_or_default(),
            sprite: Sprite {
                color: effect_data.affine_bloom_transformation(game_options.bloom_intensity),
                ..Default::default()
            },
            ..default()
        })
        .insert(TextureAtlas {
            layout: effect_assets
                .get_texture_atlas_layout(effect_type)
                .unwrap_or_default(),
            ..default()
        })
        .insert(AnimationComponent {
            timer: Timer::from_seconds(effect_data.animation.frame_duration, TimerMode::Repeating),
            direction: effect_data.animation.direction.clone(),
        })
        .insert(EffectComponent::from(effect_data))
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Effect(effect_data.effect_type.clone()),
            ..Default::default()
        })
        .insert(LockedAxes::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::from(initial_motion))
        .insert(effect_transform)
        .insert(GameCleanup)
        .insert(Name::new(effect_data.effect_type.to_string()));
}
