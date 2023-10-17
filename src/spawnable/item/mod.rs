//! Items are like consumables but have a permanent effect. These will also be purchasable.
use std::collections::HashMap;

use bevy::prelude::*;
use ron::de::from_bytes;
use serde::Deserialize;
use thetawave_interface::spawnable::{ItemComponent, ItemType, SpawnItemEvent, SpawnableType};

use crate::animation::AnimationData;

use self::{
    behavior::{ItemBehavior, ItemBehaviorPlugin},
    spawn::ItemSpawnPlugin,
};

use super::{InitialMotion, SpawnableBehavior, SpawnableComponent};

mod behavior;
mod spawn;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ItemSpawnPlugin, ItemBehaviorPlugin))
            .add_event::<SpawnItemEvent>()
            .insert_resource(ItemResource {
                items: from_bytes::<HashMap<ItemType, ItemData>>(include_bytes!(
                    "../../../assets/data/items.ron"
                ))
                .expect("Failed to parse ItemsResource from 'items.ron'"),
            });
    }
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

impl From<&ItemData> for ItemComponent {
    fn from(item_data: &ItemData) -> Self {
        ItemComponent {
            item_type: item_data.item_type.clone(),
        }
    }
}

impl From<&ItemData> for SpawnableComponent {
    fn from(item_data: &ItemData) -> Self {
        SpawnableComponent {
            spawnable_type: SpawnableType::Item(item_data.item_type.clone()),
            acceleration: item_data.acceleration,
            deceleration: item_data.deceleration,
            speed: item_data.speed,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: item_data.spawnable_behaviors.clone(),
        }
    }
}
