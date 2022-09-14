use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::GameParametersResource,
    player::PlayerComponent,
    spawnable::{spawn_projectile, InitialMotion, ProjectileResource},
    SoundEffectsAudioChannel,
};

/// Manages the players firing weapons
pub fn player_fire_weapon_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    game_parameters: Res<GameParametersResource>,
    mut player_query: Query<(&mut PlayerComponent, &Velocity, &Transform)>,
    time: Res<Time>,
    projectile_resource: Res<ProjectileResource>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    //let gamepad = gamepads.iter().next().clone();
    for (mut player_component, rb_vels, transform) in player_query.iter_mut() {
        let mut left_mouse =
            mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space);

        for gamepad in gamepads.iter() {
            left_mouse |= gamepad_input.pressed(GamepadButton {
                gamepad: *gamepad,
                button_type: GamepadButtonType::East,
            });
        }

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

            audio_channel.play(asset_server.load("sounds/player_fire_blast.wav"));

            let new_period = Duration::from_secs_f32(player_component.fire_period);
            player_component.fire_timer.reset();
            player_component.fire_timer.set_duration(new_period);
        }
    }
}
