use bevy::prelude::{Handle, Image, Res, Resource};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::character::CharacterType;

/// Collection of images for player characters
#[derive(AssetCollection, Resource)]
pub(crate) struct PlayerAssets {
    #[asset(key = "captain")]
    pub captain: Handle<Image>,
    #[asset(key = "juggernaut")]
    pub juggernaut: Handle<Image>,
    #[asset(key = "captain_outline")]
    pub captain_outline: Handle<Image>,
    #[asset(key = "juggernaut_outline")]
    pub juggernaut_outline: Handle<Image>,
}

impl PlayerAssets {
    /// Use a CharacterType enum to access an image handle
    pub(crate) fn get_asset(&self, character_type: &CharacterType) -> Handle<Image> {
        match character_type {
            CharacterType::Captain => self.captain.clone(),
            CharacterType::Juggernaut => self.juggernaut.clone(),
        }
    }

    /// Use a CharacterType enum to access a character's associated outline image handle
    pub(crate) fn get_outline_asset(&self, character_type: &CharacterType) -> Handle<Image> {
        match character_type {
            CharacterType::Captain => self.captain_outline.clone(),
            CharacterType::Juggernaut => self.juggernaut_outline.clone(),
        }
    }
}
