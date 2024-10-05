use bevy::{
    asset::Handle,
    prelude::{Image, Res, Resource},
    sprite::TextureAtlasLayout,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use thetawave_interface::spawnable::ConsumableType;

/// Collection of texture atlases and images for consumable sprites
#[derive(AssetCollection, Resource)]
pub struct ConsumableAssets {
    #[asset(key = "health_wrench.layout")]
    pub health_wrench_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "health_wrench.image")]
    pub health_wrench_image: Handle<Image>,
    #[asset(key = "money3.layout")]
    pub money3_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "money3.image")]
    pub money3_image: Handle<Image>,
    #[asset(key = "money1.layout")]
    pub money1_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "money1.image")]
    pub money1_image: Handle<Image>,
    #[asset(key = "armor.layout")]
    pub armor_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "armor.image")]
    pub armor_image: Handle<Image>,
    #[asset(key = "gain_projectiles.layout")]
    pub gain_projectiles_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "gain_projectiles.image")]
    pub gain_projectiles_image: Handle<Image>,
}

impl ConsumableAssets {
    /// Use a ConsumableType enum to access a texture atlas layout
    pub fn get_texture_atlas_layout(
        &self,
        consumable_type: &ConsumableType,
    ) -> Handle<TextureAtlasLayout> {
        match consumable_type {
            ConsumableType::Money1 => self.money1_layout.clone(),
            ConsumableType::Money3 => self.money3_layout.clone(),
            ConsumableType::HealthWrench => self.health_wrench_layout.clone(),
            ConsumableType::Armor => self.armor_layout.clone(),
            ConsumableType::GainProjectiles => self.gain_projectiles_layout.clone(),
        }
    }

    /// Use a ConsumableType enum to access an image handle
    pub fn get_image(&self, consumable_type: &ConsumableType) -> Handle<Image> {
        match consumable_type {
            ConsumableType::Money1 => self.money1_image.clone(),
            ConsumableType::Money3 => self.money3_image.clone(),
            ConsumableType::HealthWrench => self.health_wrench_image.clone(),
            ConsumableType::Armor => self.armor_image.clone(),
            ConsumableType::GainProjectiles => self.gain_projectiles_image.clone(),
        }
    }
}
