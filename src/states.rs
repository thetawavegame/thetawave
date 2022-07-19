use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    MainMenu,
    PauseMenu,
    Game,
    GameOver,
    Victory,
}

// used for tagging entities that are part of the game state
#[derive(Component)]
pub struct AppStateComponent(pub AppStates);

pub fn open_pause_menu_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let esc = keyboard_input.just_released(KeyCode::Escape);

    if esc {
        app_state.push(AppStates::PauseMenu).unwrap();
        keyboard_input.reset(KeyCode::Escape);
        rapier_config.physics_pipeline_active = false;
        rapier_config.query_pipeline_active = false;
    }
}

pub fn close_pause_menu_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let esc = keyboard_input.just_released(KeyCode::Escape);

    if esc {
        app_state.pop().unwrap();
        keyboard_input.reset(KeyCode::Escape);
        rapier_config.physics_pipeline_active = true;
        rapier_config.query_pipeline_active = true;
    }
}

pub fn start_game_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    let enter = keyboard_input.just_released(KeyCode::Return);

    if enter {
        app_state.set(AppStates::Game).unwrap();
        keyboard_input.release(KeyCode::Return);
    }
}
