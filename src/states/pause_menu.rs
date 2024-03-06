//! Systems to (set the game states to) open and close the pause menu.
use crate::audio;
use bevy::prelude::{AssetServer, NextState, Query, Res, ResMut, With};
use bevy_kira_audio::prelude::{AudioChannel, AudioControl};
use bevy_rapier2d::prelude::RapierConfiguration;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::states::GameStates;

pub(super) fn open_pause_menu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    let action_state = menu_input_query.single();

    // swiitch to pause menu state if input read
    if action_state.just_released(&MenuAction::PauseGame) {
        next_game_state.set(GameStates::Paused);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // suspend the physics engine
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
    }
}

// close pause menu if input given
pub(super) fn close_pause_menu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // pop the pause state if input read
    if action_state.just_released(&MenuAction::ExitPauseMenu) {
        next_game_state.set(GameStates::Playing);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // resume the physics engine
        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
    }
}
