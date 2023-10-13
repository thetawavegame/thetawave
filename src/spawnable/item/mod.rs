use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;
use thetawave_interface::spawnable::ItemType;

use crate::animation::AnimationData;

use self::behavior::ItemBehavior;

use super::{InitialMotion, SpawnableBehavior};

mod behavior;
mod spawn;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct ItemComponent {
    pub item_type: ItemType,
    pub item_effects: Vec<ItemEffect>,
    pub behaviors: Vec<ItemBehavior>,
}

#[derive(Deserialize)]
pub enum ItemEffect {
    GainDamage(usize),
    GainHealth(usize),
    GainFireRate(f32),
}

#[derive(Resource)]
pub struct ItemResource {
    /// Maps consumable types to data
    pub items: HashMap<ItemType, ItemData>,
}

/// Data describing items
#[derive(Deserialize)]
pub struct ItemData {
    /// Type of the item
    pub item_type: ItemType,
    /// Dimensions of the collider
    pub collider_dimensions: Vec2,
    /// Spawnable generic behaviors
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// Texture of the item
    pub animation: AnimationData,
    /// Initial motion of the item
    pub initial_motion: InitialMotion,
    /// Effects of picking up the item
    pub item_effects: Vec<ItemEffect>,
    /// Item specific behaviors
    pub item_behaviors: Vec<ItemBehavior>,
    /// Maximum speed
    pub speed: Vec2,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// z value of the transform
    pub z_level: f32,
}
