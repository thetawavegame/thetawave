use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

use crate::{
    arena::EnemyReachedBottomGateEvent,
    misc::Health,
    spawnable::{self, BossType},
    states::AppStates,
    tools::weighted_rng,
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
    TestLevel,
}

#[derive(Deserialize)]
pub struct LevelData {
    pub timeline: LevelTimeline,
    pub objective: ObjectiveType,
}

pub struct LevelCompletedEvent;

#[derive(Clone, Debug)]
pub struct Level {
    timeline_idx: usize,
    /// Timeline
    pub timeline: LevelTimeline,
    /// Tracks time of phases
    pub phase_timer: Option<Timer>,
    /// Tracks time between spawns
    pub spawn_timer: Option<Timer>,
    /// Level objective
    pub objective: ObjectiveType,
}

#[derive(Clone, Deserialize, Debug)]
pub enum ObjectiveType {
    Defense(Health),
}

impl From<LevelData> for Level {
    fn from(data: LevelData) -> Self {
        Level {
            timeline_idx: 0,
            timeline: data.timeline,
            phase_timer: None,
            spawn_timer: None,
            objective: data.objective,
        }
    }
}

impl Level {
    pub fn get_phase_name(&self) -> String {
        return match &self.timeline.phases[self.timeline_idx].phase_type {
            LevelPhaseType::FormationSpawn { formation_pool, .. } => {
                format!("FormationSpawn({:?})", formation_pool)
            }
            LevelPhaseType::Break { .. } => "Break".to_string(),
            LevelPhaseType::Boss { boss_type, .. } => format!("Boss[{:?}]", boss_type),
        };

        "None".to_string()
    }

    pub fn get_phase_number(&self) -> String {
        return self.timeline_idx.to_string();

        "None".to_string()
    }

    pub fn tick(
        &mut self,
        delta: Duration,
        spawn_formation: &mut EventWriter<formation::SpawnFormationEvent>,
        spawn_boss: &mut EventWriter<spawnable::SpawnBossEvent>,
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

        if let Some(current_phase) = self.get_current_phase() {
            // do stuff specific to phase type
            match &current_phase.phase_type {
                LevelPhaseType::FormationSpawn {
                    time,
                    initial_delay,
                    formation_pool,
                } => {
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

                        self.spawn_timer
                            .as_mut()
                            .unwrap()
                            .set_duration(Duration::from_secs_f32(
                                formation_pools[&event_formation_pool][random_idx].period,
                            ));

                        spawn_formation.send(formation::SpawnFormationEvent {
                            formation: formation_pools[&event_formation_pool][random_idx].clone(),
                        });
                    }
                }
                LevelPhaseType::Boss {
                    boss_type,
                    initial_delay,
                    is_defeated,
                } => {
                    let boss_type = boss_type.clone();

                    if self
                        .spawn_timer
                        .as_mut()
                        .unwrap()
                        .tick(delta)
                        .just_finished()
                    {
                        info!("spawn boss");
                        spawn_boss.send(spawnable::SpawnBossEvent {
                            boss_type,
                            position: Vec2::new(0.0, 100.0),
                        })
                    }
                }
                _ => {}
            }

            // if phase timer for phase, tick the timer
            if let Some(phase_timer) = &mut self.phase_timer {
                // check of the phase just ended
                if phase_timer.tick(delta).just_finished() {
                    // set level to nect phase in timeline
                    if self.timeline.phases.len() > self.timeline_idx + 1 {
                        self.timeline_idx = self.timeline_idx + 1;
                    } else {
                        // level completed
                        level_completed.send(LevelCompletedEvent);
                    }
                    self.setup_next_phase();
                } else {
                    // continue with the current phase
                }
            }
        } else {
            println!("Timeline index: {:?}", self.timeline_idx);
            panic!("Something is wrong. There is not phase at the current timeline index.")
        }
    }

    fn setup_next_phase(&mut self) {
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
                LevelPhaseType::Boss {
                    boss_type: _,
                    initial_delay,
                    is_defeated: _,
                } => {
                    self.spawn_timer = Some(Timer::from_seconds(initial_delay, false));
                    self.phase_timer = None;
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
        self.timeline.phases.get(self.timeline_idx)
    }
}

#[derive(Deserialize, Clone, Debug)]
pub struct LevelTimeline {
    pub phases: Vec<LevelPhase>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LevelPhase {
    pub phase_type: LevelPhaseType,
}

#[derive(Deserialize, Clone, Debug)]
pub enum LevelPhaseType {
    FormationSpawn {
        time: f32,
        initial_delay: f32,
        formation_pool: formation::FormationPoolType,
    },
    Break {
        time: f32,
    },
    Boss {
        boss_type: BossType,
        initial_delay: f32,
        is_defeated: bool,
    },
}

pub fn level_system(
    mut run_resource: ResMut<super::RunResource>,
    mut spawn_formation: EventWriter<formation::SpawnFormationEvent>,
    mut spawn_boss: EventWriter<spawnable::SpawnBossEvent>,
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
            &mut spawn_boss,
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
    // TODO: add case for going to next level, instead of instantly winning after one level
    for _level_completed in level_completed.iter() {
        end_game_trans_resource.start(AppStates::Victory);
    }
}

// setup first level of the game using values from the first level (phase timer spawn timer, etc)
pub fn setup_first_level(mut run_resource: ResMut<super::RunResource>) {
    if let Some(level) = &mut run_resource.level {
        match &level.timeline.phases[0].phase_type {
            LevelPhaseType::FormationSpawn {
                time,
                initial_delay,
                formation_pool,
            } => {
                level.spawn_timer = Some(Timer::from_seconds(*initial_delay, true));
                level.phase_timer = Some(Timer::from_seconds(*time, false));
            }
            LevelPhaseType::Break { time } => {
                level.spawn_timer = None;
                level.phase_timer = Some(Timer::from_seconds(*time, false));
            }
            LevelPhaseType::Boss {
                boss_type,
                initial_delay,
                is_defeated,
            } => {
                level.spawn_timer = Some(Timer::from_seconds(*initial_delay, false));
                level.phase_timer = None;
            }
        }
    }
}
