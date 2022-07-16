use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use strum_macros::Display;

use crate::spawnable::{ConsumableType, SpawnConsumableEvent};

/// Describes probability profiles for dropping consumables and items
#[derive(Deserialize)]
pub struct LootDropsResource {
    // Lists of consumable drops maped to types
    pub consumable_drops: HashMap<ConsumableDropListType, Vec<ConsumableLootDrop>>,
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
        let drop_list = &self.consumable_drops[drop_list_type];

        for loot_drop in drop_list.iter() {
            loot_drop.roll_and_spawn(consumable_event_writer, position);
        }
    }
}

/// Types of consumable drop lists
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum ConsumableDropListType {
    Nothing,
    Standard,
    MoneyAsteroid,
}

/// Probability profile for a single consumable drop
/// Number of rolls, probability per roll and the consumable to drop on a successful roll
#[derive(Deserialize)]
pub struct ConsumableLootDrop {
    pub rolls: u32,
    pub probability: f64,
    pub consumable: ConsumableType,
}

impl ConsumableLootDrop {
    pub fn roll_and_spawn(
        &self,
        consumable_event_writer: &mut EventWriter<SpawnConsumableEvent>,
        position: Vec2,
    ) {
        let mut rng = rand::thread_rng();

        // roll specified amount of times
        for _ in 0..self.rolls {
            // roll using the probability
            let roll = rng.gen_bool(self.probability);

            if roll {
                // spawn consumable if roll is successful
                consumable_event_writer.send(SpawnConsumableEvent {
                    consumable_type: self.consumable.clone(),
                    position,
                });
            }
        }
    }
}
