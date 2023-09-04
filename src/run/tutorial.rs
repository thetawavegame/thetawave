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
    pub fn update(&mut self) {
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
            } => {}
            TutorialLesson::Attack => todo!(),
            TutorialLesson::SpecialAbility => todo!(),
        }
    }
}
