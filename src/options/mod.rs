//! `thetawave` player module
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use thetawave_interface::options::input::{get_input_bindings, InputsResource, MenuAction};

mod display;
mod input;

use crate::states;
use std::default::Default;
use std::path::PathBuf;

pub use self::display::{
    set_window_icon, toggle_fullscreen_system, toggle_zoom_system, DisplayConfig,
};
use self::input::spawn_menu_explorer_system;

#[cfg_attr(
    all(not(target_arch = "wasm32"), feature = "cli"),
    derive(argh::FromArgs)
)]
#[derive(Default, Debug, PartialEq, Eq)]
/// Options used to start Thetawave. As many of these as possible are inferred/have "sensible"
/// defaults.
pub struct GameInitCLIOptions {
    #[cfg_attr(all(not(target_arch = "wasm32"), feature = "cli"), argh(option))]
    /// the directory that is used for `bevy::asset::AssetPlugin`. This is generally
    /// 'EXECUTABLE_DIR/assets/' or 'CARGO_MANIFEST_DIR/assets'.
    pub assets_dir: Option<PathBuf>,
}
impl GameInitCLIOptions {
    pub fn from_environ_on_supported_platforms_with_default_fallback() -> Self {
        #[cfg(all(not(target_arch = "wasm32"), feature = "cli"))]
        {
            return argh::from_env();
        }
        #[allow(unreachable_code)] // The CLI provides the default.
        Default::default()
    }
}
pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());

        app.insert_resource(InputsResource::new(get_input_bindings()));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, (set_window_icon, spawn_menu_explorer_system));

        app.add_systems(Update, toggle_fullscreen_system);

        app.add_systems(
            Update,
            toggle_zoom_system.run_if(in_state(states::AppStates::Game)),
        );
    }
}
