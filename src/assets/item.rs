use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use thetawave_interface::spawnable::ItemType;

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(key = "item_placeholder")]
    pub item_placeholder: Handle<TextureAtlas>,
}

impl ItemAssets {
    pub fn get_asset(&self, item_type: &ItemType) -> Handle<TextureAtlas> {
        match item_type {
            ItemType::SteelBarrel => todo!(),
            ItemType::PlasmaBlasts => todo!(),
            ItemType::HazardousReactor => todo!(),
            ItemType::WarpThruster => todo!(),
            ItemType::Tentaclover => todo!(),
            ItemType::DefenseSatellite => todo!(),
            ItemType::DoubleBarrel => todo!(),
            ItemType::YithianPlague => todo!(),
            ItemType::Spice => todo!(),
            ItemType::EnhancedPlating => todo!(),
            ItemType::StructureReinforcement => todo!(),
            ItemType::BlasterSizeEnhancer => todo!(),
            ItemType::FrequencyAugmentor => todo!(),
            ItemType::TractorBeam => todo!(),
            ItemType::BlastRepeller => todo!(),
        }
    }
}
