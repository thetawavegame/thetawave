use std::collections::HashMap;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, RigidBody, Velocity};

use crate::{
    assets::GameAudioAssets,
    audio,
    player::{PlayerComponent, PlayerInput, PlayersResource},
    spawnable::{InitialMotion, SpawnProjectileEvent},
};

pub fn player_ability_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Velocity,
        &Transform,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    mouse_input: Res<Input<MouseButton>>,
    players_resource: Res<PlayersResource>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
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

    let ability_keyboard_input =
        keyboard_input.pressed(KeyCode::LShift) || mouse_input.pressed(MouseButton::Right);

    let ability_gamepad_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::LeftTrigger,
                }),
            )
        })
        .collect();

    for (mut player_component, mut player_vel, player_trans, mut player_ext_impulse) in
        player_query.iter_mut()
    {
        let player_input = players_resource.player_inputs[player_component.player_index]
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

        let activate_ability_input = match player_input {
            PlayerInput::Keyboard => ability_keyboard_input,
            PlayerInput::Gamepad(gamepad) => ability_gamepad_inputs[&gamepad],
        };

        player_component.ability_cooldown_timer.tick(time.delta());

        // start ability if input pressed and available
        if player_component.ability_cooldown_timer.finished() && activate_ability_input {
            // perform ability
            match player_component.ability_type {
                crate::player::components::AbilityType::Charge(ability_duration) => {
                    info!("CHARGE ABILITY");
                    //if let Some(vec2_normal) = player_vel.linvel.try_normalize() {
                    if let Some(vec2_normal) = Vec2::new(
                        (-(left as i8) + right as i8) as f32,
                        (-(down as i8) + up as i8) as f32,
                    )
                    .try_normalize()
                    {
                        player_ext_impulse.impulse = 12000.0 * vec2_normal;
                    } else {
                        player_ext_impulse.impulse = Vec2::new(0.0, 12000.0);
                    }
                    //player_ext_impulse.impulse = Vec2::new(0.0, 12000.0);
                    player_component.movement_enabled = false;
                    player_component.incoming_damage_multiplier -= 0.5;
                    player_component.ability_action_timer =
                        Some(Timer::from_seconds(ability_duration, TimerMode::Once));
                }
                crate::player::components::AbilityType::MegaBlast(multiplier) => {
                    info!("MEGABLAST ABILITY");
                    audio_channel.play(audio_assets.megablast_ability.clone());
                    let projectile_transform = Transform {
                        translation: Vec3::new(
                            player_trans.translation.x
                                + player_component.projectile_offset_position.x,
                            player_trans.translation.y
                                + player_component.projectile_offset_position.y,
                            1.0,
                        ),
                        scale: Vec3::new(multiplier, multiplier, 1.0),
                        ..Default::default()
                    };

                    let initial_motion = InitialMotion {
                        linvel: Some(Vec2::new(
                            (player_component.projectile_velocity.x) + player_vel.linvel.x,
                            (player_component.projectile_velocity.y) + player_vel.linvel.y,
                        )),
                        ..Default::default()
                    };

                    spawn_projectile.send(SpawnProjectileEvent {
                        projectile_type: player_component.projectile_type.clone(),
                        transform: projectile_transform,
                        damage: player_component.attack_damage * multiplier,
                        health: None,
                        despawn_time: player_component.projectile_despawn_time,
                        initial_motion,
                    });
                }
            }
            // reset timer
            player_component.ability_cooldown_timer.reset();
        }

        if let Some(ability_action_timer) = &mut player_component.ability_action_timer {
            ability_action_timer.tick(time.delta());

            if ability_action_timer.just_finished() {
                match player_component.ability_type {
                    crate::player::components::AbilityType::Charge(_) => {
                        player_vel.linvel = Vec2::new(0.0, 0.0);
                        player_component.movement_enabled = true;
                        player_component.incoming_damage_multiplier += 0.5;
                    }
                    crate::player::components::AbilityType::MegaBlast(_) => {}
                }
            }
        }
    }
}
