//! The structures used for knowing which player inputs from controller/keyboard+mouse/etc map to
//! known game actions.
use bevy_ecs::{component::Component, system::Resource};
use bevy_reflect::Reflect;
use leafwing_input_manager::{prelude::InputMap, Actionlike};
use serde::Deserialize;

#[derive(Component)]
pub struct MenuExplorer;

/// The input behaviors from the controller/gamepad available while in the menus.
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
    ToggleTutorial,
    NavigateUp,
    NavigateDown,
    /// When pressed, show a form for the user to edit game options.
    OptionsMenu,
}

/// Player actions during the main game/while fighting mobs. Many of these can be simultaneously
/// accepted from the gamepad/controller.
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect, Deserialize)]
pub enum PlayerAction {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    BasicAttack,
    SpecialAttack,
}

/// The parsed input/key bindings used for the life of the  entire game. This is read from files/
/// compiled files very early in the game startup since this must exist in the world before we can
/// accept player/user input.
#[derive(Resource, Debug)]
pub struct InputsResource {
    pub menu: InputMap<MenuAction>,
    pub player_keyboard: InputMap<PlayerAction>,
    pub player_gamepad: InputMap<PlayerAction>,
}
