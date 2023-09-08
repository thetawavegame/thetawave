//! `thetawave` player module
use bevy::prelude::*;
use std::{
    env::current_dir,
    fs::{DirBuilder, File},
    io::prelude::*,
};

mod display;

use crate::states;
use std::default::Default;
use std::path::PathBuf;

pub use self::display::{
    set_window_icon, toggle_fullscreen_system, toggle_zoom_system, DisplayConfig,
};

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
        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, set_window_icon);

        app.add_systems(Update, toggle_fullscreen_system);

        app.add_systems(
            Update,
            toggle_zoom_system.run_if(in_state(states::AppStates::Game)),
        );
    }
}

/// Creates config file in config directory from config file in this directory
macro_rules! confgen {
    ( $($filename:expr),* ) => {
        {
            let conf_dir = current_dir().unwrap().join("config");
            if !conf_dir.is_dir() {
                DirBuilder::new()
                    .create(conf_dir.clone())
                    .expect("Confgen failed: could not create config dir.");
            }

            $({
                let default = include_bytes!($filename);
                let file_path = conf_dir.join($filename);
                if !file_path.is_file() {
                    let mut file = File::create(file_path)
                        .expect(concat!("Confgen failed: could not create config file ", $filename, "."));
                    file.write_all(default)
                        .expect(concat!("Confgen failed: could not write config file ", $filename, "."));
                }
            })*
        }
    }
}

/// Generates the display config file
pub fn generate_config_files() {
    confgen!("display.ron");
}
#[cfg(all(test, not(target_arch = "wasm32"), feature = "cli"))]
mod cli_tests {
    use argh::FromArgs;
    #[test]
    fn test_cli_parse_asset_path_dir() {
        assert_eq!(
            super::GameInitCLIOptions::from_args(&["thetawave"], &["--assets-dir", "myassets/"])
                .unwrap()
                .assets_dir,
            Some(std::path::PathBuf::from("myassets/"))
        );
    }
}
