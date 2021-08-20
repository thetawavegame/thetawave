use bevy::{prelude::*, window::WindowMode};
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
                WindowMode::Fullscreen { use_size: false }
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
        window.set_mode(match window.mode() {
            WindowMode::Fullscreen { .. } => WindowMode::Windowed,
            WindowMode::Windowed => WindowMode::Fullscreen { use_size: false },
            _ => window.mode(),
        });
    }
}
