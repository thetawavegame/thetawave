use bevy_ecs::{component::Component, system::Resource};
use bevy_input::{gamepad::GamepadButtonType, keyboard::KeyCode, mouse::MouseButton};
use bevy_reflect::Reflect;
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use serde::Deserialize;

#[derive(Component)]
pub struct MenuExplorer;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum MenuAction {
    Confirm,
    JoinKeyboard,
    ChangeCharacterKeyboard,
    JoinGamepad,
    ChangeCharacterGamepad,
    Back,
    Reset,
    ExitPauseMenu,
    PauseGame,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    BasicAttack,
    SpecialAttack,
}

#[derive(Deserialize)]
pub struct InputBindings {
    pub menu_keyboard: Vec<(KeyCode, MenuAction)>,
    pub menu_gamepad: Vec<(GamepadButtonType, MenuAction)>,
    pub player_keyboard: Vec<(KeyCode, PlayerAction)>,
    pub player_gamepad: Vec<(GamepadButtonType, PlayerAction)>,
    pub player_mouse: Vec<(MouseButton, PlayerAction)>,
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_input_bindings() -> InputBindings {
    use ron::from_str;
    use std::{env::current_dir, fs::read_to_string};

    let config_path = current_dir().unwrap().join("config");

    from_str::<InputBindings>(&read_to_string(config_path.join("input.ron")).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
fn get_input_bindings() -> InputBindings {
    use ron::de::from_bytes;

    from_bytes::<InputBindings>(include_bytes!("input.ron")).unwrap()
}

#[derive(Resource, Debug)]
pub struct InputsResource {
    pub menu: InputMap<MenuAction>,
    pub player_keyboard: InputMap<PlayerAction>,
    pub player_gamepad: InputMap<PlayerAction>,
}

impl InputsResource {
    pub fn new(bindings: InputBindings) -> Self {
        InputsResource {
            menu: InputMap::new(bindings.menu_keyboard)
                .insert_multiple(bindings.menu_gamepad)
                .to_owned(),
            player_keyboard: InputMap::new(bindings.player_keyboard)
                .insert_multiple(bindings.player_mouse)
                .to_owned(),
            player_gamepad: InputMap::new(bindings.player_gamepad),
        }
    }
}
