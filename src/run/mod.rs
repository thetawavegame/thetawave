use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use ron::de::from_bytes;
use std::time::Duration;
use thetawave_interface::states::{AppStates, GameStates};

use crate::{
    arena::MobReachedBottomGateEvent,
    assets::GameAudioAssets,
    audio,
    player::PlayersResource,
    spawnable::{MobDestroyedEvent, SpawnMobEvent},
    states::{self},
    ui::EndGameTransitionResource,
    GameEnterSet, GameUpdateSet,
};

mod formation;
mod level;

pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{
        level_system, next_level_system, setup_first_level, LevelCompletedEvent, LevelsResource,
        LevelsResourceData, ObjectiveType,
    },
};

pub struct RunPlugin;

impl Plugin for RunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<FormationPoolsResource>(include_bytes!(
                "../../assets/data/formation_pools.ron"
            ))
            .unwrap(),
        )
        .insert_resource(RunResource::from(
            from_bytes::<RunResourceData>(include_bytes!("../../assets/data/run.ron")).unwrap(),
        ))
        .insert_resource(LevelsResource::from(
            from_bytes::<LevelsResourceData>(include_bytes!("../../assets/data/levels.ron"))
                .unwrap(),
        ));

        app.add_event::<SpawnFormationEvent>()
            .add_event::<LevelCompletedEvent>();

        app.add_systems(
            OnEnter(states::AppStates::Game),
            setup_first_level.in_set(GameEnterSet::BuildLevel),
        );

        app.add_systems(
            Update,
            (
                level_system.in_set(GameUpdateSet::Level),
                spawn_formation_system.in_set(GameUpdateSet::Spawn),
                next_level_system.in_set(GameUpdateSet::NextLevel),
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );

        app.add_systems(
            Update,
            reset_run_system.run_if(in_state(states::AppStates::GameOver)),
        );

        app.add_systems(
            Update,
            reset_run_system.run_if(in_state(states::AppStates::Victory)),
        );

        app.add_systems(
            Update,
            reset_run_system.run_if(in_state(states::GameStates::Paused)),
        );
    }
}

// TODO: set to a progression of levels
/// Right now just set to one level
pub type RunResourceData = String;

#[derive(Resource)]
pub struct RunResource {
    /// Type of the level
    pub level_type: String,
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
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        mob_destroyed_event_reader: &mut EventReader<MobDestroyedEvent>,
        mob_reached_bottom: &mut EventReader<MobReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
        end_game_trans_resource: &mut EndGameTransitionResource,
        audio_channel: &AudioChannel<audio::BackgroundMusicAudioChannel>,
        audio_assets: &GameAudioAssets,
    ) {
        if let Some(level) = &mut self.level {
            level.tick(
                delta,
                spawn_formation,
                level_completed,
                spawn_mob_event_writer,
                mob_destroyed_event_reader,
                mob_reached_bottom,
                formation_pools,
                end_game_trans_resource,
                audio_channel,
                audio_assets,
            );
        }
    }
}

/// Restarts the run reseting all of the values in the game
#[allow(clippy::too_many_arguments)]
pub fn reset_run_system(
    gamepads: Res<Gamepads>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::MenuAudioChannel>>,
    bg_auido_channel: Res<AudioChannel<audio::BackgroundMusicAudioChannel>>,
    mut players_resource: ResMut<PlayersResource>,
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
        next_app_state.set(AppStates::MainMenu);
        next_game_state.set(GameStates::Playing);
        *players_resource = PlayersResource::default();

        // play menu input sound
        // TODO: change to using loaded assets
        audio_channel.play(asset_server.load("sounds/menu_input_success.wav"));
        bg_auido_channel.stop();

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
