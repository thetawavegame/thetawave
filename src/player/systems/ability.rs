use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, RigidBody, Velocity};

use crate::player::PlayerComponent;

pub fn player_ability_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Velocity,
        &Transform,
        &mut ExternalImpulse,
    )>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepads: Res<Gamepads>,
    gamepad_input: Res<Input<GamepadButton>>,
    mouse_input: Res<Input<MouseButton>>,
) {
    for (mut player_component, mut player_vel, player_trans, mut player_ext_impulse) in
        player_query.iter_mut()
    {
        let mut up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let mut down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let mut left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let mut right =
            keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        for gamepad in gamepads.iter() {
            up |= gamepad_input.pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::DPadUp,
            });
            down |= gamepad_input.pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::DPadDown,
            });
            left |= gamepad_input.pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::DPadLeft,
            });
            right |= gamepad_input.pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::DPadRight,
            });
        }

        player_component.ability_cooldown_timer.tick(time.delta());

        // start ability if input pressed and available
        if player_component.ability_cooldown_timer.finished()
            && (keyboard_input.pressed(KeyCode::LShift) || mouse_input.pressed(MouseButton::Right))
        {
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
                crate::player::components::AbilityType::MegaBlast => {
                    info!("MEGABLAST ABILITY");
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
                    crate::player::components::AbilityType::MegaBlast => {}
                }
            }
        }
    }
}
