use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use thetawave_interface::{input::PlayerAction, player::PlayerComponent, weapon::WeaponComponent};

use crate::spawnable::{FireWeaponEvent, InitialMotion};

pub fn scale_fire_rate_system(mut player_query: Query<(&PlayerComponent, &mut WeaponComponent)>) {
    for (player, mut weapon) in player_query.iter_mut() {
        weapon.set_reload_time_from_money(player.money);
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
