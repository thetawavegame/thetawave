use bevy_ecs::component::Component;
use bevy_reflect::Reflect;
use leafwing_input_manager::Actionlike;
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
