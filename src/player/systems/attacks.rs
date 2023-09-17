use std::{collections::HashMap, time::Duration};

use bevy::prelude::*;
use bevy_rapier2d::na::clamp;
use bevy_rapier2d::prelude::*;

use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};

use crate::{
    player::{PlayerComponent, PlayerInput, PlayersResource},
    spawnable::{InitialMotion, SpawnProjectileEvent},
};

/// Increase fire rate of player based on the amount of money collected
// TODO: Remove hardcoded values
pub fn player_scale_fire_rate_system(mut player_query: Query<&mut PlayerComponent>) {
    for mut player in player_query.iter_mut() {
        player.fire_period = 1.0 / (1.5 * ((0.8 * player.money as f32) + 4.0).ln());
    }
}

/// Manages the players firing weapons
#[allow(clippy::too_many_arguments)]
pub fn player_fire_weapon_system(
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut PlayerComponent, &Velocity, &Transform, Entity)>,
    time: Res<Time>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    players_resource: Res<PlayersResource>,
) {
    // get keyboard fire input
    let fire_keyboard_input =
        keyboard_input.pressed(KeyCode::Space) || mouse_input.pressed(MouseButton::Left);

    // get gamepad fire input
    let fire_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::RightTrigger,
                }),
            )
        })
        .collect();

    for (mut player_component, rb_vels, transform, entity) in player_query.iter_mut() {
        // check if player matches input
        let player_input = players_resource.player_inputs[player_component.player_index]
            .clone()
            .unwrap();

        // get fire input for player
        let fire_input = match player_input {
            PlayerInput::Keyboard => fire_keyboard_input,
            PlayerInput::Gamepad(gamepad) => fire_gamepad_inputs[&gamepad],
        };

        // tick fire timer
        player_component.fire_timer.tick(time.delta());

        // fire blast if timer finished and input pressed
        if !player_component.fire_timer.finished() || !fire_input {
            continue;
        }

        let spread_angle = clamp(
            300.0 / player_component.projectile_count as f32,
            0.0,
            300.0,
        );
        let center_projectile_index = (player_component.projectile_count - 1) as f32 / 2.0;

        for i in 0..player_component.projectile_count {
            let center_adjusted_index = i as f32 - center_projectile_index;
            let projectile_transform = Transform {
                translation: Vec3::new(
                    transform.translation.x + player_component.projectile_offset_position.x,
                    transform.translation.y + player_component.projectile_offset_position.y,
                    1.0,
                ),
                ..Default::default()
            };

            // pass player velocity into the spawned blast
            let initial_motion = InitialMotion {
                linvel: Some(Vec2::new(
                    (player_component.projectile_velocity.x) + rb_vels.linvel.x + center_adjusted_index * spread_angle,
                    (player_component.projectile_velocity.y) + rb_vels.linvel.y - f32::abs(center_adjusted_index * spread_angle),
                )),
                ..Default::default()
            };

            // spawn the projectile
            spawn_projectile.send(SpawnProjectileEvent {
                projectile_type: player_component.projectile_type.clone(),
                transform: projectile_transform,
                damage: player_component.attack_damage,
                despawn_time: player_component.projectile_despawn_time,
                initial_motion,
                source: entity,
            });
        }

        // play firing blast sound effect
        sound_effect_event_writer.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::PlayerFireBlast,
        });

        // reset the timer to the player's fire period stat
        let new_period = Duration::from_secs_f32(player_component.fire_period);
        player_component.fire_timer.reset();
        player_component.fire_timer.set_duration(new_period);
    }
}
