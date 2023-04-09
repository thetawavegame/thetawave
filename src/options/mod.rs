//! `thetawave` player module
use bevy::prelude::*;
use std::{
    env::current_dir,
    fs::{DirBuilder, File},
    io::prelude::*,
};

mod display;

use crate::states;

pub use self::display::{toggle_fullscreen_system, toggle_zoom_system, DisplayConfig};

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(toggle_fullscreen_system);

        app.add_systems((toggle_zoom_system,).in_set(OnUpdate(states::AppStates::Game)));
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
