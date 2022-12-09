use super::AppStates;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::audio;

// opens pause menu if input given
pub fn open_pause_menu_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // check for keyboard or gamepad input
    let mut pause_input = keyboard_input.just_released(KeyCode::Escape);

    for gamepad in gamepads.iter() {
        pause_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        });
    }

    // swiitch to pause menu state if input read
    if pause_input {
        // push pause state
        app_state.push(AppStates::PauseMenu).unwrap();

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Escape);

        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::Start,
            });
        }

        // suspend the physics engine
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
    }
}

// close pause menu if input given
pub fn close_pause_menu_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // check for keyboard or gamepad input
    let mut unpause_input = keyboard_input.just_released(KeyCode::Escape);

    for gamepad in gamepads.iter() {
        unpause_input |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::Start,
        });
    }

    // pop the pause state if input read
    if unpause_input {
        // pop pause state
        app_state.pop().unwrap();

        // play sound effect
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset input
        keyboard_input.reset(KeyCode::Escape);

        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::Start,
            });
        }

        // resume the physics engine

        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
    }
}
