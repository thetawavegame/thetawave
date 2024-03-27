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
    /// Maximum possible projectiles for 1 of the player/mobs shots. Mainly kept low for perf and as
    /// a hard cap (along with fire rate) on how much of a "bullet hell" each mob/player creates.
    pub max_player_projectiles: u16,
    /// Maximum possible speed of an entity
    pub max_speed: f32,
    /// Distance between the center of the screen and the player spawn point
    pub player_spawn_distance: f32,
    /// Sprite image size multiplier
    pub sprite_scale: f32,
    /// Threshold to set velocity to zero
    pub stop_threshold: f32,
    /// Range of mouse scanning
    pub scan_range: f32,
}
