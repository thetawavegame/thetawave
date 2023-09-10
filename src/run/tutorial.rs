use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::Deserialize;
use thetawave_interface::options::input::PlayerAction;

use crate::player::PlayerComponent;

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

            info!(
                "{} {} {} {} {} {} {} {}",
                up_timer.finished(),
                down_timer.finished(),
                left_timer.finished(),
                right_timer.finished(),
                up_left_timer.finished(),
                up_right_timer.finished(),
                down_left_timer.finished(),
                down_right_timer.finished()
            );

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
