//! `thetawave` arena module
mod barrier;
mod gate;

pub use self::{
    barrier::spawn_barriers_system,
    gate::{despawn_gates_system, spawn_despawn_gates_system},
};
