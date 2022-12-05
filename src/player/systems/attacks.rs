use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::PlayerComponent,
    spawnable::{InitialMotion, SpawnProjectileEvent},
    SoundEffectsAudioChannel,
};

// TODO: remove from game
/// Increase fire rate of player based on the amount of money collected
pub fn player_scale_fire_rate_system(mut player_query: Query<&mut PlayerComponent>) {
    for mut player in player_query.iter_mut() {
        player.fire_period = 1.0 / (2.0 * ((player.money as f32) + 4.0).ln());
    }
}

/// Manages the players firing weapons
#[allow(clippy::too_many_arguments)]
pub fn player_fire_weapon_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerComponent, &Velocity, &Transform)>,
    time: Res<Time>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    for (mut player_component, rb_vels, transform) in player_query.iter_mut() {
        // get input for firing weapons
        let mut left_mouse =
            mouse_input.pressed(MouseButton::Left) || keyboard_input.pressed(KeyCode::Space);

        for gamepad in gamepads.iter() {
            left_mouse |= gamepad_input.pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::East,
            });
        }

        // tick fire timer
        player_component.fire_timer.tick(time.delta());

        // fire blast if timer finished and input pressed
        if player_component.fire_timer.finished() && left_mouse {
            // position of the spawned blast
            let position = Vec2::new(
                transform.translation.x + player_component.projectile_offset_position.x,
                transform.translation.y + player_component.projectile_offset_position.y,
            );

            // pass player velocity into the spawned blast
            let initial_motion = InitialMotion {
                linvel: Some(Vec2::new(
                    (player_component.projectile_velocity.x) + rb_vels.linvel.x,
                    (player_component.projectile_velocity.y) + rb_vels.linvel.y,
                )),
                ..Default::default()
            };

            // spawn the projectile
            spawn_projectile.send(SpawnProjectileEvent {
                projectile_type: player_component.projectile_type.clone(),
                position,
                damage: player_component.attack_damage,
                despawn_time: player_component.projectile_despawn_time,
                initial_motion,
            });

            // play firing blast sound effect
            audio_channel.play(asset_server.load("sounds/player_fire_blast.wav"));

            // reset the timer to the player's fire period stat
            let new_period = Duration::from_secs_f32(player_component.fire_period);
            player_component.fire_timer.reset();
            player_component.fire_timer.set_duration(new_period);
        }
    }
}
