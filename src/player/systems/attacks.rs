use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::GameParametersResource,
    player::PlayerComponent,
    spawnable::{spawn_projectile, InitialMotion, ProjectileResource},
};

pub fn player_fire_weapon_system(
    keyboard_input: Res<Input<MouseButton>>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut player_query: Query<(&mut PlayerComponent, &RigidBodyVelocity, &RigidBodyPosition)>,
    time: Res<Time>,
    projectile_resource: Res<ProjectileResource>,
    mut commands: Commands,
) {
    for (mut player, rb_vels, rb_pos) in player_query.iter_mut() {
        let left_mouse = keyboard_input.pressed(MouseButton::Left);

        // tick down fire timer
        player.fire_timer.tick(time.delta());

        // fire blast if timer finished and input pressed
        if player.fire_timer.finished() && left_mouse {
            // position of the spawned blast
            let position = Vec2::new(
                rb_pos.position.translation.x + player.projectile_offset_position.x,
                rb_pos.position.translation.y + player.projectile_offset_position.y,
            );

            let initial_motion = InitialMotion {
                random_angvel: None,
                linvel: Some(Vec2::new(
                    (player.projectile_velocity.x * rapier_config.scale) + rb_vels.linvel.x,
                    (player.projectile_velocity.y * rapier_config.scale) + rb_vels.linvel.y,
                )),
            };

            spawn_projectile(
                &player.projectile_type,
                &projectile_resource,
                position,
                10.0, // TODO: pass from player component
                player.projectile_despawn_time,
                initial_motion,
                &mut commands,
                &rapier_config,
                &game_parameters,
            );

            player.fire_timer.reset();
        }
    }
}
