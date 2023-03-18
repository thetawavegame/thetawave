use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Clone)]
pub enum InputAction {
    // Movement
    Up,
    Down,
    Left,
    Right,
    // Weapon
    FireWeapon,
    // Abilities
    Ability,
    // Menu
    Pause,
    Select,
    Back,
    Restart,
}
