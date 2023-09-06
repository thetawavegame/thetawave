use bevy_ecs::{component::Component, system::Resource};
use bevy_input::{gamepad::GamepadButtonType, keyboard::KeyCode};
use bevy_reflect::Reflect;
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use serde::Deserialize;

#[derive(Component)]
pub struct MenuExplorer;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum MenuAction {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Join,
    Back,
    Reset,
    ExitPauseMenu,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    BasicAttack,
    SpecialAttack,
    Pause,
}

#[derive(Deserialize)]
pub struct InputBindings {
    pub menu_keyboard: Vec<(KeyCode, MenuAction)>,
    pub menu_gamepad: Vec<(GamepadButtonType, MenuAction)>,
    pub player_keyboard: Vec<(KeyCode, PlayerAction)>,
    pub player_gamepad: Vec<(GamepadButtonType, PlayerAction)>,
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
            player_keyboard: InputMap::new(bindings.player_keyboard),
            player_gamepad: InputMap::new(bindings.player_gamepad),
        }
    }
}
