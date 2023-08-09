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
                    if let Some(text) = text.as_mut().unwrap().sections.get_mut(0) {
                        // set alpha color channel to the percent left in the timer
                        text.style.color.set_a(timer.percent_left());
                    }
                    if timer.just_finished() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
