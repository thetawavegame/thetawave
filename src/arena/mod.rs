//! `thetawave` arena module
mod barrier;
mod gate;

pub use self::{
    barrier::{spawn_barriers_system, ArenaBarrierComponent},
    gate::{despawn_gates_system, spawn_despawn_gates_system, MobReachedBottomGateEvent},
};
