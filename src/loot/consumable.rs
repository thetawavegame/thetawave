use bevy::prelude::*;
use rand::Rng;
use serde::Deserialize;
use strum_macros::Display;
use thetawave_interface::spawnable::ConsumableType;

use crate::spawnable::SpawnConsumableEvent;

/// Types of consumable drop lists
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum DropListType {
    #[default]
    Nothing,
    Standard,
    MoneyAsteroid,
    /// Some of the better drops that the play should need to work for.
    Boss,
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
    /// Roll for loot drops and spawn consumables
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
