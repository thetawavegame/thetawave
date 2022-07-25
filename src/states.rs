use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rapier2d::prelude::*;

use crate::MenuAudioChannel;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    MainMenu,
    PauseMenu,
    Game,
    GameOver,
    Victory,
}

// used for tagging entities that are part of the game state
#[derive(Component)]
pub struct AppStateComponent(pub AppStates);

pub fn open_pause_menu_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<MenuAudioChannel>>,
) {
    let mut esc = keyboard_input.just_released(KeyCode::Escape);

    for gamepad in gamepads.iter() {
        esc |= gamepad_input.just_released(GamepadButton(*gamepad, GamepadButtonType::Start));
    }

    if esc {
        app_state.push(AppStates::PauseMenu).unwrap();
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
        keyboard_input.reset(KeyCode::Escape);
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton(*gamepad, GamepadButtonType::Start));
        }
    }
}

pub fn close_pause_menu_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<MenuAudioChannel>>,
) {
    let mut esc = keyboard_input.just_released(KeyCode::Escape);

    for gamepad in gamepads.iter() {
        esc |= gamepad_input.just_released(GamepadButton(*gamepad, GamepadButtonType::Start));
    }

    if esc {
        app_state.pop().unwrap();
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
        keyboard_input.reset(KeyCode::Escape);
        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton(*gamepad, GamepadButtonType::Start));
        }
    }
}

pub fn start_game_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<MenuAudioChannel>>,
) {
    let mut enter = keyboard_input.just_released(KeyCode::Return);

    for gamepad in gamepads.iter() {
        enter |= gamepad_input.just_released(GamepadButton(*gamepad, GamepadButtonType::South));
    }

    if enter {
        app_state.set(AppStates::Game).unwrap();
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
        keyboard_input.release(KeyCode::Return);
        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton(*gamepad, GamepadButtonType::South));
        }
    }
}
