//! Mainly exposes `Level` for moving between different phases of the same level, and progressing
//! between levels.
use crate::run::level_phase::LevelPhaseType;
use crate::run::tutorial::modify_player_spawn_params_for_lesson_phase;
use bevy::{
    log::info,
    math::Quat,
    prelude::{EventReader, EventWriter, Query, ResMut, Resource, With},
    time::{Stopwatch, Time, Timer},
};
use leafwing_input_manager::prelude::ActionState;
use serde::Deserialize;
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::InputRestrictionsAtSpawn;
use thetawave_interface::{
    audio::{BGMusicType, ChangeBackgroundMusicEvent, PlaySoundEffectEvent},
    objective::{MobReachedBottomGateEvent, Objective},
    player::PlayerComponent,
    run::CyclePhaseEvent,
    spawnable::{MobDestroyedEvent, MobSegmentDestroyedEvent, SpawnMobEvent},
};

use crate::spawnable::BossesDestroyedEvent;

use super::{FormationPoolsResource, SpawnFormationEvent};

#[derive(Resource, Deserialize)]
pub(super) struct PremadeLevelsResource {
    pub levels_data: HashMap<String, LevelData>,
}

/// A defined section of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    /// phase type
    pub phase_type: LevelPhaseType,
    /// music to play during phase
    pub bg_music_transition: Option<BGMusicTransition>,
    pub intro_text: Option<String>,
}

/// Background music transition
#[derive(Deserialize, Clone, Debug)]
pub struct BGMusicTransition {
    pub loop_from: Option<f64>,
    pub bg_music_type: Option<BGMusicType>,
    pub fade_in: Option<f32>,
    pub fade_out: Option<f32>,
}

impl From<&BGMusicTransition> for ChangeBackgroundMusicEvent {
    fn from(value: &BGMusicTransition) -> Self {
        ChangeBackgroundMusicEvent {
            bg_music_type: value.bg_music_type.clone(),
            loop_from: value.loop_from,
            fade_in: value.fade_in.map(Duration::from_secs_f32),
            fade_out: value.fade_out.map(Duration::from_secs_f32),
        }
    }
}

/// Data used to initialize levels
#[derive(Deserialize)]
pub struct LevelData {
    /// timeline of the phases of the level
    pub phases: Vec<LevelPhase>,
    /// objective of the level (besides surviving)
    pub objective: Option<Objective>,
    /// descriptive name of the level
    pub name: String,
}

pub type LevelPhases = VecDeque<LevelPhase>;

/// The state of a full level. This will be mutated while the level is being played.
#[derive(Clone, Debug)]
pub struct Level {
    /// Phases that have been completed so far in the run
    pub completed_phases: LevelPhases,
    /// Phase that is currently active
    pub current_phase: Option<LevelPhase>,
    /// Phases that have yet to be played in the level
    pub queued_phases: LevelPhases,
    /// Optional objective is an additional failure condition for a level
    pub objective: Option<Objective>,
    /// Tracks how long the player has been in the level
    pub level_time: Stopwatch,
    /// Name of the level
    pub name: String,
}

impl Level {
    pub fn get_name(&self) -> String {
        if let Some(objective) = &self.objective {
            format!("{}: {}", self.name, objective.clone().get_name())
        } else {
            self.name.clone()
        }
    }
}

impl From<&LevelData> for Level {
    fn from(data: &LevelData) -> Self {
        Level {
            completed_phases: vec![].into(),
            current_phase: None,
            queued_phases: data.phases.clone().into(),
            objective: data.objective.clone(),
            level_time: Stopwatch::new(),
            name: data.name.clone(),
        }
    }
}

impl Level {
    pub fn cycle_phase(
        &mut self,
        cycle_phase_event_writer: &mut EventWriter<CyclePhaseEvent>,
    ) -> bool {
        // "clean up" the just completed phase & push it to the back of the queue to be replayed
        if let Some(current_phase) = &self.current_phase {
            self.completed_phases.push_back(current_phase.clone());
            self.current_phase = None;
        }

        // pop the next level (if it exists) into the the current level
        self.current_phase = self.queued_phases.pop_front();

        info!("Phase cycled");

        cycle_phase_event_writer.send(CyclePhaseEvent);

        // return true if no phase was available to cycle to the current phase
        self.current_phase.is_none()
    }

    pub fn init_phase(
        &mut self,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
    ) {
        if let Some(current_phase) = &self.current_phase {
            if let Some(bg_music_transition) = &current_phase.bg_music_transition {
                // change music
                change_bg_music_event_writer
                    .send(ChangeBackgroundMusicEvent::from(bg_music_transition));
            }
        }
    }

    // returns true if level has been completed
    pub fn tick(
        &mut self,
        time: &Time,
        player_query: &Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        bosses_destroyed_event_reader: &mut EventReader<BossesDestroyedEvent>,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
        cycle_phase_event_writer: &mut EventWriter<CyclePhaseEvent>,
        mob_destroyed_event: &mut EventReader<MobDestroyedEvent>,
        mob_reached_bottom_event: &mut EventReader<MobReachedBottomGateEvent>,
        mob_segment_destroyed_event: &mut EventReader<MobSegmentDestroyedEvent>,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
        mut player_spawn_params: ResMut<InputRestrictionsAtSpawn>,
    ) -> bool {
        self.level_time.tick(time.delta());

        if let Some(mut modified_current_phase) = self.current_phase.clone() {
            let phase_completed = match &mut modified_current_phase.phase_type {
                LevelPhaseType::FormationSpawn {
                    phase_timer,
                    spawn_timer,
                    formation_pool,
                    ..
                } => {
                    Self::tick_spawn_timer(
                        spawn_timer,
                        time,
                        spawn_formation_event_writer,
                        formations_res,
                        formation_pool.to_string(),
                    );

                    Self::tick_phase_timer(phase_timer, time)
                }
                LevelPhaseType::Break { phase_timer, .. } => {
                    Self::tick_phase_timer(phase_timer, time)
                }
                LevelPhaseType::Boss {
                    mob_type,
                    position,
                    spawn_timer,
                    ..
                } => {
                    if spawn_timer.finished() {
                        // check if no entities with a BossComponent tag exist

                        if bosses_destroyed_event_reader.is_empty() {
                            return false;
                        } else {
                            bosses_destroyed_event_reader.clear();
                            true
                        }
                    } else {
                        spawn_timer.tick(time.delta());
                        if spawn_timer.just_finished() {
                            spawn_mob_event_writer.send(SpawnMobEvent {
                                mob_type: mob_type.clone(),
                                position: *position,
                                rotation: Quat::default(),
                                boss: true,
                            });
                        }
                        false
                    }
                }
                LevelPhaseType::Tutorial {
                    tutorial_lesson, ..
                } => {
                    modify_player_spawn_params_for_lesson_phase(
                        &mut (*player_spawn_params),
                        tutorial_lesson,
                    );
                    let finished_tutorial_section = tutorial_lesson.update(
                        player_query,
                        mob_destroyed_event,
                        time,
                        spawn_mob_event_writer,
                        mob_reached_bottom_event,
                        mob_segment_destroyed_event,
                        play_sound_effect_event_writer,
                    );
                    if finished_tutorial_section {
                        *player_spawn_params = InputRestrictionsAtSpawn::default();
                    }
                    finished_tutorial_section
                }
            };

            self.current_phase = Some(modified_current_phase);

            // this will short circuit and not call cycle_phase if !phase_completed
            if phase_completed {
                info!("Phase completed");
                if !self.cycle_phase(cycle_phase_event_writer) {
                    self.init_phase(change_bg_music_event_writer);
                }
            }

            false
        } else {
            true
        }
    }

    fn tick_phase_timer(phase_timer: &mut Timer, time: &Time) -> bool {
        phase_timer.tick(time.delta());

        phase_timer.just_finished()
    }

    pub fn tick_spawn_timer(
        spawn_timer: &mut Timer,
        time: &Time,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        formation_key: String,
    ) {
        spawn_timer.tick(time.delta());

        if spawn_timer.just_finished() {
            if let Some(formation) = formations_res.get_random_formation(formation_key) {
                spawn_formation_event_writer.send(SpawnFormationEvent {
                    formation: formation.clone(),
                });
                spawn_timer.set_duration(Duration::from_secs_f32(formation.period));
                spawn_timer.reset();
                info!("Spawn timer duration reset to: {}", formation.period);
            }
        }
    }
}
