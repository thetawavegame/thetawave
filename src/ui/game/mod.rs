use bevy::{
    app::{Plugin, Update},
    ecs::schedule::IntoSystemConfigs,
    prelude::{in_state, OnEnter},
};
use thetawave_interface::states;

use crate::GameEnterSet;

mod border_gradient;
mod game_center;
mod level;
mod parent;
mod phase;
mod player;

pub(super) struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<border_gradient::BorderGradientEvent>();

        app.add_systems(
            OnEnter(states::AppStates::Game),
            parent::setup_game_ui_system.after(GameEnterSet::BuildUi),
        );

        app.add_systems(
            Update,
            (
                player::update_player_health_ui_system,
                player::update_player_shields_ui_system,
                player::update_player_armor_ui_system,
                player::update_player_abilities_ui_system,
                phase::update_phase_ui_system,
                level::update_level_ui_system,
                game_center::update_center_text_ui_system,
                game_center::text_fade_out_system,
                border_gradient::border_gradient_start_system,
                border_gradient::border_gradient_update_system,
                border_gradient::border_gradient_on_gate_interaction_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}
