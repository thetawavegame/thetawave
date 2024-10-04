//! Exposes a plugin that renders a rectangular boundary that the player cannot cross, but mobs
//! can. Also handles sending events when mobs reach the botton of the screen.

use barrier::spawn_barriers_system;
use bevy::{
    app::{App, Plugin, Update},
    prelude::{in_state, IntoSystemConfigs, OnEnter},
};
use thetawave_interface::{objective::MobReachedBottomGateEvent, states};
mod barrier;
mod gate;

use crate::GameEnterSet;

use self::gate::{despawn_gates_system, spawn_despawn_gates_system};

pub(crate) use self::barrier::ArenaBarrierComponent;

/// Plugin that spawns a rectangular boundary for the main game play area and fires off
/// `MobReachedBottomGateEvent` at the right times
pub(super) struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MobReachedBottomGateEvent>();

        app.add_systems(
            OnEnter(states::AppStates::Game),
            (spawn_barriers_system, spawn_despawn_gates_system).in_set(GameEnterSet::BuildLevel),
        );

        app.add_systems(
            Update,
            despawn_gates_system
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}
