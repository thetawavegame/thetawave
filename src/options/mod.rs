//! `thetawave` player module
use bevy::{core_pipeline::tonemapping::Tonemapping, prelude::*};
use leafwing_input_manager::prelude::InputManagerPlugin;
use thetawave_interface::{
    game::options::GameOptions,
    input::{InputsResource, MenuAction},
    states,
};

mod display;
mod input;

use input::get_input_bindings;
use std::default::Default;
use std::env::current_dir;
use std::fs::{DirBuilder, File};
use std::io::Write;
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
    #[cfg_attr(
        all(not(target_arch = "wasm32"), feature = "arcade"),
        argh(switch, short = 'a')
    )]
    /// whether to use instructions, serial port IO, etc. specific to deploying on an arcade
    /// machine. This should almost never be enabled.
    pub arcade: bool,
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

pub fn apply_game_options_system(
    game_options: Res<GameOptions>,
    mut camera_2d_query: Query<
        (&mut Camera, &mut Tonemapping),
        (With<Camera2d>, Without<Camera3d>),
    >,
    mut camera_3d_query: Query<
        (&mut Camera, &mut Tonemapping),
        (With<Camera3d>, Without<Camera2d>),
    >,
) {
    if let (Ok((mut camera_2d, mut tonemapping_2d)), Ok((mut camera_3d, mut tonemapping_3d))) = (
        camera_2d_query.get_single_mut(),
        camera_3d_query.get_single_mut(),
    ) {
        camera_2d.hdr = game_options.bloom_enabled;
        camera_3d.hdr = game_options.bloom_enabled;

        if game_options.bloom_enabled && game_options.bloom_intensity >= 0. {
            *tonemapping_2d = Tonemapping::TonyMcMapface;
            *tonemapping_3d = Tonemapping::TonyMcMapface;
        } else {
            *tonemapping_2d = Tonemapping::None;
            *tonemapping_3d = Tonemapping::None;
        }
    } else {
        error!("Failed to get singleton 2d and 3d cameras to apply game opts");
    }
}

/// Whether we are playing on an arcade machine. This affects some different UI elements.
/// Generally this will be set at app startup (either inferred or explicitly provided as a game
/// startup parameter, and should probably not be mutated during the game.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct PlayingOnArcadeResource(bool);

#[derive(Default)]
pub struct OptionsPlugin {
    pub arcade: bool,
}

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());

        app.insert_resource(InputsResource::from(get_input_bindings()));
        app.insert_resource(PlayingOnArcadeResource(self.arcade));
        app.insert_resource(GameOptions::default());

        app.add_systems(Startup, spawn_menu_explorer_system);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, set_window_icon);

        app.add_systems(Update, toggle_fullscreen_system);

        app.add_systems(
            Update,
            toggle_zoom_system.run_if(in_state(states::AppStates::Game)),
        );

        app.add_systems(
            OnEnter(states::AppStates::MainMenu),
            apply_game_options_system,
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
    confgen!("input.ron");
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
