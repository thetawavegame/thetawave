use bevy::prelude::*;

mod health;
pub use self::health::{DamageDealtEvent, Health};

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>()
            .add_systems(Update, health::damage_system);
    }
}
