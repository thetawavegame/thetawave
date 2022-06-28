use crate::{game::GameParametersResource, player::PlayerComponent};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::RigidBodyVelocity};

/// Move player by modifying velocity with input
pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut player_info: Query<(&PlayerComponent, &mut Velocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        // get key presses
        let up = keyboard_input.pressed(KeyCode::W);
        let down = keyboard_input.pressed(KeyCode::S);
        let left = keyboard_input.pressed(KeyCode::A);
        let right = keyboard_input.pressed(KeyCode::D);

        // convert to axis multipliers
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        // handle movement in x direction
        if x_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.x += player.acceleration.x * (x_axis as f32);
            if rb_vels.linvel.x.abs() > player.speed.x {
                rb_vels.linvel.x = (rb_vels.linvel.x / rb_vels.linvel.x.abs()) * player.speed.x;
            }
        } else if rb_vels.linvel.x.abs() > game_parameters.stop_threshold {
            // decelerate
            rb_vels.linvel.x -= player.deceleration.x * (rb_vels.linvel.x / rb_vels.linvel.x.abs());
        } else {
            rb_vels.linvel.x = 0.0;
        }

        // handle movement in y direction
        if y_axis != 0 {
            // accelerate to the player's maximum speed stat
            rb_vels.linvel.y += player.acceleration.y * (y_axis as f32);
            if rb_vels.linvel.y.abs() > player.speed.y {
                rb_vels.linvel.y = (rb_vels.linvel.y / rb_vels.linvel.y.abs()) * player.speed.y;
            }
        } else if rb_vels.linvel.y.abs() > game_parameters.stop_threshold {
            // decelerate
            rb_vels.linvel.y -= player.deceleration.y * (rb_vels.linvel.y / rb_vels.linvel.y.abs());
        } else {
            rb_vels.linvel.y = 0.0;
        }
    }
}
