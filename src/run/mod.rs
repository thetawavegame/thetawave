use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::time::Duration;

use crate::{
    arena::MobReachedBottomGateEvent, assets::GameAudioAssets, audio, states::AppStates,
    ui::EndGameTransitionResource,
};

mod formation;
mod level;

use self::level::LevelType;
pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{
        level_system, next_level_system, setup_first_level, LevelCompletedEvent, LevelsResource,
        LevelsResourceData, ObjectiveType,
    },
};

// TODO: set to a progression of levels
/// Right now just set to one level
pub type RunResourceData = level::LevelType;

#[derive(Resource)]
pub struct RunResource {
    /// Type of the level
    pub level_type: LevelType,
    /// Level struct itself
    pub level: Option<level::Level>,
}

impl From<RunResourceData> for RunResource {
    fn from(resource_data: RunResourceData) -> Self {
        RunResource {
            level_type: resource_data,
            level: None,
        }
    }
}

impl RunResource {
    /// Create the level from the level type
    pub fn create_level(&mut self, levels_resource: &level::LevelsResource) {
        self.level = Some(
            levels_resource
                .levels
                .get(&self.level_type)
                .unwrap()
                .clone(),
        );
    }

    /// Returns true if the level is ready to start
    pub fn is_ready(&self) -> bool {
        self.level.is_some()
    }

    /// Progress the run, right noew just ticks the level
    #[allow(clippy::too_many_arguments)]
    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        level_completed: &mut EventWriter<level::LevelCompletedEvent>,
        mob_reached_bottom: &mut EventReader<MobReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
        end_game_trans_resource: &mut EndGameTransitionResource,
    ) {
        if let Some(level) = &mut self.level {
            level.tick(
                delta,
                spawn_formation,
                level_completed,
                mob_reached_bottom,
                formation_pools,
                end_game_trans_resource,
            );
        }
    }
}

/// Restarts the run reseting all of the values in the game
pub fn reset_run_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
) {
    // get input
    let mut reset = keyboard_input.just_released(KeyCode::R);

    for gamepad in gamepads.iter() {
        reset |= gamepad_input.just_released(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::East,
        });
    }

    // if reset input provided reset th run
    if reset {
        // go to the main menu state
        app_state.replace(AppStates::MainMenu).unwrap();

        // play menu input sound
        // TODO: change to using loaded assets
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));

        // reset the input
        keyboard_input.reset(KeyCode::R);
        for gamepad in gamepads.iter() {
            gamepad_input.reset(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::East,
            });
        }
    }
}
