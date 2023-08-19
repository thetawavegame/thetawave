use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};
use thetawave_interface::spawnable::MobType;
use thetawave_interface::states::AppStates;

use crate::{
    arena::MobReachedBottomGateEvent,
    assets::{BGMusicType, GameAudioAssets},
    audio,
    spawnable::{MobDestroyedEvent, SpawnMobEvent},
    tools::weighted_rng,
    ui::EndGameTransitionResource,
};

use super::{formation, objective::Objective, RunDefeatType, RunEndEvent, RunOutcomeType};

/// Structure stored in data file to describe level
pub type LevelsResourceData = HashMap<String, LevelData>;

/// Resource for storing defined predefined levels
#[derive(Clone, Resource)]
pub struct LevelsResource {
    /// Leveltypes maped to levels
    pub levels: HashMap<String, Level>,
}

impl From<LevelsResourceData> for LevelsResource {
    fn from(resource_data: LevelsResourceData) -> Self {
        LevelsResource {
            levels: resource_data
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

/// Level timeline for carrying phases of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelTimeline {
    /// level phases
    pub phases: Vec<LevelPhase>,
}

/// A defined section of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    /// phase type
    pub phase_type: LevelPhaseType,
    /// music to play during phase
    pub bg_music_transition: Option<BGMusicTransition>,
}

/// Background music transition
#[derive(Deserialize, Clone, Debug)]
pub struct BGMusicTransition {
    pub loop_from: f64,
    pub bg_music: BGMusicType,
}

/// Describes a distinct portion of the level
#[derive(Deserialize, Clone, Debug)]
pub enum LevelPhaseType {
    FormationSpawn {
        time: f32,
        initial_delay: f32,
        formation_pool: String,
    },
    Break {
        time: f32,
    },
    Boss {
        mob_type: MobType,
        position: Vec2,
        initial_delay: f32,
        is_defeated: bool,
    },
}

/// Data used to initialize levels
#[derive(Deserialize)]
pub struct LevelData {
    /// timeline of the phases of the level
    pub timeline: LevelTimeline,
    /// objective of the level (besides surviving)
    pub objective: Objective,
}

/// Event to alert when level has been completed
#[derive(Event)]
pub struct LevelCompletedEvent;

/// Struct to manage a level
#[derive(Clone, Debug)]
pub struct Level {
    /// Index of the current phase
    timeline_idx: usize,
    /// Timeline
    pub timeline: LevelTimeline,
    /// Tracks time of phases
    pub phase_timer: Option<Timer>,
    /// Tracks time between spawns
    pub spawn_timer: Option<Timer>,
    /// Level objective
    pub objective: Objective,
}

impl From<LevelData> for Level {
    fn from(data: LevelData) -> Self {
        Level {
            timeline_idx: 0,
            timeline: data.timeline,
            phase_timer: None,
            spawn_timer: None,
            objective: data.objective,
        }
    }
}
