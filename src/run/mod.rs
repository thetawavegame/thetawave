use bevy::prelude::*;
use std::time::Duration;

use crate::arena::EnemyReachedBottomGateEvent;

mod formation;
mod level;

pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{
        level_system, next_level_system, LevelCompletedEvent, LevelsResource, LevelsResourceData,
        ObjectiveType,
    },
};

pub type RunResourceData = Vec<level::LevelType>;

pub struct RunResource {
    pub level_idx: usize,
    pub level_types: Vec<level::LevelType>,
    pub levels: Vec<level::Level>,
}

impl From<RunResourceData> for RunResource {
    fn from(resource_data: RunResourceData) -> Self {
        RunResource {
            level_idx: 0,
            level_types: resource_data,
            levels: vec![],
        }
    }
}

impl RunResource {
    pub fn create_levels(&mut self, levels_resource: &level::LevelsResource) {
        for level_type in self.level_types.iter() {
            self.levels
                .push(levels_resource.levels.get(level_type).unwrap().clone());
        }
    }

    pub fn is_ready(&self) -> bool {
        self.level_types.len() == self.levels.len()
    }

    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        level_completed: &mut EventWriter<level::LevelCompletedEvent>,
        enemy_reached_bottom: &mut EventReader<EnemyReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
    ) {
        // remove this and create a vector of levels on startup
        self.levels[self.level_idx].tick(
            delta,
            spawn_formation,
            level_completed,
            enemy_reached_bottom,
            formation_pools,
        );
    }
}
