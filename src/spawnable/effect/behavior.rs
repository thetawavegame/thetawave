use bevy::prelude::*;
use serde::Deserialize;

use super::EffectComponent;

/// Types of behaviors that can be performed by effects
#[derive(Deserialize, Clone)]
pub enum EffectBehavior {
    DespawnAfterAnimation,
}

/// Execute behaviors specific to effects
pub fn effect_execute_behavior_system(
    mut commands: Commands,
    mut effect_query: Query<(
        Entity,
        &EffectComponent,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (entity, effect_component, sprite, texture_atlas_handle) in effect_query.iter_mut() {
        let behaviors = effect_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                EffectBehavior::DespawnAfterAnimation => {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    if sprite.index == texture_atlas.textures.len() - 1 {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}
