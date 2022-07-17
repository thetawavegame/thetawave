use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::GameParametersResource,
    player::PlayerComponent,
    spawnable::{spawn_projectile, InitialMotion, ProjectileResource},
};

/// Manages the players firing weapons
pub fn player_fire_weapon_system(
    keyboard_input: Res<Input<MouseButton>>,
    game_parameters: Res<GameParametersResource>,
    mut player_query: Query<(&mut PlayerComponent, &Velocity, &Transform)>,
    time: Res<Time>,
    projectile_resource: Res<ProjectileResource>,
    mut commands: Commands,
) {
    for (mut player_component, rb_vels, transform) in player_query.iter_mut() {
        let left_mouse = keyboard_input.pressed(MouseButton::Left);

        // tick down fire timer
        player_component.fire_timer.tick(time.delta());

        // fire blast if timer finished and input pressed
        if player_component.fire_timer.finished() && left_mouse {
            // position of the spawned blast
            let position = Vec2::new(
                transform.translation.x + player_component.projectile_offset_position.x,
                transform.translation.y + player_component.projectile_offset_position.y,
            );

            let initial_motion = InitialMotion {
                linvel: Some(Vec2::new(
                    (player_component.projectile_velocity.x) + rb_vels.linvel.x,
                    (player_component.projectile_velocity.y) + rb_vels.linvel.y,
                )),
                ..Default::default()
            };

            spawn_projectile(
                &player_component.projectile_type,
                &projectile_resource,
                position,
                player_component.attack_damage,
                player_component.projectile_despawn_time,
                initial_motion,
                &mut commands,
                &game_parameters,
            );

            player_component.fire_timer.reset();
        }
    }
}
