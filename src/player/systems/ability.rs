use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::player::PlayerComponent;

pub fn player_ability_system(
    mut player_query: Query<(&mut PlayerComponent, &Velocity, &Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut player_component, player_vel, player_trans) in player_query.iter_mut() {
        player_component.ability_timer.tick(time.delta());

        if player_component.ability_timer.finished() && keyboard_input.pressed(KeyCode::LShift) {
            // perform ability
            match player_component.ability_type {
                crate::player::components::AbilityType::Charge => {
                    info!("CHARGE ABILITY");
                }
                crate::player::components::AbilityType::MegaBlast => {
                    info!("MEGABLAST ABILITY");
                }
            }
            // reset timer
            player_component.ability_timer.reset();
        }
    }
}
