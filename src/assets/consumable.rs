use bevy::prelude::{Color, Handle, Resource, TextureAtlas};
use bevy_asset_loader::prelude::AssetCollection;

use thetawave_interface::spawnable::ConsumableType;

#[derive(AssetCollection, Resource)]
pub struct ConsumableAssets {
    #[asset(key = "health_wrench")]
    pub health_wrench: Handle<TextureAtlas>,
    #[asset(key = "money3")]
    pub money3: Handle<TextureAtlas>,
    #[asset(key = "money1")]
    pub money1: Handle<TextureAtlas>,
    #[asset(key = "armor")]
    pub armor: Handle<TextureAtlas>,
    #[asset(key = "gain_projectiles")]
    pub gain_projectiles: Handle<TextureAtlas>,
}

impl ConsumableAssets {
    pub fn get_asset(&self, consumable_type: &ConsumableType) -> Handle<TextureAtlas> {
        match consumable_type {
            ConsumableType::Money1 => self.money1.clone(),
            ConsumableType::Money3 => self.money3.clone(),
            ConsumableType::HealthWrench => self.health_wrench.clone(),
            ConsumableType::Armor => self.armor.clone(),
            ConsumableType::GainProjectiles => self.gain_projectiles.clone(),
        }
    }

    #[allow(unused)] // Placeholder for if we put this in the item config files
    pub fn get_color(&self, consumable_type: &ConsumableType, bloom_intensity: f32) -> Color {
        Color::rgb(
            1.0 + 0.6 * bloom_intensity,
            1.0 + 0.6 * bloom_intensity,
            1.0 + 0.6 * bloom_intensity,
        )
    }
}
