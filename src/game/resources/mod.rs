//! Resources for managing the game
use serde::Deserialize;

/// Values used globally throughout the game
#[derive(Deserialize)]
pub struct GameParametersResource {
    /// Scale of rapier physics
    pub physics_scale: f32,
    /// Sprite image size multiplier
    pub sprite_scale: f32,
    /// Threshold to set velocity to zero
    pub stop_threshold: f32,
}
