//! `thetawave_lib` player module

pub mod components;
mod spawn;
pub mod systems;

pub use self::{
    components::PlayerComponent, spawn::spawn_player_system, systems::player_movement_system,
};
