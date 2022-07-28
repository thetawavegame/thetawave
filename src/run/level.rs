use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

use crate::{
    arena::EnemyReachedBottomGateEvent, misc::Health, states::AppStates, tools::weighted_rng,
    ui::EndGameTransitionResource,
};

use super::formation;

pub type LevelsResourceData = HashMap<LevelType, LevelData>;

#[derive(Clone)]
pub struct LevelsResource {
    pub levels: HashMap<LevelType, Level>,
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

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LevelType {
    EarthOrbit,
}

#[derive(Deserialize)]
pub struct LevelData {
    pub timeline: LevelTimeline,
    pub initial_delay: f32,
    pub objective: ObjectiveType,
}

pub struct LevelCompletedEvent;

#[derive(Clone)]
pub struct Level {
    timeline_idx: Option<usize>,
    /// Timeline
    pub timeline: LevelTimeline,
    /// Tracks time of phases
    pub phase_timer: Option<Timer>,
    /// Tracks time between spawns
    pub spawn_timer: Option<Timer>,
    /// Level objective
    pub objective: ObjectiveType,
}

#[derive(Clone, Deserialize)]
pub enum ObjectiveType {
    Defense(Health),
}

impl From<LevelData> for Level {
    fn from(data: LevelData) -> Self {
        Level {
            timeline_idx: None,
            timeline: data.timeline,
            phase_timer: Some(Timer::from_seconds(data.initial_delay, true)),
            spawn_timer: None,
            objective: data.objective,
        }
    }
}

impl Level {
    pub fn get_phase_name(&self) -> String {
        if let Some(timeline_idx) = self.timeline_idx {
            return match self.timeline.phases[timeline_idx].phase_type {
                LevelPhaseType::FormationSpawn { .. } => "Formation".to_string(),
                LevelPhaseType::Break { .. } => "Break".to_string(),
            };
        }

        "None".to_string()
    }

    pub fn get_phase_number(&self) -> String {
        if let Some(timeline_idx) = self.timeline_idx {
            return timeline_idx.to_string();
        }

        "None".to_string()
    }

    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        level_completed: &mut EventWriter<LevelCompletedEvent>,
        enemy_reached_bottom: &mut EventReader<EnemyReachedBottomGateEvent>,
        formation_pools: &formation::FormationPoolsResource,
        end_game_trans_resource: &mut EndGameTransitionResource,
    ) {
        #[allow(clippy::single_match)]
        match &mut self.objective {
            ObjectiveType::Defense(health) => {
                for event in enemy_reached_bottom.iter() {
                    if event.0 > 0.0 {
                        health.take_damage(event.0);
                    } else {
                        health.heal(-event.0);
                    }
                    if health.is_dead() {
                        end_game_trans_resource.start(AppStates::GameOver);
                    }
                }
            }
        }

        if let Some(phase_timer) = &mut self.phase_timer {
            if phase_timer.tick(delta).just_finished() {
                if let Some(timeline_idx) = self.timeline_idx {
                    // sets level to next phase in timeline

                    if self.timeline.phases.len() > timeline_idx + 1 {
                        self.timeline_idx = Some(timeline_idx + 1);
                    } else {
                        level_completed.send(LevelCompletedEvent);
                    }
                } else {
                    // sets level to first phase in timeline
                    self.timeline_idx = Some(0);
                }
                self.setup_next_phase(level_completed);
            } else {
                //continue with current phase
                if let Some(current_phase) = self.get_current_phase() {
                    #[allow(clippy::single_match)]
                    match &current_phase.phase_type {
                        LevelPhaseType::FormationSpawn { formation_pool, .. } => {
                            // this is bad code. fix it
                            let event_formation_pool = formation_pool.clone();
                            // use spawn timer to spawn from formation pool when ready
                            if self
                                .spawn_timer
                                .as_mut()
                                .unwrap()
                                .tick(delta)
                                .just_finished()
                            {
                                // send spawn formation event
                                let weights = formation_pools[&event_formation_pool]
                                    .iter()
                                    .map(|x| x.weight)
                                    .collect();

                                let random_idx = weighted_rng(weights);

                                self.spawn_timer.as_mut().unwrap().set_duration(
                                    Duration::from_secs_f32(
                                        formation_pools[&event_formation_pool][random_idx].period,
                                    ),
                                );

                                spawn_formation.send(formation::SpawnFormationEvent {
                                    formation: formation_pools[&event_formation_pool][random_idx]
                                        .clone(),
                                });
                            }
                        }
                        _ => {}
                    }
                } else if self.timeline_idx.is_some() {
                    println!("Timeline index: {:?}", self.timeline_idx);
                    panic!("Something is wrong. There is not phase at the current timeline index.")
                }
            }
        }
    }

    fn setup_next_phase(&mut self, level_completed: &mut EventWriter<LevelCompletedEvent>) {
        let current_phase = self.get_current_phase();
        if let Some(current_phase) = current_phase {
            // setup next phase
            match current_phase.phase_type.clone() {
                LevelPhaseType::FormationSpawn {
                    time,
                    initial_delay,
                    formation_pool: _,
                } => {
                    self.spawn_timer = Some(Timer::from_seconds(initial_delay, true));
                    self.phase_timer = Some(Timer::from_seconds(time, false));
                }
                LevelPhaseType::Break { time } => {
                    self.spawn_timer = None;
                    self.phase_timer = Some(Timer::from_seconds(time, false));
                }
            }
        }
        /*
        else {
            // setup next level
            // send event to iterate run resources level index
            level_completed.send(LevelCompletedEvent);
        }
        */
    }

    fn get_current_phase(&self) -> Option<&LevelPhase> {
        self.timeline.phases.get(self.timeline_idx?)
    }
}

#[derive(Deserialize, Clone)]
pub struct LevelTimeline {
    pub phases: Vec<LevelPhase>,
}

#[derive(Deserialize, Clone)]
pub struct LevelPhase {
    pub phase_type: LevelPhaseType,
}

#[derive(Deserialize, Clone)]
pub enum LevelPhaseType {
    FormationSpawn {
        time: f32,
        initial_delay: f32,
        formation_pool: formation::FormationPoolType,
    },
    Break {
        time: f32,
    },
}

pub fn level_system(
    mut run_resource: ResMut<super::RunResource>,
    mut spawn_formation: EventWriter<formation::SpawnFormationEvent>,
    mut level_completed: EventWriter<LevelCompletedEvent>,
    mut enemy_reached_bottom: EventReader<EnemyReachedBottomGateEvent>,
    formation_pools: Res<formation::FormationPoolsResource>,
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
) {
    if run_resource.is_ready() && !end_game_trans_resource.start {
        run_resource.tick(
            time.delta(),
            &mut spawn_formation,
            &mut level_completed,
            &mut enemy_reached_bottom,
            &formation_pools,
            &mut end_game_trans_resource,
        );
    }
}

/// Progress to the next level when current level is completed
pub fn next_level_system(
    mut level_completed: EventReader<LevelCompletedEvent>,
    mut run_resource: ResMut<super::RunResource>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
) {
    for _level_completed in level_completed.iter() {
        end_game_trans_resource.start(AppStates::Victory);
    }
}
