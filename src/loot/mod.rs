use crate::spawnable::SpawnConsumableEvent;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

mod consumable;

pub use self::consumable::*;

/// Describes probability profiles for dropping consumables and items
#[derive(Resource, Deserialize)]
pub struct LootDropsResource {
    // Lists of consumable drops maped to types
    pub consumable_drops: HashMap<ConsumableDropListType, Vec<ConsumableLootDrop>>,
    // TODO: add items as loot drops once added into the game
    //pub item_drops: HashMap<ItemDropType, Vec<ItemLootDrop>>,
}

impl LootDropsResource {
    /// Roll for consumables from drop list
    pub fn roll_and_spawn_consumables(
        &self,
        drop_list_type: &ConsumableDropListType,
        consumable_event_writer: &mut EventWriter<SpawnConsumableEvent>,
        position: Vec2,
    ) {
        // get drops list from resource
        let drop_list = &self.consumable_drops[drop_list_type];

        // roll for each piece of loot in the drop list
        for loot_drop in drop_list.iter() {
            loot_drop.roll_and_spawn(consumable_event_writer, position);
        }
    }
}
