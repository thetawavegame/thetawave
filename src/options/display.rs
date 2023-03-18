use crate::game::GameParametersResource;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};
use serde::Deserialize;

/// Display settings of the window
#[derive(Deserialize)]
pub struct DisplayConfig {
    /// Width of window
    pub width: f32,
    /// Height of window
    pub height: f32,
    /// True of fullsceen, false if windowed
    pub fullscreen: bool,
}

impl From<DisplayConfig> for Window {
    fn from(display_config: DisplayConfig) -> Self {
        Window {
            title: "Thetawave".to_string(),
            resolution: (display_config.width, display_config.height).into(),
            resizable: false,
            mode: if display_config.fullscreen {
                WindowMode::SizedFullscreen
            } else {
                WindowMode::Windowed
            },
            ..Default::default()
        }
    }
}

// TODO: fix this function, doesn't toggle back to windowed correctly
/// Toggles the window between full screen and windowed on key press
pub fn toggle_fullscreen_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // get primary window
    //let window = windows.get_primary_mut().unwrap();
    let mut primary_window = window_query.get_single_mut().unwrap();
    // get input for toggling full screen
    let fullscreen_input = keyboard_input.just_released(KeyCode::F);

    // set window mode to the mode it's not in
    if fullscreen_input {
        let new_mode = match primary_window.mode {
            WindowMode::BorderlessFullscreen { .. } => {
                primary_window.set_maximized(false);
                WindowMode::Windowed
            }
            WindowMode::Windowed => {
                primary_window.set_maximized(true);
                WindowMode::BorderlessFullscreen
            }
            _ => primary_window.mode,
        };

        primary_window.mode = new_mode;
    }
}

/// Toggles a zoomed out camera perspective on key press
pub fn toggle_zoom_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<Camera2d>>,
    game_parameters: Res<GameParametersResource>,
) {
    // get input for toggling zoom
    let zoom_input = keyboard_input.just_released(KeyCode::V);

    // toggle zoom to opposite scale
    if zoom_input {
        for mut proj in camera_query.iter_mut() {
            if proj.scale == 1.0 {
                proj.scale = game_parameters.camera_zoom_out_scale;
            } else {
                proj.scale = 1.0;
            }
        }
    }
}
