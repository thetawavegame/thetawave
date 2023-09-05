use std::{env::current_dir, fs::read_to_string};

use bevy::prelude::*;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};
use ron::from_str;
use serde::Deserialize;
use thetawave_interface::{
    options::input::{MenuAction, MenuExplorer, PlayerAction},
    player::PlayersResource,
    states::GameCleanup,
};

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
            (KeyCode::ShiftLeft, MenuAction::Join),
            (KeyCode::Return, MenuAction::Confirm),
            (KeyCode::Escape, MenuAction::Back),
            (KeyCode::R, MenuAction::Reset),
            (KeyCode::Escape, MenuAction::ExitPauseMenu),
        ],
        menu_gamepad: vec![
            (GamepadButtonType::DPadUp, MenuAction::Up),
            (GamepadButtonType::DPadDown, MenuAction::Down),
            (GamepadButtonType::DPadLeft, MenuAction::Left),
            (GamepadButtonType::DPadRight, MenuAction::Right),
            (GamepadButtonType::Start, MenuAction::Confirm),
            (GamepadButtonType::South, MenuAction::Join),
            (GamepadButtonType::East, MenuAction::Back),
            (GamepadButtonType::East, MenuAction::Reset),
            (GamepadButtonType::Start, MenuAction::ExitPauseMenu),
        ],
        player_keyboard: vec![
            (KeyCode::W, PlayerAction::MoveUp),
            (KeyCode::S, PlayerAction::MoveDown),
            (KeyCode::A, PlayerAction::MoveLeft),
            (KeyCode::D, PlayerAction::MoveRight),
            (KeyCode::Space, PlayerAction::BasicAttack),
            (KeyCode::ShiftLeft, PlayerAction::SpecialAttack),
            (KeyCode::Escape, PlayerAction::Pause),
        ],
        player_gamepad: vec![
            (GamepadButtonType::DPadUp, PlayerAction::MoveUp),
            (GamepadButtonType::DPadDown, PlayerAction::MoveDown),
            (GamepadButtonType::DPadLeft, PlayerAction::MoveLeft),
            (GamepadButtonType::DPadRight, PlayerAction::MoveRight),
            (GamepadButtonType::RightTrigger, PlayerAction::BasicAttack),
            (GamepadButtonType::LeftTrigger, PlayerAction::SpecialAttack),
            (GamepadButtonType::Start, PlayerAction::Pause),
        ],
    }
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

/// Spawns entity to track navigation over menus
pub fn spawn_menu_explorer_system(mut commands: Commands, inputs_res: Res<InputsResource>) {
    commands
        .spawn(InputManagerBundle::<MenuAction> {
            action_state: ActionState::default(),
            input_map: inputs_res.menu.clone(),
        })
        .insert(MenuExplorer);
}

#[derive(Component)]
pub enum PlayerControllerComponent {
    One,
    Two,
    Three,
    Four,
}

impl PlayerControllerComponent {
    pub fn get_from_idx(i: usize) -> Self {
        if i == 0 {
            return PlayerControllerComponent::One;
        } else if i == 1 {
            return PlayerControllerComponent::Two;
        } else if i == 2 {
            return PlayerControllerComponent::Three;
        } else if i == 3 {
            return PlayerControllerComponent::Four;
        }

        panic!("More than four players registered");
    }
}

pub fn spawn_player_controllers_system(
    mut commands: Commands,
    inputs_res: Res<InputsResource>,
    players_res: Res<PlayersResource>,
) {
    for (i, input) in players_res.player_inputs.iter().enumerate() {
        if let Some(player_input) = input {
            info!("Player controller spawned");
            commands
                .spawn(PlayerControllerComponent::get_from_idx(i))
                .insert(InputManagerBundle::<PlayerAction> {
                    action_state: ActionState::default(),
                    input_map: match player_input {
                        thetawave_interface::player::PlayerInput::Keyboard => {
                            inputs_res.player_keyboard.clone()
                        }
                        thetawave_interface::player::PlayerInput::Gamepad(id) => inputs_res
                            .player_gamepad
                            .clone()
                            .set_gamepad(Gamepad { id: *id })
                            .build(),
                    },
                })
                .insert(GameCleanup);
        }
    }
}

pub fn read_menu_actions(query: Query<&ActionState<MenuAction>, With<MenuExplorer>>) {
    let action_state = query.single();

    if action_state.just_pressed(MenuAction::Back) {
        info!("Menu back");
    } else if action_state.just_pressed(MenuAction::Up) {
        info!("Menu up");
    }
}
