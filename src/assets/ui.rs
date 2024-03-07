use bevy::{
    prelude::{Handle, Resource, TextureAtlasLayout},
    render::texture::Image,
};
use bevy_asset_loader::prelude::AssetCollection;

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
}
