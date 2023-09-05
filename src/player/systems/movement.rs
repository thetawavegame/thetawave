use std::collections::HashMap;

use crate::{
    game::GameParametersResource,
    player::{PlayerComponent, PlayersResource},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use thetawave_interface::player::PlayerInput;

/// Move player by modifying velocity with input
pub fn player_movement_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    players_resource: Res<PlayersResource>,
    game_parameters: Res<GameParametersResource>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity)>,
) {
    let up_keyboard_input =
        keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    let down_keyboard_input =
        keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    let left_keyboard_input =
        keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right_keyboard_input =
        keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    let up_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadUp,
                }),
            )
        })
        .collect();

    let down_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadDown,
                }),
            )
        })
        .collect();

    let left_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadLeft,
                }),
            )
        })
        .collect();

    let right_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadRight,
                }),
            )
        })
        .collect();

    for (player, mut vel) in player_info.iter_mut() {
        let player_input = players_resource.player_inputs[player.player_index]
            .clone()
            .unwrap();

        let up = match player_input {
            PlayerInput::Keyboard => up_keyboard_input,
            PlayerInput::Gamepad(gamepad) => up_gamepad_inputs[&gamepad],
        };

        let down = match player_input {
            PlayerInput::Keyboard => down_keyboard_input,
            PlayerInput::Gamepad(gamepad) => down_gamepad_inputs[&gamepad],
        };

        let left = match player_input {
            PlayerInput::Keyboard => left_keyboard_input,
            PlayerInput::Gamepad(gamepad) => left_gamepad_inputs[&gamepad],
        };

        let right = match player_input {
            PlayerInput::Keyboard => right_keyboard_input,
            PlayerInput::Gamepad(gamepad) => right_gamepad_inputs[&gamepad],
        };

        if player.movement_enabled {
            // convert to axis multipliers
            let x_axis = -(left as i8) + right as i8;
            let y_axis = -(down as i8) + up as i8;

            // handle movement in x direction
            if x_axis != 0 {
                // accelerate to the player's maximum speed stat
                vel.linvel.x += player.acceleration.x * (x_axis as f32);
                if vel.linvel.x.abs() > player.speed.x {
                    vel.linvel.x = (vel.linvel.x / vel.linvel.x.abs()) * player.speed.x;
                }
            } else if vel.linvel.x.abs() > game_parameters.stop_threshold {
                // decelerate
                vel.linvel.x -= player.deceleration.x * (vel.linvel.x / vel.linvel.x.abs());
            } else {
                vel.linvel.x = 0.0;
            }

            // handle movement in y direction
            if y_axis != 0 {
                // accelerate to the player's maximum speed stat
                vel.linvel.y += player.acceleration.y * (y_axis as f32);
                if vel.linvel.y.abs() > player.speed.y {
                    vel.linvel.y = (vel.linvel.y / vel.linvel.y.abs()) * player.speed.y;
                }
            } else if vel.linvel.y.abs() > game_parameters.stop_threshold {
                // decelerate
                vel.linvel.y -= player.deceleration.y * (vel.linvel.y / vel.linvel.y.abs());
            } else {
                vel.linvel.y = 0.0;
            }
        }
    }
}
