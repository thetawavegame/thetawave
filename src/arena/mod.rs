//! `thetawave` arena module

use bevy::prelude::*;
mod barrier;
mod gate;

use crate::{states, GameEnterSet};

pub use self::{barrier::*, gate::*};

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MobReachedBottomGateEvent>();

        app.add_systems(
            (
                spawn_barriers_system.in_set(GameEnterSet::BuildLevel),
                spawn_despawn_gates_system.in_set(GameEnterSet::BuildLevel),
            )
                .in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (despawn_gates_system,)
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );
    }
}
