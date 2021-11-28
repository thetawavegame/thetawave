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
    /// Maximum possible speed of an entity
    pub max_speed: f32,
    /// Standard z coordinate of camera
    pub camera_z: f32,
    /// Z coordinate of camera in zoomed out mode
    pub camera_zoom_out_z: f32,
}
