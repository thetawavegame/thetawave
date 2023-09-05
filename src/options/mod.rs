//! `thetawave` player module
use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use thetawave_interface::{options::input::MenuAction, states::AppStates};

mod display;
mod input;

use crate::states;

pub use self::display::{
    set_window_icon, toggle_fullscreen_system, toggle_zoom_system, DisplayConfig,
};
use self::input::{
    get_input_bindings, read_menu_actions, spawn_menu_explorer_system,
    spawn_player_controllers_system, InputsResource,
};

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());

        app.insert_resource(InputsResource::new(get_input_bindings()));

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, (set_window_icon, spawn_menu_explorer_system));

        app.add_systems(Update, toggle_fullscreen_system);

        app.add_systems(
            Update,
            toggle_zoom_system.run_if(in_state(states::AppStates::Game)),
        );

        app.add_systems(
            Update,
            read_menu_actions.run_if(in_state(AppStates::MainMenu)),
        );

        app.add_systems(OnEnter(AppStates::Game), spawn_player_controllers_system);
    }
}
