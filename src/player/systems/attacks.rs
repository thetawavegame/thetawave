use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::PlayerAction;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    player::PlayerComponent,
};

use crate::spawnable::{InitialMotion, SpawnProjectileEvent};

/// Increase fire rate of player based on the amount of money collected
// TODO: Remove hardcoded values
pub fn player_scale_fire_rate_system(mut player_query: Query<&mut PlayerComponent>) {
    for mut player in player_query.iter_mut() {
        player.fire_period = 1.0 / (1.5 * ((0.8 * player.money as f32) + 4.0).ln());
    }
}

/// Manages the players firing weapons
pub fn player_fire_weapon_system(
    mut player_query: Query<(
        &mut PlayerComponent,
        &Velocity,
        &Transform,
        &ActionState<PlayerAction>,
        Entity,
    )>,
    time: Res<Time>,
    mut spawn_projectile: EventWriter<SpawnProjectileEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
) {
    for (mut player_component, rb_vels, transform, action_state, entity) in player_query.iter_mut()
    {
        let fire_input = action_state.pressed(PlayerAction::BasicAttack);

        // tick fire timer
        player_component.fire_timer.tick(time.delta());

        // fire blast if timer finished and input pressed
        if player_component.fire_timer.finished()
            && fire_input
            && player_component.main_attack_is_enabled()
        {
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
                    (player_component.projectile_velocity.x) + rb_vels.linvel.x,
                    (player_component.projectile_velocity.y) + rb_vels.linvel.y,
                )),
                ..Default::default()
            };

            // spawn the projectile
            spawn_projectile.send(SpawnProjectileEvent {
                projectile_type: player_component.projectile_type.clone(),
                transform: projectile_transform,
                range: player_component.projectile_range,
                damage: player_component.attack_damage,
                despawn_time: player_component.projectile_despawn_time,
                initial_motion,
                source: entity,
            });

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
}
