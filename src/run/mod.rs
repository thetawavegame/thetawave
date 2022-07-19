use bevy::prelude::*;
use std::time::Duration;

use crate::{
    arena::EnemyReachedBottomGateEvent, game_over::EndGameTransitionResource, states::AppStates,
};

mod formation;
mod level;

use self::level::LevelType;
pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{
        level_system, next_level_system, LevelCompletedEvent, LevelsResource, LevelsResourceData,
        ObjectiveType,
    },
};

pub type RunResourceData = level::LevelType;

pub struct RunResource {
    //pub level_idx: usize,
    pub level_type: LevelType,
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
    pub fn create_level(&mut self, levels_resource: &level::LevelsResource) {
        self.level = Some(
            levels_resource
                .levels
                .get(&self.level_type)
                .unwrap()
                .clone(),
        );
    }

    pub fn is_ready(&self) -> bool {
        self.level.is_some()
    }

    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        level_completed: &mut EventWriter<level::LevelCompletedEvent>,
        enemy_reached_bottom: &mut EventReader<EnemyReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
    ) {
        if let Some(level) = &mut self.level {
            level.tick(
                delta,
                spawn_formation,
                level_completed,
                enemy_reached_bottom,
                formation_pools,
            );
        }
    }
}

pub fn reset_run_system(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<AppStates>>,
) {
    let reset = keyboard_input.just_released(KeyCode::R);

    if reset {
        app_state.set(AppStates::MainMenu).unwrap();
        keyboard_input.reset(KeyCode::R);
    }
}
