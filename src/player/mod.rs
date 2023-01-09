//! `thetawave` player module

mod components;
mod resources;
mod spawn;
mod systems;

pub use self::{
    components::PlayerComponent,
    resources::{Character, CharacterType, CharactersResource},
    spawn::spawn_player_system,
    systems::{
        player_death_system, player_fire_weapon_system, player_movement_system,
        player_scale_fire_rate_system,
    },
};
