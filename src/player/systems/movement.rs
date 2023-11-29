use crate::game::GameParametersResource;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::PlayerComponent;

/// Move player by modifying velocity with input
pub fn player_movement_system(
    game_parameters: Res<GameParametersResource>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity, &ActionState<PlayerAction>)>,
) {
    for (player, mut vel, action_state) in player_info.iter_mut() {
        let up = action_state.pressed(PlayerAction::MoveUp);
        let down = action_state.pressed(PlayerAction::MoveDown);
        let left = action_state.pressed(PlayerAction::MoveLeft);
        let right = action_state.pressed(PlayerAction::MoveRight);

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
