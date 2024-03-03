use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::system::{Query, Res};
use bevy::math::Vec2;
use bevy::time::{Time, Timer, TimerMode};
use bevy::transform::components::Transform;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::audio::SoundEffectType;
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::{AbilityType, PlayerComponent, PlayerMovementComponent};
use thetawave_interface::weapon::{WeaponComponent, WeaponProjectileData};

use crate::spawnable::{FireWeaponEvent, InitialMotion};

#[allow(clippy::too_many_arguments)]
pub fn player_ability_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut PlayerMovementComponent,
        &mut Velocity,
        &Transform,
        &mut ExternalImpulse,
        &ActionState<PlayerAction>,
        Entity,
        &WeaponComponent,
    )>,
    time: Res<Time>,
    mut fire_weapon: EventWriter<FireWeaponEvent>,
) {
    for (
        mut player_component,
        mut player_movement,
        mut player_vel,
        player_trans,
        mut player_ext_impulse,
        action_state,
        entity,
        weapon_component,
    ) in player_query.iter_mut()
    // No-op for players whose special attack is disabled
    {
        let activate_ability_input = action_state.pressed(&PlayerAction::SpecialAttack);
        let up = action_state.pressed(&PlayerAction::MoveUp);
        let down = action_state.pressed(&PlayerAction::MoveDown);
        let left = action_state.pressed(&PlayerAction::MoveLeft);
        let right = action_state.pressed(&PlayerAction::MoveRight);

        // update ability cooldown timer
        player_component.ability_cooldown_timer.tick(time.delta());

        // start ability if input pressed and available
        if player_component.ability_cooldown_timer.finished()
            && activate_ability_input
            && player_component.ability_is_enabled()
        {
            // perform ability
            match player_component.ability_type {
                // TODO: move hardcoded values to player componnet
                // charge player in direction
                AbilityType::Charge(ability_duration) => {
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
                    player_movement.movement_enabled = false;
                    player_component.incoming_damage_multiplier -= 0.5;
                    player_component.ability_action_timer =
                        Some(Timer::from_seconds(ability_duration, TimerMode::Once));
                }
                // shoot a giant projectile
                AbilityType::MegaBlast(multiplier) => {
                    let initial_motion = InitialMotion {
                        linvel: Some(player_vel.linvel),
                        ..Default::default()
                    };

                    fire_weapon.send(FireWeaponEvent {
                        weapon_projectile_data: WeaponProjectileData {
                            damage: weapon_component.projectile_data.damage * multiplier as usize,
                            speed: weapon_component.projectile_data.speed + (multiplier * 50.0),
                            count: (weapon_component.projectile_data.count / 2).max(1),
                            size: multiplier,
                            sound: SoundEffectType::MegablastAbility,
                            ..weapon_component.projectile_data.clone()
                        },
                        source_transform: *player_trans,
                        source_entity: entity,
                        initial_motion,
                    });
                }
            }
            // reset ability timer
            player_component.ability_cooldown_timer.reset();
        }

        // handle ability action timer
        if let Some(ability_action_timer) = &mut player_component.ability_action_timer {
            // tick timer
            ability_action_timer.tick(time.delta());

            // change values when timer finished
            if ability_action_timer.just_finished() {
                match player_component.ability_type {
                    AbilityType::Charge(_) => {
                        player_vel.linvel = Vec2::new(0.0, 0.0);
                        player_movement.movement_enabled = true;
                        player_component.incoming_damage_multiplier += 0.5;
                    }
                    AbilityType::MegaBlast(_) => {}
                }
            }
        }
    }
}
