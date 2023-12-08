//! `thetawave` player module
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use ron::de::from_bytes;

use thetawave_interface::{
    input::PlayerAction,
    player::{InputRestrictionsAtSpawn, PlayersResource},
    states::{AppStates, GameStates},
};

use crate::{GameEnterSet, GameUpdateSet};

pub use self::{
    resources::CharactersResource,
    spawn::spawn_players_system,
    systems::{
        player_ability_system, player_death_system, player_fire_weapon_system,
        player_movement_system, player_scale_fire_rate_system, player_tilt_system,
        players_reset_system,
    },
};

mod resources;
mod spawn;
mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default());

        app.insert_resource(
            from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
                .unwrap(),
        );

        app.insert_resource(PlayersResource::default())
            .insert_resource(InputRestrictionsAtSpawn::default());

        app.add_systems(
            OnEnter(AppStates::Game),
            spawn_players_system.in_set(GameEnterSet::SpawnPlayer),
        );

        app.add_systems(
            Update,
            (
                player_fire_weapon_system,
                player_death_system,
                player_scale_fire_rate_system,
                player_movement_system.in_set(GameUpdateSet::Movement),
                player_tilt_system.in_set(GameUpdateSet::Movement),
                player_ability_system.in_set(GameUpdateSet::Abilities),
            )
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );

        // reset the run after exiting the end game screens and when entering the main menu
        app.add_systems(OnExit(AppStates::GameOver), players_reset_system);
        app.add_systems(OnExit(AppStates::Victory), players_reset_system);
        app.add_systems(OnEnter(AppStates::MainMenu), players_reset_system);
    }
}
