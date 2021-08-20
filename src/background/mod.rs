//! `thetawave_lib` background module

use bevy::prelude::Commands;
use bevy::prelude::*;

mod resources;

pub use self::resources::BackgroundsResource;

/// Component to manage movement of planets
#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct PlanetComponent {
    /// Speed of rotation about the z axis
    pub rotation_speed: f32,
}

/// Rotate planets about their z axis
pub fn rotate_planet_system(mut query: Query<(&mut Transform, &PlanetComponent)>) {
    for (mut transform, planet) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(planet.rotation_speed);
    }
}

/// Create background from resource
pub fn create_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    backgrounds: Res<BackgroundsResource>,
) {
    backgrounds.backgrounds["solar_system"].spawn(&mut commands, &asset_server);
}
