use bevy::{
    prelude::{Handle, Resource, TextureAtlasLayout},
    render::texture::Image,
    text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::abilities::{SlotOneAbilityType, SlotTwoAbilityType};

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(key = "font.lunchds")]
    pub lunchds_font: Handle<Font>,
    #[asset(key = "thetawave_logo.layout")]
    pub thetawave_logo_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "thetawave_logo.image")]
    pub thetawave_logo_image: Handle<Image>,
    #[asset(key = "thetawave_menu_button.layout")]
    pub thetawave_menu_button_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "thetawave_menu_button.image")]
    pub thetawave_menu_button_image: Handle<Image>,
    #[asset(key = "ability_icon.mega_blast")]
    pub mega_blast_ability: Handle<Image>,
    #[asset(key = "ability_icon.charge")]
    pub charge_ability: Handle<Image>,
    #[asset(key = "ability_icon.standard_blast")]
    pub standard_blast_ability: Handle<Image>,
    #[asset(key = "ability_icon.standard_bullet")]
    pub standard_bullet_ability: Handle<Image>,
    #[asset(key = "ability_slot.left")]
    pub left_ability_slot: Handle<Image>,
    #[asset(key = "ability_slot.right")]
    pub right_ability_slot: Handle<Image>,
    #[asset(key = "warning_gradient")]
    pub warning_gradient: Handle<Image>,
    #[asset(key = "defense_gradient")]
    pub defense_gradient: Handle<Image>,
}

impl UiAssets {
    pub fn get_slot_1_ability_image(&self, ability_type: &SlotOneAbilityType) -> Handle<Image> {
        match ability_type {
            SlotOneAbilityType::StandardBlast => self.standard_blast_ability.clone(),
            SlotOneAbilityType::StandardBullet => self.standard_bullet_ability.clone(),
        }
    }

    pub fn get_slot_2_ability_image(&self, ability_type: &SlotTwoAbilityType) -> Handle<Image> {
        match ability_type {
            SlotTwoAbilityType::MegaBlast => self.mega_blast_ability.clone(),
            SlotTwoAbilityType::Charge => self.charge_ability.clone(),
        }
    }

    pub fn get_ability_slot_image(&self, is_flipped: bool) -> Handle<Image> {
        if is_flipped {
            self.right_ability_slot.clone()
        } else {
            self.left_ability_slot.clone()
        }
    }
}
