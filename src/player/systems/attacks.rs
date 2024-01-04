use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use thetawave_interface::{input::PlayerAction, player::PlayerComponent, weapon::WeaponComponent};

use crate::spawnable::{FireWeaponEvent, InitialMotion};

const MIN_RELOAD_TIME: f32 = 0.1;

pub fn scale_fire_rate_system(mut player_query: Query<(&PlayerComponent, &mut WeaponComponent)>) {
    for (player, mut weapon) in player_query.iter_mut() {
        let diminishing_factor = 0.08;
        let adjusted_money = (1.0 + player.money as f32).ln();
        let new_reload_time =
            weapon.base_reload_time - (adjusted_money * diminishing_factor).max(MIN_RELOAD_TIME);

        weapon.set_reload_time(new_reload_time);
    }
}

/// Manages the players firing weapons
pub fn fire_weapon_system(
    mut player_query: Query<
        (
            &mut WeaponComponent,
            &Velocity,
            &Transform,
            &ActionState<PlayerAction>,
            Entity,
        ),
        With<PlayerComponent>,
    >,
    mut fire_weapon: EventWriter<FireWeaponEvent>,
) {
    for (mut weapon, rb_vels, transform, action_state, entity) in player_query.iter_mut() {
        let fire_input = action_state.pressed(PlayerAction::BasicAttack);

        // fire blast if timer finished and input pressed
        if !weapon.can_fire() || !fire_input {
            continue;
        }

        if let Some(weapon_projectile_data) = weapon.fire_weapon() {
            // pass player velocity into the spawned blast
            let initial_motion = InitialMotion {
                linvel: Some(rb_vels.linvel),
                ..Default::default()
            };

            // send the event to fire the weapon
            fire_weapon.send(FireWeaponEvent {
                weapon_projectile_data,
                source_transform: *transform,
                source_entity: entity,
                initial_motion,
            });
        }
    }
}
