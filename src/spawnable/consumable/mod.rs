use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    game::GameParametersResource,
    spawnable::{ConsumableType, InitialMotion},
};

#[derive(Component)]
pub struct ConsumableComponent {
    pub consumable_type: ConsumableType,
}

#[derive(Deserialize)]
pub struct ConsumableData {
    pub consumable_type: ConsumableType,
}

pub struct ConsumableResource {
    pub consumables: HashMap<ConsumableType, ConsumableData>,
    pub texture_atlas_handle: HashMap<ConsumableType, Handle<TextureAtlas>>,
}

pub fn spawn_consumable(
    consumable_type: &ConsumableType,
    consumable_resource: &ConsumableResource,
    position: Vec2,
    initial_motion: InitialMotion,
    commands: &mut Commands,
    game_parameters: &GameParametersResource,
) {
}
