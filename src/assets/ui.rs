use bevy::{
    prelude::{Handle, Resource, TextureAtlasLayout},
    render::texture::Image,
    text::Font,
};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::{
    abilities::{SlotOneAbilityType, SlotTwoAbilityType},
    character::CharacterStatType,
};

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(key = "thetawave_logo.layout")]
    pub thetawave_logo_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "thetawave_logo.image")]
    pub thetawave_logo_image: Handle<Image>,
    #[asset(key = "thetawave_menu_button.layout")]
    pub thetawave_menu_button_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "thetawave_menu_button.image")]
    pub thetawave_menu_button_image: Handle<Image>,
    #[asset(key = "large_menu_button.layout")]
    pub large_menu_button_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "large_menu_button.image")]
    pub large_menu_button_image: Handle<Image>,
    #[asset(key = "gamepad_button_a.layout")]
    pub gamepad_button_a_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "gamepad_button_a.image")]
    pub gamepad_button_a_image: Handle<Image>,
    #[asset(key = "keyboard_key_return.layout")]
    pub keyboard_key_return_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "keyboard_key_return.image")]
    pub keyboard_key_return_image: Handle<Image>,
    #[asset(key = "arrow_right.layout")]
    pub arrow_right_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "arrow_right.image")]
    pub arrow_right_image: Handle<Image>,
    #[asset(key = "arrow_left.layout")]
    pub arrow_left_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "arrow_left.image")]
    pub arrow_left_image: Handle<Image>,
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
    #[asset(key = "font.wibletown")]
    pub wibletown_font: Handle<Font>,
    #[asset(key = "warning_gradient")]
    pub warning_gradient: Handle<Image>,
    #[asset(key = "defense_gradient")]
    pub defense_gradient: Handle<Image>,
    #[asset(key = "stat_icon.damage")]
    pub damage_icon: Handle<Image>,
    #[asset(key = "stat_icon.speed")]
    pub speed_icon: Handle<Image>,
    #[asset(key = "stat_icon.range")]
    pub range_icon: Handle<Image>,
    #[asset(key = "stat_icon.size")]
    pub size_icon: Handle<Image>,
    #[asset(key = "stat_icon.fire_rate")]
    pub fire_rate_icon: Handle<Image>,
    #[asset(key = "stat_icon.health")]
    pub health_icon: Handle<Image>,
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

    pub fn get_stat_icon(&self, stat: &CharacterStatType) -> Handle<Image> {
        match stat {
            CharacterStatType::Damage => self.damage_icon.clone(),
            CharacterStatType::Health => self.health_icon.clone(),
            CharacterStatType::Range => self.range_icon.clone(),
            CharacterStatType::FireRate => self.fire_rate_icon.clone(),
            CharacterStatType::Size => self.size_icon.clone(),
            CharacterStatType::Speed => self.speed_icon.clone(),
        }
    }
}
