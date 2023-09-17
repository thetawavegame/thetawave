use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    options::input::PlayerAction,
    player::{AbilityType, PlayerComponent},
    run::{LevelPhaseType, TutorialLesson},
};

use crate::{
    run::CurrentRunProgressResource,
    spawnable::{InitialMotion, SpawnProjectileEvent},
};

#[allow(clippy::too_many_arguments)]
pub fn player_ability_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &mut Velocity,
        &Transform,
        &mut ExternalImpulse,
        &ActionState<PlayerAction>,
        Entity,
    )>,
    time: Res<Time>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    run_resource: Res<CurrentRunProgressResource>,
) {
    // first make sure that the players can shoot based on the level
    let can_use_ability = run_resource.current_level.clone().is_some_and(|level| {
        level.current_phase.is_some_and(|phase| {
            if let LevelPhaseType::Tutorial {
                tutorial_lesson, ..
            } = phase.phase_type
            {
                matches!(tutorial_lesson, TutorialLesson::SpecialAbility)
            } else {
                true
            }
        })
    });

    if can_use_ability {
        for (
            mut player_component,
            mut player_vel,
            player_trans,
            mut player_ext_impulse,
            action_state,
            entity,
        ) in player_query.iter_mut()
        {
            let activate_ability_input = action_state.pressed(PlayerAction::SpecialAttack);
            let up = action_state.pressed(PlayerAction::MoveUp);
            let down = action_state.pressed(PlayerAction::MoveDown);
            let left = action_state.pressed(PlayerAction::MoveLeft);
            let right = action_state.pressed(PlayerAction::MoveRight);

            // update ability cooldown timer
            player_component.ability_cooldown_timer.tick(time.delta());

            // start ability if input pressed and available
            if player_component.ability_cooldown_timer.finished() && activate_ability_input {
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
                        player_component.movement_enabled = false;
                        player_component.incoming_damage_multiplier -= 0.5;
                        player_component.ability_action_timer =
                            Some(Timer::from_seconds(ability_duration, TimerMode::Once));
                    }
                    // shoot a giant projectile
                    AbilityType::MegaBlast(multiplier) => {
                        sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::MegablastAbility,
                        });
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
                            damage: (player_component.attack_damage as f32 * multiplier).round()
                                as usize,
                            despawn_time: player_component.projectile_despawn_time,
                            initial_motion,
                            source: entity,
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
                            player_component.movement_enabled = true;
                            player_component.incoming_damage_multiplier += 0.5;
                        }
                        AbilityType::MegaBlast(_) => {}
                    }
                }
            }
        }
    }
}
