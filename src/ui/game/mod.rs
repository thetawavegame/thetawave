use bevy::{
    app::{Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
};
use thetawave_interface::states;

use crate::GameEnterSet;

mod game_center;
mod level;
mod parent;
mod phase;
mod player;

pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(states::AppStates::Game),
            parent::setup_game_ui_system.after(GameEnterSet::BuildUi),
        );

        app.add_systems(
            Update,
            (
                player::update_player_ui_system,
                phase::update_phase_ui_system,
                level::update_level_ui_system,
                game_center::update_center_text_ui_system,
                game_center::text_fade_out_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}
