use bevy::prelude::*;
use thetawave_interface::health::DamageDealtEvent;

mod health;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageDealtEvent>().add_systems(
            Update,
            (health::damage_system, health::regenerate_shields_system),
        );
    }
}
