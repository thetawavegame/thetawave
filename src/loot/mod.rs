use bevy::prelude::*;
use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::spawnable::{ItemType, SpawnItemEvent};

mod consumable;

use crate::spawnable::SpawnConsumableEvent;

pub use self::consumable::*;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<LootDropsResource>(include_bytes!("../../assets/data/loot_drops.ron"))
                .unwrap(),
        );
    }
}

/// Describes probability profiles for dropping consumables and items
#[derive(Resource, Deserialize)]
pub struct LootDropsResource {
    // Lists of consumable drops mapped to types
    pub drops: HashMap<DropListType, Vec<LootDrop>>,
}

#[derive(Deserialize)]
pub enum LootDrop {
    Consumable(ConsumableLootDrop),
    Item(ItemType),
}

impl LootDropsResource {
    /// Roll for consumables from drop list
    pub fn spawn_loot_drops(
        &self,
        drop_list_type: &DropListType,
        consumable_event_writer: &mut EventWriter<SpawnConsumableEvent>,
        item_event_writer: &mut EventWriter<SpawnItemEvent>,
        position: Vec2,
    ) {
        // get drops list from resource
        let drop_list = &self.drops[drop_list_type];

        // roll for each piece of loot in the drop list
        for loot_drop in drop_list.iter() {
            match loot_drop {
                LootDrop::Consumable(consumable_loot_drop) => {
                    consumable_loot_drop.roll_and_spawn(consumable_event_writer, position);
                }
                LootDrop::Item(item_type) => {
                    item_event_writer.send(SpawnItemEvent {
                        item_type: item_type.clone(),
                        position,
                    });
                }
            }
        }
    }
}
