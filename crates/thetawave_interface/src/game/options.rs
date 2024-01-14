use bevy_ecs::system::Resource;
use serde::Deserialize;

pub const DEFAULT_OPTIONS_PROFILE_ID: usize = 1;

/// The 'model' of the Options Sqlite table.
/// Defaults the least graphically intense options.
#[derive(Debug, Default, Clone, Deserialize, Resource)]
pub struct GameOptions {
    pub bloom_enabled: bool,
    pub bloom_intensity: f32,
}
