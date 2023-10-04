use crate::run::tutorial::TutorialLesson;
use bevy::ecs::system::SystemParam;
use bevy::math::Vec2;
use bevy::prelude::Timer;
use serde::Deserialize;
use thetawave_interface::spawnable::MobType;

/// Describes a distinct portion of the level
#[derive(Deserialize, Clone, Debug)]
pub enum LevelPhaseType {
    FormationSpawn {
        phase_timer: Timer,
        spawn_timer: Timer,
        formation_pool: String,
    },
    Break {
        phase_timer: Timer,
    },
    Boss {
        mob_type: MobType,
        position: Vec2,
        spawn_timer: Timer,
    },
    Tutorial {
        tutorial_lesson: TutorialLesson,
    },
}

impl LevelPhaseType {
    pub fn get_name(&self) -> String {
        match self {
            LevelPhaseType::FormationSpawn { .. } => "Formation Invasion".to_string(),
            LevelPhaseType::Break { .. } => "Break".to_string(),
            LevelPhaseType::Boss { mob_type, .. } => format!("Boss: {}", mob_type.get_name()),
            LevelPhaseType::Tutorial {
                tutorial_lesson, ..
            } => format!("Tutorial: {}", tutorial_lesson.get_name()),
        }
    }
}
