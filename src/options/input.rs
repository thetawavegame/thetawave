use bevy::prelude::*;
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use thetawave_interface::options::input::{InputsResource, MenuAction, MenuExplorer};

/// Spawns entity to track navigation over menus
pub fn spawn_menu_explorer_system(mut commands: Commands, inputs_res: Res<InputsResource>) {
    commands
        .spawn(InputManagerBundle::<MenuAction> {
            action_state: ActionState::default(),
            input_map: inputs_res.menu.clone(),
        })
        .insert(MenuExplorer);
}
