use std::{env::current_dir, fs::read_to_string};

use bevy::prelude::{GamepadButtonType, KeyCode, Resource};
use leafwing_input_manager::prelude::InputMap;
use ron::from_str;
use serde::Deserialize;
use thetawave_interface::options::input::{MenuAction, PlayerAction};

#[derive(Deserialize)]
pub struct InputBindings {
    pub menu_keyboard: Vec<(KeyCode, MenuAction)>,
    pub menu_gamepad: Vec<(GamepadButtonType, MenuAction)>,
    pub player_keyboard: Vec<(KeyCode, PlayerAction)>,
    pub player_gamepad: Vec<(GamepadButtonType, PlayerAction)>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_input_bindings() -> InputBindings {
    let config_path = current_dir().unwrap().join("config");

    from_str::<InputBindings>(&read_to_string(config_path.join("input.ron")).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
fn get_input_bindings() -> InputBindings {
    InputBindings {
        menu_keyboard: vec![
            (KeyCode::W, MenuAction::Up),
            (KeyCode::S, MenuAction::Down),
            (KeyCode::A, MenuAction::Left),
            (KeyCode::D, MenuAction::Right),
            (KeyCode::Return, MenuAction::Confirm),
            (KeyCode::Escape, MenuAction::Back),
        ],
        menu_gamepad: vec![
            (GamepadButtonType::DPadUp, MenuAction::Up),
            (GamepadButtonType::DPadDown, MenuAction::Down),
            (GamepadButtonType::DPadLeft, MenuAction::Left),
            (GamepadButtonType::DPadRight, MenuAction::Right),
            (GamepadButtonType::South, MenuAction::Confirm),
            (GamepadButtonType::East, MenuAction::Back),
        ],
        player_keyboard: vec![
            (KeyCode::W, PlayerAction::MoveUp),
            (KeyCode::S, PlayerAction::MoveDown),
            (KeyCode::A, PlayerAction::MoveLeft),
            (KeyCode::D, PlayerAction::MoveRight),
            (KeyCode::Space, PlayerAction::BasicAttack),
            (KeyCode::ShiftLeft, PlayerAction::SpecialAttack),
        ],
        player_gamepad: vec![
            (GamepadButtonType::DPadUp, PlayerAction::MoveUp),
            (GamepadButtonType::DPadDown, PlayerAction::MoveDown),
            (GamepadButtonType::DPadLeft, PlayerAction::MoveLeft),
            (GamepadButtonType::DPadRight, PlayerAction::MoveRight),
            (GamepadButtonType::RightTrigger, PlayerAction::BasicAttack),
            (GamepadButtonType::LeftTrigger, PlayerAction::SpecialAttack),
        ],
    }
}

#[derive(Resource, Debug)]
pub struct MenuInputsResource {
    /// Menus can be controlled with any input from keyboard or gamepad
    pub menu_keyboard: InputMap<MenuAction>,
    pub menu_gamepad: InputMap<MenuAction>,
}

impl MenuInputsResource {
    pub fn new(bindings: InputBindings) -> Self {
        MenuInputsResource {
            menu_keyboard: InputMap::new(bindings.menu_keyboard),
            menu_gamepad: InputMap::new(bindings.menu_gamepad),
        }
    }
}
