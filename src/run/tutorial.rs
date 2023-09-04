use bevy::time::Timer;
use serde::Deserialize;

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
    pub fn update(&mut self) -> bool {
        match self {
            TutorialLesson::Movement {
                up_timer,
                down_timer,
                left_timer,
                right_timer,
                up_left_timer,
                up_right_timer,
                down_left_timer,
                down_right_timer,
            } => {
                // return true if all the timers are finished
                up_timer.finished()
                    && down_timer.finished()
                    && left_timer.finished()
                    && right_timer.finished()
                    && up_left_timer.finished()
                    && up_right_timer.finished()
                    && down_left_timer.finished()
                    && down_right_timer.finished()
            }
            TutorialLesson::Attack => todo!(),
            TutorialLesson::SpecialAbility => todo!(),
        }
    }
}
