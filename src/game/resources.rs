//! Resources for managing the game

use bevy::prelude::*;
use serde::Deserialize;

/// Values used globally throughout the game
#[derive(Resource, Deserialize)]
pub struct GameParametersResource {
    /// Standard z coordinate of camera
    pub camera_z: f32,
    /// Z coordinate of camera in zoomed out mode
    pub camera_zoom_out_scale: f32,
    /// Maximum possible projectiles of the player
    pub max_player_projectiles: f32,
    /// Maximum possible speed of an entity
    pub max_speed: f32,
    pub player_spawn_distance: f32,
    /// Sprite image size multiplier
    pub sprite_scale: f32,
    /// Threshold to set velocity to zero
    pub stop_threshold: f32,
    /// Range of mouse scanning
    pub scan_range: f32,
}
