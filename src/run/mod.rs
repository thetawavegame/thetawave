use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use ron::de::from_bytes;
use std::{collections::VecDeque, time::Duration};
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
mod objective;

pub use self::objective::Objective;
pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{LevelCompletedEvent, LevelsResource, LevelsResourceData},
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
            .add_event::<LevelCompletedEvent>()
            .add_event::<RunEndEvent>();

        /*
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
        */
        /*
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
        */
    }
}

pub enum RunOutcomeType {
    Victory,
    Defeat(RunDefeatType),
}

pub enum RunDefeatType {
    PlayersDestroyed,
    DefenseDestroyed,
}

#[derive(Event)]
pub struct RunEndEvent {
    pub outcome: RunOutcomeType,
}

// TODO: set to a progression of levels
/// Right now just set to one level
pub type RunResourceData = VecDeque<String>;

#[derive(Resource)]
pub struct RunResource {
    /// List of string level keys that are matched to values in the levelsresource
    pub levels: VecDeque<String>,
    /// Tracks the level currently being played
    pub current_level: Option<level::Level>,
}

impl From<RunResourceData> for RunResource {
    fn from(resource_data: RunResourceData) -> Self {
        RunResource {
            levels: resource_data,
            current_level: None,
        }
    }
}

impl RunResource {}
