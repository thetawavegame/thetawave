use crate::game::GameParametersResource;
use bevy::{prelude::*, window::WindowMode};
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

impl From<DisplayConfig> for WindowDescriptor {
    fn from(display_config: DisplayConfig) -> Self {
        WindowDescriptor {
            title: "Thetawave".to_string(),
            width: display_config.width,
            height: display_config.height,
            cursor_visible: true,
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
pub fn toggle_fullscreen_system(keyboard_input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    // get primary window
    let window = windows.get_primary_mut().unwrap();
    // get input for toggling full screen
    let fullscreen_input = keyboard_input.just_released(KeyCode::F);

    // set window mode to the mode it's not in
    if fullscreen_input {
        let new_mode = match window.mode() {
            WindowMode::BorderlessFullscreen { .. } => {
                window.set_maximized(false);
                WindowMode::Windowed
            }
            WindowMode::Windowed => {
                window.set_maximized(true);
                WindowMode::BorderlessFullscreen
            }
            _ => window.mode(),
        };

        window.set_mode(new_mode);
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
