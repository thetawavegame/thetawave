//! `thetawave` player module

mod components;
mod resources;
mod spawn;
mod systems;

pub use self::{
    components::PlayerComponent,
    resources::{Character, CharactersResource},
    spawn::spawn_player_system,
    systems::player_movement_system,
};
