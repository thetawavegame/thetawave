use self::behavior::EffectBehaviorData;

use super::InitialMotion;
use crate::animation::AnimationData;
use crate::spawnable::effect::behavior::EffectBehaviorPlugin;
use crate::spawnable::effect::spawn::EffectSpawnPlugin;
use bevy::prelude::*;
use ron::de::from_bytes;
use serde::Deserialize;
use std::{collections::HashMap, ops::Range};
use thetawave_interface::spawnable::{EffectType, TextEffectType};

mod behavior;
mod spawn;

/// `EffectPlugin` is responsible for managing and spawning in-game effects.
///
/// This plugin encapsulates all functionalities related to effect spawnables within the game.
/// It loads effect data from external RON files, registers necessary events and resources,
/// and initializes other plugins for managing effects.
pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EffectBehaviorPlugin, EffectSpawnPlugin))
            .add_event::<SpawnEffectEvent>()
            .insert_resource(EffectsResource {
                effects: from_bytes::<HashMap<EffectType, EffectData>>(include_bytes!(
                    "../../../assets/data/effects.ron"
                ))
                .expect("Failed to parse EffectsResource from 'effects.ron'"),
            })
            .insert_resource(TextEffectsResource {
                text_effects: from_bytes::<HashMap<TextEffectType, TextEffectData>>(
                    include_bytes!("../../../assets/data/text_effects.ron"),
                )
                .expect("Failed to parse TextEffectsResource from 'text_effects.ron'"),
            });
    }
}

/// Core component of effect
#[derive(Component)]
pub struct EffectComponent {
    /// Type of the effect
    pub effect_type: EffectType,
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
    pub effect_behaviors_data: Vec<EffectBehaviorData>,
    /// Z level of transform
    pub z_level: f32,
}

impl From<&EffectData> for EffectComponent {
    fn from(value: &EffectData) -> Self {
        EffectComponent {
            effect_type: value.effect_type.clone(),
            behaviors: value
                .effect_behaviors_data
                .clone()
                .into_iter()
                .map(|data| data.into())
                .collect(),
        }
    }
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
