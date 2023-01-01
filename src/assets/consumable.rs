use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::spawnable::ConsumableType;

#[derive(AssetCollection, Resource)]
pub struct ConsumableAssets {
    #[asset(key = "health_wrench")]
    pub health_wrench: Handle<TextureAtlas>,
    #[asset(key = "defense_wrench")]
    pub defense_wrench: Handle<TextureAtlas>,
    #[asset(key = "money5")]
    pub money5: Handle<TextureAtlas>,
    #[asset(key = "money1")]
    pub money1: Handle<TextureAtlas>,
    #[asset(key = "armor")]
    pub armor: Handle<TextureAtlas>,
}

impl ConsumableAssets {
    pub fn get_asset(&self, consumable_type: &ConsumableType) -> Handle<TextureAtlas> {
        match consumable_type {
            ConsumableType::DefenseWrench => self.defense_wrench.clone(),
            ConsumableType::Money1 => self.money1.clone(),
            ConsumableType::Money5 => self.money5.clone(),
            ConsumableType::HealthWrench => self.health_wrench.clone(),
            ConsumableType::Armor => self.armor.clone(),
        }
    }
}
