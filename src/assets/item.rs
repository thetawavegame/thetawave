use bevy::prelude::{Handle, Resource, TextureAtlasLayout};
use bevy_asset_loader::prelude::AssetCollection;

use thetawave_interface::spawnable::ItemType;

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(key = "item_placeholder")]
    pub item_placeholder: Handle<TextureAtlasLayout>,
}

impl ItemAssets {
    pub fn get_asset(&self, item_type: &ItemType) -> Handle<TextureAtlasLayout> {
        match item_type {
            ItemType::EnhancedPlating => self.item_placeholder.clone(),
            /*
            ItemType::SteelBarrel => self.item_placeholder.clone(),
            ItemType::PlasmaBlasts => self.item_placeholder.clone(),
            ItemType::HazardousReactor => self.item_placeholder.clone(),
            ItemType::WarpThruster => self.item_placeholder.clone(),
            ItemType::Tentaclover => self.item_placeholder.clone(),
            ItemType::DefenseSatellite => self.item_placeholder.clone(),
            ItemType::DoubleBarrel => self.item_placeholder.clone(),
            ItemType::YithianPlague => self.item_placeholder.clone(),
            ItemType::Spice => self.item_placeholder.clone(),
            ItemType::StructureReinforcement => self.item_placeholder.clone(),
            ItemType::BlasterSizeEnhancer => self.item_placeholder.clone(),
            ItemType::FrequencyAugmentor => self.item_placeholder.clone(),
            ItemType::TractorBeam => self.item_placeholder.clone(),
            ItemType::BlastRepeller => self.item_placeholder.clone(),
            */
        }
    }
}
