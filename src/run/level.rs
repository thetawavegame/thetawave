use bevy::{prelude::*, time::Stopwatch};
use serde::Deserialize;
use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};
use thetawave_interface::spawnable::MobType;

use crate::{
    assets::BGMusicType,
    spawnable::{BossesDestroyedEvent, SpawnMobEvent},
};

use super::{objective::Objective, FormationPoolsResource, SpawnFormationEvent};

#[derive(Resource, Deserialize)]
pub struct PremadeLevelsResource {
    pub levels_data: HashMap<String, LevelData>,
}

/// A defined section of the level
#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    /// phase type
    pub phase_type: LevelPhaseType,
    /// music to play during phase
    pub bg_music_transition: Option<BGMusicTransition>,
    #[serde(default)]
    pub phase_time: Stopwatch,
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
}

/// Data used to initialize levels
#[derive(Deserialize)]
pub struct LevelData {
    /// timeline of the phases of the level
    pub phases: Vec<LevelPhase>,
    /// objective of the level (besides surviving)
    pub objective: Objective,
}

/// Event to alert when level has been completed
#[derive(Event)]
pub struct LevelCompletedEvent;

/// Struct to manage a level
#[derive(Clone, Debug)]
pub struct Level {
    pub completed_phases: VecDeque<LevelPhase>,
    pub current_phase: Option<LevelPhase>,
    pub queued_phases: VecDeque<LevelPhase>,
    /// Level objective
    pub objective: Objective,

    pub level_time: Stopwatch,
}

impl From<&LevelData> for Level {
    fn from(data: &LevelData) -> Self {
        Level {
            completed_phases: vec![].into(),
            current_phase: None,
            queued_phases: data.phases.clone().into(),
            objective: data.objective.clone(),
            level_time: Stopwatch::new(),
        }
    }
}

impl Level {
    pub fn cycle_phase(&mut self) -> bool {
        // clone the current phase (if it exists) into the back of the completed phases queue
        if let Some(current_phase) = &self.current_phase {
            self.completed_phases.push_back(current_phase.clone());
            self.current_phase = None;
        }

        // pop the next level (if it exists) into the the current level
        self.current_phase = self.queued_phases.pop_front();

        info!("Phase cycled");

        // return true if no phase was available to cycle to the current phase
        self.current_phase.is_none()
    }

    pub fn init_phase(&mut self) {
        info!("Phase initialized");
    }

    // returns true if level has been completed
    pub fn tick(
        &mut self,
        time: &Time,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        bosses_destroyed_event_reader: &mut EventReader<BossesDestroyedEvent>,
    ) -> bool {
        self.level_time.tick(time.delta());

        if let Some(mut modified_current_phase) = self.current_phase.clone() {
            let phase_completed = match &mut modified_current_phase.phase_type {
                LevelPhaseType::FormationSpawn {
                    phase_timer,
                    spawn_timer,
                    formation_pool,
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
                LevelPhaseType::Break { phase_timer } => Self::tick_phase_timer(phase_timer, time),
                LevelPhaseType::Boss {
                    mob_type,
                    position,
                    spawn_timer,
                } => {
                    if spawn_timer.finished() {
                        // check if no entities with a BossComponent tag exist
                        !bosses_destroyed_event_reader.is_empty()
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
            };

            self.current_phase = Some(modified_current_phase);

            // this will short circuit and not call cycle_phase if !phase_completed
            if phase_completed && !self.cycle_phase() {
                self.init_phase();
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
        let formation = formations_res.get_random_formation(formation_key);
        spawn_timer.tick(time.delta());

        if spawn_timer.just_finished() {
            spawn_formation_event_writer.send(SpawnFormationEvent {
                formation: formation.clone(),
            });
            spawn_timer.set_duration(Duration::from_secs_f32(formation.period));
            spawn_timer.reset();
            info!("Spawn timer duration reset to: {}", formation.period);
        }
    }
}
