use crate::animation::AnimationComponent;
use crate::GameUpdateSet;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use serde::Deserialize;
use thetawave_interface::states;

use super::EffectComponent;

/// `EffectBehaviorPlugin` manages the behaviors of in-game effects.
///
/// This plugin is responsible for updating and managing the behaviors of effects during the game.
/// It adds systems to the app which handle the behavior of effects based on their type and state,
/// such as despawning effects after their animation completes, or fading out text and sprite effects.
pub struct EffectBehaviorPlugin;

impl Plugin for EffectBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                despawn_after_animation_effect_behavior_system
                    .in_set(GameUpdateSet::ExecuteBehavior),
                fade_out_text_effect_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                fade_out_sprite_effect_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                fade_out_despawn_after_animation_effect_behavior_system
                    .in_set(GameUpdateSet::ExecuteBehavior),
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Types of behaviors that can be performed by effects
#[derive(Deserialize, Clone)]
pub enum EffectBehavior {
    DespawnAfterAnimation,
    FadeOut(Timer),
    FadeOutAndDespawnAfterAnimation(Stopwatch),
}

/// Checks if each effect entity has a `DespawnAfterAnimation` behavior.
/// Recursively despawns the effect entities with this behavior after
/// its last animation frame is complete.
fn despawn_after_animation_effect_behavior_system(
    mut commands: Commands,
    effect_query: Query<(
        Entity,
        &EffectComponent,
        &TextureAtlasSprite,
        &AnimationComponent,
        &Handle<TextureAtlas>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    // Check if entity has  an `DespawnAfterAnimation` behavior
    for (entity, effect_component, sprite, animation, texture_atlas_handle) in effect_query.iter() {
        if effect_component
            .behaviors
            .iter()
            .any(|behavior| matches!(behavior, EffectBehavior::DespawnAfterAnimation))
        {
            // Despawn effect entity after animation is complete
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                if sprite.index == texture_atlas.textures.len() - 1
                    && animation.timer.just_finished()
                {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}

/// Checks if each effect entity with a `Text` component has a `FadeOut` behavior.
/// Recursively despawns the effect entities with this behavior after
/// the timer is complete, while also fading out linearly based on the percent of time left in the timer.
fn fade_out_text_effect_behavior_system(
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut EffectComponent, &mut Text)>,
    time: Res<Time>,
) {
    for (entity, mut effect_component, mut text) in effect_query.iter_mut() {
        if let Some(timer) = effect_component.behaviors.iter_mut().find_map(|behavior| {
            if let EffectBehavior::FadeOut(timer) = behavior {
                Some(timer)
            } else {
                None
            }
        }) {
            timer.tick(time.delta());

            // if the timer just completed, recursively despawn the effect entity, otherwise change the alpha
            if timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            } else {
                // Set alpha in all sections in the text component
                for color in text
                    .sections
                    .iter_mut()
                    .map(|section| &mut section.style.color)
                {
                    color.set_a(timer.percent_left());
                }
            }
        }
    }
}

/// Checks if each effect entity with a `TextureAtlasSprite` component has a `FadeOutMs` behavior.
/// Recursively despawns the effect entities with this behavior after
/// the timer is complete, while also fading out linearly based on the percent of time left in the timer.
fn fade_out_sprite_effect_behavior_system(
    mut commands: Commands,
    mut effect_query: Query<(Entity, &mut EffectComponent, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (entity, mut effect_component, mut sprite) in effect_query.iter_mut() {
        if let Some(timer) = effect_component.behaviors.iter_mut().find_map(|behavior| {
            if let EffectBehavior::FadeOut(timer) = behavior {
                Some(timer)
            } else {
                None
            }
        }) {
            timer.tick(time.delta());

            // if the timer just completed, recursively despawn the effect entity, otherwise change the alpha
            if timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            } else {
                sprite.color.set_a(timer.percent_left());
            }
        }
    }
}

/// Checks if each effect entity has a `FadeOutAndDespawnAfterAnimation` behavior.
/// Recursively despawns the effect entities with this behavior after
/// the animation is complete, while also fading out along an exponential decay curve.
fn fade_out_despawn_after_animation_effect_behavior_system(
    mut commands: Commands,
    mut effect_query: Query<(
        Entity,
        &mut EffectComponent,
        &mut TextureAtlasSprite,
        &AnimationComponent,
        &Handle<TextureAtlas>,
    )>,
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (entity, mut effect_component, mut sprite, animation, texture_atlas_handle) in
        effect_query.iter_mut()
    {
        if let (Some(texture_atlas), Some(stopwatch)) = (
            texture_atlases.get(texture_atlas_handle),
            effect_component.behaviors.iter_mut().find_map(|behavior| {
                if let EffectBehavior::FadeOutAndDespawnAfterAnimation(stopwatch) = behavior {
                    Some(stopwatch)
                } else {
                    None
                }
            }),
        ) {
            // Despawn if the animation is completed, otherwise continue fading out
            if sprite.index == texture_atlas.textures.len() - 1 && animation.timer.just_finished() {
                commands.entity(entity).despawn_recursive();
            } else {
                stopwatch.tick(time.delta());

                // Get the total animation time duration of a frame * the number of frames
                let total_animation_time =
                    animation.timer.duration().as_secs_f32() * texture_atlas.textures.len() as f32;

                // Get an alpha value along an exponential decay curve
                let elapsed_time = stopwatch.elapsed().as_secs_f32();
                let decay_constant = -(total_animation_time.recip()) * 0.001_f32.ln();
                let alpha = (1.0_f32 * (-decay_constant * elapsed_time).exp())
                    .max(0.0)
                    .min(1.0);

                sprite.color.set_a(alpha);
            }
        }
    }
}
