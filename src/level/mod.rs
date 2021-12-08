use bevy::prelude::*;
use serde::Deserialize;

use crate::spawnable::FormationPool;

pub struct Level {
    /// Timeline
    pub timeline: LevelTimeline,
    /// Tracks time of phases
    pub phase_timer: Timer,
    // TODO: Implement objectives
}

pub struct LevelTimeline {
    phases: Vec<LevelPhase>,
}

pub struct LevelPhase {
    phase_type: LevelPhaseType,
}

pub enum LevelPhaseType {
    FormationSpawn {
        time: f32,
        formation_pool: FormationPool,
    },
    Break {
        time: f32,
    },
}
