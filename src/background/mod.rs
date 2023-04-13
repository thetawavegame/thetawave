//! `thetawave` background module

use bevy::prelude::Commands;
use bevy::prelude::*;
use ron::de::from_bytes;

mod resources;

use crate::{states, GameEnterSet};

pub use self::resources::BackgroundsResource;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<BackgroundsResource>(include_bytes!("../../assets/data/backgrounds.ron"))
                .unwrap(),
        );

        app.add_systems(
            (create_background_system.in_set(GameEnterSet::BuildLevel),)
                .in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (rotate_planet_system,)
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );
    }
}

/// Component to manage movement of planets
#[derive(Reflect, Default, Component)]
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
