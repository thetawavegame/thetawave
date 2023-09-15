use bevy_ecs::{prelude::Event, query::With, system::Query};
use bevy_math::Vec2;
use bevy_time::{Time, Timer};
use leafwing_input_manager::prelude::ActionState;
use serde::Deserialize;

use crate::{
    character::CharacterType, options::input::PlayerAction, player::PlayerComponent,
    spawnable::MobType,
};

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

#[derive(Event)]
pub struct CyclePhaseEvent;

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
        character_type: CharacterType,
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

#[derive(Deserialize, Clone, Debug)]
pub enum TutorialLesson {
    Movement {
        up_timer: Timer,
        down_timer: Timer,
        left_timer: Timer,
        right_timer: Timer,
        up_left_timer: Timer,
        up_right_timer: Timer,
        down_left_timer: Timer,
        down_right_timer: Timer,
    },
    Attack,
    SpecialAbility,
}

impl TutorialLesson {
    pub fn get_name(&self) -> String {
        match self {
            TutorialLesson::Movement { .. } => "Movement".to_string(),
            TutorialLesson::Attack => "Attack".to_string(),
            TutorialLesson::SpecialAbility => "Special Ability".to_string(),
        }
    }

    pub fn get_movement_timer_strs(&self) -> Vec<(String, bool)> {
        vec![
            self.get_up_timer_progress_str(),
            self.get_down_timer_progress_str(),
            self.get_left_timer_progress_str(),
            self.get_right_timer_progress_str(),
            self.get_up_left_timer_progress_str(),
            self.get_up_right_timer_progress_str(),
            self.get_down_left_timer_progress_str(),
            self.get_down_right_timer_progress_str(),
        ]
    }

    pub fn get_up_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { up_timer, .. } = self {
            (
                format!(
                    "Up: {:.1}/{:.1}",
                    up_timer.elapsed_secs(),
                    up_timer.duration().as_secs_f32()
                ),
                up_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_down_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { down_timer, .. } = self {
            (
                format!(
                    "Down: {:.1}/{:.1}",
                    down_timer.elapsed_secs(),
                    down_timer.duration().as_secs_f32()
                ),
                down_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_left_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { left_timer, .. } = self {
            (
                format!(
                    "Left: {:.1}/{:.1}",
                    left_timer.elapsed_secs(),
                    left_timer.duration().as_secs_f32()
                ),
                left_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_right_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { right_timer, .. } = self {
            (
                format!(
                    "Right: {:.1}/{:.1}",
                    right_timer.elapsed_secs(),
                    right_timer.duration().as_secs_f32()
                ),
                right_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_up_left_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { up_left_timer, .. } = self {
            (
                format!(
                    "Up+Left: {:.1}/{:.1}",
                    up_left_timer.elapsed_secs(),
                    up_left_timer.duration().as_secs_f32()
                ),
                up_left_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_up_right_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement { up_right_timer, .. } = self {
            (
                format!(
                    "Up+Right: {:.1}/{:.1}",
                    up_right_timer.elapsed_secs(),
                    up_right_timer.duration().as_secs_f32()
                ),
                up_right_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_down_left_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement {
            down_left_timer, ..
        } = self
        {
            (
                format!(
                    "Down+Left: {:.1}/{:.1}",
                    down_left_timer.elapsed_secs(),
                    down_left_timer.duration().as_secs_f32()
                ),
                down_left_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn get_down_right_timer_progress_str(&self) -> (String, bool) {
        if let Self::Movement {
            down_right_timer, ..
        } = self
        {
            (
                format!(
                    "Down+Right: {:.1}/{:.1}",
                    down_right_timer.elapsed_secs(),
                    down_right_timer.duration().as_secs_f32()
                ),
                down_right_timer.finished(),
            )
        } else {
            ("".to_string(), false)
        }
    }

    pub fn update(
        &mut self,
        player_query: &mut Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
        time: &Time,
    ) -> bool {
        // tutorial will only be run for single player
        let action_state = player_query.single();

        match self {
            TutorialLesson::Attack => todo!(),
            TutorialLesson::SpecialAbility => todo!(),
            TutorialLesson::Movement { .. } => self.movement_tutorial(action_state, time),
        }
    }

    fn movement_tutorial(&mut self, action_state: &ActionState<PlayerAction>, time: &Time) -> bool {
        // return true if all the timers are finished
        if let TutorialLesson::Movement {
            up_timer,
            down_timer,
            left_timer,
            right_timer,
            up_left_timer,
            up_right_timer,
            down_left_timer,
            down_right_timer,
        } = self
        {
            let up = action_state.pressed(PlayerAction::MoveUp);
            let down = action_state.pressed(PlayerAction::MoveDown);
            let left = action_state.pressed(PlayerAction::MoveLeft);
            let right = action_state.pressed(PlayerAction::MoveRight);

            // tick timers
            if up && !down && !left && !right {
                up_timer.tick(time.delta());
            } else if !up && down && !left && !right {
                down_timer.tick(time.delta());
            } else if !up && !down && left && !right {
                left_timer.tick(time.delta());
            } else if !up && !down && !left && right {
                right_timer.tick(time.delta());
            } else if up && !down && left && !right {
                up_left_timer.tick(time.delta());
            } else if up && !down && !left && right {
                up_right_timer.tick(time.delta());
            } else if !up && down && left && !right {
                down_left_timer.tick(time.delta());
            } else if !up && down && !left && right {
                down_right_timer.tick(time.delta());
            }

            // return true if all timers are finshed
            up_timer.finished()
                && down_timer.finished()
                && left_timer.finished()
                && right_timer.finished()
                && up_left_timer.finished()
                && up_right_timer.finished()
                && down_left_timer.finished()
                && down_right_timer.finished()
        } else {
            false
        }
    }
}
