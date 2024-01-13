/// The 'model' of the Options Sqlite table.
#[derive(Debug, Default, Clone)]
pub struct GameOptions {
    pub bloom_enabled: bool,
    pub bloom_intensity: f32,
}
