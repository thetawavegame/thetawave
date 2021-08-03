//! `thetawave_lib` player module

pub mod components;
mod resources;
mod spawn;
pub mod systems;

pub use self::{
    components::PlayerComponent,
    resources::{Character, CharactersResource},
    spawn::spawn_player_system,
    systems::player_movement_system,
};
