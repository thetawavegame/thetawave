use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::player::CharacterType;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
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
    pub fn get_asset(&self, character_type: &CharacterType) -> Handle<Image> {
        match character_type {
            CharacterType::Captain => self.captain.clone(),
            CharacterType::Juggernaut => self.juggernaut.clone(),
        }
    }

    pub fn get_outline_asset(&self, character_type: &CharacterType) -> Handle<Image> {
        match character_type {
            CharacterType::Captain => self.captain_outline.clone(),
            CharacterType::Juggernaut => self.juggernaut_outline.clone(),
        }
    }
}
