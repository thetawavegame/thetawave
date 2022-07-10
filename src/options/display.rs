use crate::game::GameParametersResource;
use bevy::{prelude::*, render::camera::Camera2d, window::WindowMode};
use serde::Deserialize;

/// Display settings of the window
#[derive(Deserialize)]
pub struct DisplayConfig {
    /// Width of window
    width: f32,
    /// Height of window
    height: f32,
    /// True of fullsceen, false if windowed
    fullscreen: bool,
}

impl From<DisplayConfig> for WindowDescriptor {
    fn from(display_config: DisplayConfig) -> Self {
        WindowDescriptor {
            title: "Theta Wave".to_string(),
            width: display_config.width,
            height: display_config.height,
            mode: if display_config.fullscreen {
                WindowMode::BorderlessFullscreen
            } else {
                WindowMode::Windowed
            },
            ..Default::default()
        }
    }
}

/// Toggles the window between full screen and windowed on key press
pub fn toggle_fullscreen_system(keyboard_input: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let fullscreen_input = keyboard_input.just_released(KeyCode::F);

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
    let fullscreen_input = keyboard_input.just_released(KeyCode::V);

    if fullscreen_input {
        for mut proj in camera_query.iter_mut() {
            if proj.scale == 1.0 {
                proj.scale = game_parameters.camera_zoom_out_scale;
            } else {
                proj.scale = 1.0;
            }
        }
    }
}
