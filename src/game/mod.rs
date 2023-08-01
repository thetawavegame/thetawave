//! `thetawave` game module
use bevy::prelude::*;
use ron::de::from_bytes;
pub mod counters;
mod resources;

pub use self::resources::GameParametersResource;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<GameParametersResource>(include_bytes!(
                "../../assets/data/game_parameters.ron"
            ))
            .unwrap(),
        );
    }
}
