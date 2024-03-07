//! The logic for the tutorial level. Generally, a `TutorialLesson` is added to a collection of
//! levels to invoke some behavior each tick, until all objectives/milestones are met (a sentinel
//! in the tick/update function is returned).
use bevy::math::Quat;
use bevy::prelude::{EventReader, EventWriter, Query, Time, Timer, With};
use leafwing_input_manager::action_state::ActionState;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::ops::Range;
use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::objective::MobReachedBottomGateEvent;
use thetawave_interface::player::{
    InputRestrictionsAtSpawn, PlayerAbilitiesComponent, PlayerComponent,
};
use thetawave_interface::spawnable::{
    AllyMobType, MobDestroyedEvent, MobSegmentDestroyedEvent, MobSegmentType, MobType,
    NeutralMobSegmentType, NeutralMobType, SpawnMobEvent,
};
use thetawave_interface::weapon::WeaponComponent;

fn enable_player_actions_at_end_of_phase(
    players: &mut Query<(&mut PlayerAbilitiesComponent, &mut WeaponComponent)>,
) {
    for (mut player_abilities, mut weapon) in players.iter_mut() {
        player_abilities.enable_special_attacks();
        weapon.enable();
    }
}
pub(super) fn modify_player_spawn_params_for_lesson_phase(
    spawn_params: &mut InputRestrictionsAtSpawn,
    lesson: &TutorialLesson,
) {
    match lesson {
        TutorialLesson::Movement { .. } => {
            (*spawn_params).forbid_main_attack_reason = Some("In movement tutorial".into());
            (*spawn_params).forbid_special_attack_reason = Some("In movement tutorial".into());
        }
        TutorialLesson::Attack { .. } => {
            (*spawn_params).forbid_special_attack_reason = Some("In attack tutorial".into());
        }
        TutorialLesson::Ability { .. } => {
            (*spawn_params).forbid_main_attack_reason = Some("In ability tutorial".into());
        }
    }
}

/// The state of the player's tutorial. Methods update this (and transition to different tutorial
/// state variants) for each game tick until the tutorial is complete.
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
    Attack {
        mobs_to_destroy: usize,
        mobs_to_protect: usize,
        initial_spawn_timer: Timer,
        spawn_range_x: Range<f32>,
        spawn_y: f32,
    },
    Ability {
        mobs_to_destroy: usize,
        initial_spawn_timer: Timer,
        spawn_range_x: Range<f32>,
        spawn_y: f32,
    },
}

impl TutorialLesson {
    pub fn get_name(&self) -> String {
        match self {
            TutorialLesson::Movement { .. } => "Movement".to_string(),
            TutorialLesson::Attack { .. } => "Attack".to_string(),
            TutorialLesson::Ability { .. } => "Ability".to_string(),
        }
    }

    pub fn get_ability_strs(&self) -> Vec<(String, bool)> {
        vec![if let Self::Ability {
            mobs_to_destroy, ..
        } = self
        {
            (
                format!("Destroy drones: {}", mobs_to_destroy,),
                *mobs_to_destroy == 0,
            )
        } else {
            ("".to_string(), false)
        }]
    }

    pub fn get_attack_strs(&self) -> Vec<(String, bool)> {
        vec![
            self.get_mobs_to_destroy_str(),
            self.get_mobs_to_protect_str(),
        ]
    }

    fn get_mobs_to_destroy_str(&self) -> (String, bool) {
        if let Self::Attack {
            mobs_to_destroy, ..
        } = self
        {
            (
                format!("Destroy drones: {}", mobs_to_destroy,),
                *mobs_to_destroy == 0,
            )
        } else {
            ("".to_string(), false)
        }
    }

    fn get_mobs_to_protect_str(&self) -> (String, bool) {
        if let Self::Attack {
            mobs_to_protect, ..
        } = self
        {
            (
                format!("Protect haulers: {}", mobs_to_protect,),
                *mobs_to_protect == 0,
            )
        } else {
            ("".to_string(), false)
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

    fn get_up_timer_progress_str(&self) -> (String, bool) {
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

    fn get_down_timer_progress_str(&self) -> (String, bool) {
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

    fn get_left_timer_progress_str(&self) -> (String, bool) {
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

    fn get_right_timer_progress_str(&self) -> (String, bool) {
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

    fn get_up_left_timer_progress_str(&self) -> (String, bool) {
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

    fn get_up_right_timer_progress_str(&self) -> (String, bool) {
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

    fn get_down_left_timer_progress_str(&self) -> (String, bool) {
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

    fn get_down_right_timer_progress_str(&self) -> (String, bool) {
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
        player_query: &Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
        mob_destroyed_event: &mut EventReader<MobDestroyedEvent>,
        time: &Time,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        mob_reached_bottom_event: &mut EventReader<MobReachedBottomGateEvent>,
        mob_segment_destroyed_event: &mut EventReader<MobSegmentDestroyedEvent>,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
        players: &mut Query<(&mut PlayerAbilitiesComponent, &mut WeaponComponent)>,
    ) -> bool {
        self.disable_player_actions_for_current_phase(players);
        // tutorial will only be run for single player
        if let Ok(action_state) = player_query.get_single() {
            let finished_tutorial_phase = match self {
                TutorialLesson::Attack { .. } => self.attack_tutorial(
                    mob_destroyed_event,
                    time,
                    spawn_mob_event_writer,
                    mob_reached_bottom_event,
                    mob_segment_destroyed_event,
                    play_sound_effect_event_writer,
                ),
                TutorialLesson::Ability { .. } => self.ability_tutorial(
                    mob_destroyed_event,
                    time,
                    spawn_mob_event_writer,
                    mob_reached_bottom_event,
                    play_sound_effect_event_writer,
                ),
                TutorialLesson::Movement { .. } => {
                    self.movement_tutorial(action_state, time, play_sound_effect_event_writer)
                }
            };
            if finished_tutorial_phase {
                enable_player_actions_at_end_of_phase(players);
            }
            finished_tutorial_phase
        } else {
            false
        }
    }
    fn disable_player_actions_for_current_phase(
        &self,
        players: &mut Query<(&mut PlayerAbilitiesComponent, &mut WeaponComponent)>,
    ) {
        for (mut player_abilities, mut weapon) in players.iter_mut() {
            match self {
                Self::Ability { .. } => {
                    weapon.disable();
                }
                Self::Attack { .. } => {
                    player_abilities.disable_special_attacks();
                }
                Self::Movement { .. } => {
                    weapon.disable();
                    player_abilities.disable_special_attacks();
                }
            }
        }
    }

    fn attack_tutorial(
        &mut self,
        mob_destroyed_event: &mut EventReader<MobDestroyedEvent>,
        time: &Time,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        mob_reached_bottom_event: &mut EventReader<MobReachedBottomGateEvent>,
        mob_segment_destroyed_event: &mut EventReader<MobSegmentDestroyedEvent>,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    ) -> bool {
        if let TutorialLesson::Attack {
            mobs_to_destroy,
            mobs_to_protect,
            initial_spawn_timer,
            spawn_range_x,
            spawn_y,
        } = self
        {
            // spawn initial mob
            initial_spawn_timer.tick(time.delta());
            if initial_spawn_timer.just_finished() {
                spawn_mob_event_writer.send(SpawnMobEvent {
                    mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                    position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y).into(),
                    rotation: Quat::default(),
                    boss: false,
                });
            }

            // check if mob was destroyed
            for event in mob_destroyed_event.read() {
                if matches!(
                    event.mob_type,
                    MobType::Neutral(NeutralMobType::TutorialDrone)
                ) {
                    // update mobs left to destroy
                    *mobs_to_destroy = mobs_to_destroy.checked_sub(1).unwrap_or(0);

                    // spawn another mob if more are left
                    if *mobs_to_destroy != 0 {
                        spawn_mob_event_writer.send(SpawnMobEvent {
                            mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                            position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                .into(),
                            rotation: Quat::default(),
                            boss: false,
                        });
                    } else if *mobs_to_protect > 0 {
                        play_sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::ObjectiveCompleted,
                        });

                        spawn_mob_event_writer.send(SpawnMobEvent {
                            mob_type: MobType::Ally(AllyMobType::TutorialHauler2),
                            position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                .into(),
                            rotation: Quat::default(),
                            boss: false,
                        });
                    }
                }
            }

            for event in mob_segment_destroyed_event.read() {
                if matches!(
                    event.mob_segment_type,
                    MobSegmentType::Neutral(NeutralMobSegmentType::TutorialHaulerBack)
                ) && *mobs_to_protect != 0
                {
                    spawn_mob_event_writer.send(SpawnMobEvent {
                        mob_type: MobType::Ally(AllyMobType::TutorialHauler2),
                        position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y).into(),
                        rotation: Quat::default(),
                        boss: false,
                    });
                }
            }

            // check if mob reached the bottom of the arena
            for event in mob_reached_bottom_event.read() {
                if let Some(mob_segment_type) = &event.mob_segment_type {
                    if matches!(
                        mob_segment_type,
                        MobSegmentType::Neutral(NeutralMobSegmentType::TutorialHaulerBack)
                    ) {
                        *mobs_to_protect = mobs_to_protect.checked_sub(1).unwrap_or(0);

                        // spawn another mob if more are left
                        if *mobs_to_protect != 0 {
                            spawn_mob_event_writer.send(SpawnMobEvent {
                                mob_type: MobType::Ally(AllyMobType::TutorialHauler2),
                                position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                    .into(),
                                rotation: Quat::default(),
                                boss: false,
                            });
                        } else {
                            play_sound_effect_event_writer.send(PlaySoundEffectEvent {
                                sound_effect_type: SoundEffectType::ObjectiveCompleted,
                            });
                        }
                    }
                } else if let Some(mob_type) = &event.mob_type {
                    if matches!(mob_type, MobType::Neutral(NeutralMobType::TutorialDrone)) {
                        spawn_mob_event_writer.send(SpawnMobEvent {
                            mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                            position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                .into(),
                            rotation: Quat::default(),
                            boss: false,
                        });
                    }
                }
            }

            // return true if there are no more mobs to destroy or protect
            *mobs_to_destroy == 0 && *mobs_to_protect == 0
        } else {
            false
        }
    }

    fn ability_tutorial(
        &mut self,
        mob_destroyed_event: &mut EventReader<MobDestroyedEvent>,
        time: &Time,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        mob_reached_bottom_event: &mut EventReader<MobReachedBottomGateEvent>,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    ) -> bool {
        if let TutorialLesson::Ability {
            mobs_to_destroy,
            initial_spawn_timer,
            spawn_range_x,
            spawn_y,
        } = self
        {
            // spawn initial mob
            initial_spawn_timer.tick(time.delta());
            if initial_spawn_timer.just_finished() {
                spawn_mob_event_writer.send(SpawnMobEvent {
                    mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                    position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y).into(),
                    rotation: Quat::default(),
                    boss: false,
                });
            }

            // check if mob was destroyed
            for event in mob_destroyed_event.read() {
                if matches!(
                    event.mob_type,
                    MobType::Neutral(NeutralMobType::TutorialDrone)
                ) {
                    // update mobs left to destroy
                    *mobs_to_destroy = mobs_to_destroy.checked_sub(1).unwrap_or(0);

                    // spawn another mob if more are left
                    if *mobs_to_destroy != 0 {
                        spawn_mob_event_writer.send(SpawnMobEvent {
                            mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                            position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                .into(),
                            rotation: Quat::default(),
                            boss: false,
                        });
                    } else {
                        play_sound_effect_event_writer.send(PlaySoundEffectEvent {
                            sound_effect_type: SoundEffectType::ObjectiveCompleted,
                        });
                    }
                }
            }

            // check if mob reached the bottom of the arena
            for event in mob_reached_bottom_event.read() {
                if let Some(mob_type) = &event.mob_type {
                    if matches!(mob_type, MobType::Neutral(NeutralMobType::TutorialDrone)) {
                        spawn_mob_event_writer.send(SpawnMobEvent {
                            mob_type: MobType::Neutral(NeutralMobType::TutorialDrone),
                            position: (thread_rng().gen_range(spawn_range_x.clone()), *spawn_y)
                                .into(),
                            rotation: Quat::default(),
                            boss: false,
                        });
                    }
                }
            }

            // return true if there are no more mobs to destroy
            *mobs_to_destroy == 0
        } else {
            false
        }
    }

    fn movement_tutorial(
        &mut self,
        action_state: &ActionState<PlayerAction>,
        time: &Time,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
    ) -> bool {
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
            let up = action_state.pressed(&PlayerAction::MoveUp);
            let down = action_state.pressed(&PlayerAction::MoveDown);
            let left = action_state.pressed(&PlayerAction::MoveLeft);
            let right = action_state.pressed(&PlayerAction::MoveRight);

            // tick timers
            let objective_completed = if up && !down && !left && !right {
                up_timer.tick(time.delta());
                up_timer.just_finished()
            } else if !up && down && !left && !right {
                down_timer.tick(time.delta());
                down_timer.just_finished()
            } else if !up && !down && left && !right {
                left_timer.tick(time.delta());
                left_timer.just_finished()
            } else if !up && !down && !left && right {
                right_timer.tick(time.delta());
                right_timer.just_finished()
            } else if up && !down && left && !right {
                up_left_timer.tick(time.delta());
                up_left_timer.just_finished()
            } else if up && !down && !left && right {
                up_right_timer.tick(time.delta());
                up_right_timer.just_finished()
            } else if !up && down && left && !right {
                down_left_timer.tick(time.delta());
                down_left_timer.just_finished()
            } else if !up && down && !left && right {
                down_right_timer.tick(time.delta());
                down_right_timer.just_finished()
            } else {
                false
            };

            // play objective completed sound if any timer just finished
            if objective_completed {
                play_sound_effect_event_writer.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::ObjectiveCompleted,
                });
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
