use bevy::prelude::*;
use serde::Deserialize;

use crate::{animation::AnimationComponent, spawnable::SpawnableComponent};

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum EffectBehavior {
    DespawnAfterAnimation,
}

pub fn effect_execute_behavior_system(
    mut effect_query: Query<(
        &mut SpawnableComponent,
        &super::EffectComponent,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut spawnable_component, effect_component, sprite, texture_atlas_handle) in
        effect_query.iter_mut()
    {
        let behaviors = effect_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                EffectBehavior::DespawnAfterAnimation => {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    if sprite.index as usize == texture_atlas.textures.len() - 1 {
                        spawnable_component.should_despawn = true;
                    }
                }
            }
        }
    }
}
