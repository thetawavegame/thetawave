use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::{
    options::input::{MenuAction, MenuExplorer},
    player::PlayersResource,
    states::AppStates,
};

use crate::audio;

// Start the game by entering the Game state
pub fn start_game_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    players_resource: Res<PlayersResource>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(MenuAction::Confirm) && players_resource.player_data[0].is_some()
    {
        // set the state to game
        next_app_state.set(AppStates::InitializeRun);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
    }
}

// Start the game by entering the Game state
pub fn start_instructions_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(MenuAction::Confirm) {
        // set the state to game
        next_app_state.set(AppStates::Instructions);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
    }
}

pub fn start_character_selection_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(MenuAction::Confirm) {
        // set the state to game
        next_app_state.set(AppStates::CharacterSelection);

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
    }
}
