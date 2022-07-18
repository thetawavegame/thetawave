use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    MainMenu,
    PauseMenu,
    Game,
    GameOver,
}

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
