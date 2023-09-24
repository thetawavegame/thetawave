use bevy::prelude::*;
use serde::Deserialize;

use super::EffectComponent;

/// Types of behaviors that can be performed by effects
#[derive(Deserialize, Clone)]
pub enum EffectBehavior {
    DespawnAfterAnimation,
    FadeOutMs(Timer),
}

/// Execute behaviors specific to effects
#[allow(clippy::complexity)]
pub fn effect_execute_behavior_system(
    mut commands: Commands,
    mut effect_query: Query<(
        Entity,
        &mut EffectComponent,
        Option<&TextureAtlasSprite>,
        Option<&Handle<TextureAtlas>>,
        Option<&mut Text>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    time: Res<Time>,
) {
    for (entity, mut effect_component, sprite, texture_atlas_handle, mut text) in
        effect_query.iter_mut()
    {
        for behavior in effect_component.behaviors.iter_mut() {
            match behavior {
                EffectBehavior::DespawnAfterAnimation => {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle.unwrap()).unwrap();
                    if sprite.unwrap().index == texture_atlas.textures.len() - 1 {
                        commands.entity(entity).despawn_recursive();
                    }
                }
                EffectBehavior::FadeOutMs(timer) => {
                    timer.tick(time.delta());

                    // if the effect has a text field set the alpha to the percent left in the timer
                    if let Some(text) = text.as_mut() {
                        if let Some(section) = text.sections.get_mut(0) {
                            section.style.color.set_a(timer.percent_left());
                        }
                    }

                    // despawn the effect entity when the timer is finished
                    if timer.just_finished() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

/// Checks if each effect entity has a DespawnAfterAnimation behavior.
/// Recursively despawns the effect entities with this behavior after
/// its last animation frame is complete.
pub fn despawn_after_animation_effect_behavior_system(
    mut commands: Commands,
    effect_query: Query<(
        Entity,
        &EffectComponent,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    // check if entity has  an `DespawnAfterAnimation` behavior
    for (entity, effect_component, sprite, texture_atlas_handle) in effect_query.iter() {
        if effect_component
            .behaviors
            .iter()
            .any(|behavior| matches!(behavior, EffectBehavior::DespawnAfterAnimation))
        {
            // despawn effect entity after animation is complete
            if let Some(texture_atlas) = texture_atlases.get(texture_atlas_handle) {
                if sprite.index == texture_atlas.textures.len() - 1 {
                    commands.entity(entity).despawn_recursive();
                }
            }
        }
    }
}
