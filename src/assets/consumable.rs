use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
}

impl ConsumableAssets {
    pub fn get_asset(&self, consumable_type: &ConsumableType) -> Handle<TextureAtlas> {
        match consumable_type {
            ConsumableType::Money1 => self.money1.clone(),
            ConsumableType::Money3 => self.money3.clone(),
            ConsumableType::HealthWrench => self.health_wrench.clone(),
            ConsumableType::Armor => self.armor.clone(),
        }
    }

    pub fn get_color(&self, consumable_type: &ConsumableType) -> Color {
        match consumable_type {
            ConsumableType::Money1 => Color::rgb(1.6, 1.6, 1.6),
            ConsumableType::Money3 => Color::rgb(1.6, 1.6, 1.6),
            ConsumableType::HealthWrench => Color::rgb(1.6, 1.6, 1.6),
            ConsumableType::Armor => Color::rgb(1.6, 1.6, 1.6),
        }
    }
}
