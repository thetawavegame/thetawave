use crate::animation::AnimationComponent;
use crate::assets::EffectAssets;
use crate::spawnable::effect::{EffectComponent, TextEffectData, TextEffectsResource};
use crate::spawnable::{EffectsResource, InitialMotion, SpawnEffectEvent, SpawnableComponent};
use crate::states::GameCleanup;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::Rng;
use thetawave_interface::spawnable::{EffectType, SpawnableType, TextEffectType};
use thetawave_interface::states;

/// `EffectSpawnPlugin` manages the spawning of in-game effects.
///
/// This plugin is responsible for adding the system that handles the spawning of effects during the game.
pub struct EffectSpawnPlugin;

impl Plugin for EffectSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_effect_system)
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Handles spawning of effects from events
fn spawn_effect_system(
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

/// Spawn a text effect
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
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Effect(EffectType::Text(text_effect_type.clone())),
            ..default()
        })
        .insert(EffectComponent {
            effect_type: EffectType::Text(text_effect_type.clone()),
            behaviors: effect_data.effect_behaviors.clone(),
        })
        .insert(GameCleanup);
}

/// Spawn a non-text effect from effect type
fn spawn_effect(
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
        .insert(SpawnableComponent {
            spawnable_type: SpawnableType::Effect(effect_data.effect_type.clone()),
            ..default()
        })
        .insert(LockedAxes::default())
        .insert(RigidBody::KinematicVelocityBased)
        .insert(Velocity::from(initial_motion))
        .insert(effect_transform)
        .insert(GameCleanup)
        .insert(Name::new(effect_data.effect_type.to_string()));
}
