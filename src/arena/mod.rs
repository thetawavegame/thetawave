//! `thetawave` arena module

use bevy::prelude::*;
use thetawave_interface::objective::MobReachedBottomGateEvent;
mod barrier;
mod gate;

use crate::{states, GameEnterSet};

pub use self::{barrier::*, gate::*};

pub struct ArenaPlugin;

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
