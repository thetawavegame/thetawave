use crate::{game::GameParametersResource, player::PlayerComponent};
use bevy::{input::gamepad, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_rust_arcade::{ArcadeInput, ArcadeInputEvent};

/// Move player by modifying velocity with input
pub fn player_movement_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    game_parameters: Res<GameParametersResource>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity)>,
) {
    for (player, mut vel) in player_info.iter_mut() {
        // get key presses
        let mut up = keyboard_input.pressed(KeyCode::W);
        let mut down = keyboard_input.pressed(KeyCode::S);
        let mut left = keyboard_input.pressed(KeyCode::A);
        let mut right = keyboard_input.pressed(KeyCode::D);

        for gamepad in gamepads.iter() {
            up |= gamepad_input.pressed(GamepadButton(*gamepad, GamepadButtonType::DPadUp));
            down |= gamepad_input.pressed(GamepadButton(*gamepad, GamepadButtonType::DPadDown));
            left |= gamepad_input.pressed(GamepadButton(*gamepad, GamepadButtonType::DPadLeft));
            right |= gamepad_input.pressed(GamepadButton(*gamepad, GamepadButtonType::DPadRight));
        }

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
