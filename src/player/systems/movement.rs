use std::f32::consts::PI;

use bevy::ecs::query::With;
use bevy::ecs::system::{Query, Res};
use bevy::transform::components::Transform;
use bevy_rapier2d::dynamics::Velocity;
use leafwing_input_manager::prelude::ActionState;

use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::{PlayerComponent, PlayerMovementComponent};

use crate::game::GameParametersResource;

/// Move player by modifying velocity with input
pub fn player_movement_system(
    game_parameters: Res<GameParametersResource>,
    mut player_info: Query<(
        &PlayerMovementComponent,
        &mut Velocity,
        &ActionState<PlayerAction>,
    )>,
) {
    for (player_movement, mut vel, action_state) in player_info.iter_mut() {
        let up = action_state.pressed(&PlayerAction::MoveUp);
        let down = action_state.pressed(&PlayerAction::MoveDown);
        let left = action_state.pressed(&PlayerAction::MoveLeft);
        let right = action_state.pressed(&PlayerAction::MoveRight);

        if !player_movement.movement_enabled {
            continue;
        }
        // convert to axis multipliers
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // handle movement in x direction
        if x_axis != 0 {
            // accelerate to the player's maximum speed stat
            vel.linvel.x += player_movement.acceleration.x * (x_axis as f32);
            if vel.linvel.x.abs() > player_movement.speed.x {
                vel.linvel.x = (vel.linvel.x / vel.linvel.x.abs()) * player_movement.speed.x;
            }
        } else if vel.linvel.x.abs() > game_parameters.stop_threshold {
            // decelerate
            vel.linvel.x -= player_movement.deceleration.x * (vel.linvel.x / vel.linvel.x.abs());
        } else {
            vel.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            vel.linvel.y += player_movement.acceleration.y * (y_axis as f32);
            if vel.linvel.y.abs() > player_movement.speed.y {
                vel.linvel.y = (vel.linvel.y / vel.linvel.y.abs()) * player_movement.speed.y;
            }
        } else if vel.linvel.y.abs() > game_parameters.stop_threshold {
            // decelerate
            vel.linvel.y -= player_movement.deceleration.y * (vel.linvel.y / vel.linvel.y.abs());
        } else {
            vel.linvel.y = 0.0;
        }
    }
}

/// Tilt facing direction of player based on its velocity
pub fn player_tilt_system(
    mut player_info: Query<(&Velocity, &mut Transform), With<PlayerComponent>>,
) {
    for (vel, mut player_trans) in player_info.iter_mut() {
        let rotation_amount = -vel.linvel.x.atan2(vel.linvel.y.abs()) / PI;

        player_trans.rotation.z += rotation_amount * 0.05;
        player_trans.rotation.z *= 0.9;
        player_trans.rotation.z = player_trans.rotation.z.clamp(-0.25, 0.25);
        if player_trans.rotation.z.abs() < 0.001 {
            player_trans.rotation.z = 0.0;
        }
    }
}
