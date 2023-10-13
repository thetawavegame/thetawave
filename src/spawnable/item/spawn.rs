use bevy::prelude::*;
use thetawave_interface::spawnable::ItemType;

use crate::{assets::ItemAssets, game::GameParametersResource};

#[derive(Event)]
pub struct SpawnItemEvent {
    pub item_type: ItemType,
    pub position: Vec2,
}

pub fn spawn_item_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnItemEvent>,
    item_resource: Res<ItemResource>,
    item_assets: Res<ItemAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        todo!("Spawn item");
    }
}
