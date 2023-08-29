use bevy::prelude::*;

use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use thetawave_interface::states::{AppStates, GameStates};

use crate::{
    arena::{DefenseInteraction, MobReachedBottomGateEvent},
    assets::{GameAudioAssets, SoundEffectType},
    audio::{self, ChangeBackgroundMusicEvent, PlaySoundEffectEvent},
    player::PlayersResource,
    spawnable::{BossesDestroyedEvent, MobDestroyedEvent, SpawnMobEvent},
    states::{self},
    GameEnterSet, GameUpdateSet,
};

mod formation;
mod level;
mod objective;

use self::level::Level;
pub use self::objective::Objective;
pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::{LevelCompletedEvent, PremadeLevelsResource},
};

pub struct RunPlugin;

impl Plugin for RunPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<FormationPoolsResource>(include_bytes!(
                "../../assets/data/formation_pools.ron"
            ))
            .unwrap(),
        )
        .insert_resource(
            from_bytes::<PremadeRunsResource>(include_bytes!("../../assets/data/premade_runs.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<PremadeLevelsResource>(include_bytes!(
                "../../assets/data/premade_levels.ron"
            ))
            .unwrap(),
        )
        .insert_resource(RunResource::default());

        app.add_event::<SpawnFormationEvent>()
            .add_event::<LevelCompletedEvent>()
            .add_event::<RunEndEvent>();

        app.add_systems(OnEnter(AppStates::InitializeRun), init_run_system);

        app.add_systems(
            Update,
            (tick_run_system, handle_objective_system, run_end_system)
                .in_set(GameUpdateSet::Level)
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );

        app.add_systems(
            Update,
            spawn_formation_system
                .in_set(GameUpdateSet::Spawn)
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );

        // reset the run after exiting the end game screens and when entering the main menu
        app.add_systems(OnExit(AppStates::GameOver), run_reset_system);
        app.add_systems(OnExit(AppStates::Victory), run_reset_system);
        app.add_systems(OnEnter(AppStates::MainMenu), run_reset_system);
    }
}

#[derive(Resource, Deserialize)]
pub struct PremadeRunsResource {
    pub runs: HashMap<String, Vec<String>>,
}

pub enum RunOutcomeType {
    #[allow(dead_code)] // TODO: Why is this never constructed????
    Victory,
    Defeat(RunDefeatType),
}

pub enum RunDefeatType {
    #[allow(dead_code)] // TODO: Why is this never constructed????
    PlayersDestroyed,
    DefenseDestroyed,
}

#[derive(Event)]
pub struct RunEndEvent {
    pub outcome: RunOutcomeType,
}

#[derive(Resource, Default, Debug)]
pub struct RunResource {
    /// List of string level keys that are matched to values in the levelsresource
    pub queued_levels: VecDeque<Level>,
    pub completed_levels: VecDeque<Level>,
    /// Tracks the level currently being played
    pub current_level: Option<Level>,
}

impl RunResource {
    pub fn generate_random(&mut self) {}

    /// Generate a premade level using a String run key
    pub fn generate_premade(
        &mut self,
        run_key: String,
        premade_runs_res: &PremadeRunsResource,
        premade_levels_res: &PremadeLevelsResource,
    ) {
        // get the level keys from the premade runs resource
        let level_keys = premade_runs_res.runs.get(&run_key).unwrap();

        // get levels from the levels resource
        let levels: VecDeque<Level> = level_keys
            .iter()
            .map(|key| Level::from(premade_levels_res.levels_data.get(key).unwrap()))
            .collect();

        // set levels in the run resource
        self.queued_levels = levels;

        info!("Generated premade level");
    }

    pub fn cycle_level(&mut self, run_end_event_writer: &mut EventWriter<RunEndEvent>) {
        // clone the current level (if it exists) into the back of the completed levels queue
        if let Some(current_level) = &self.current_level {
            self.completed_levels.push_back(current_level.clone());
            self.current_level = None;
        }

        // pop the next level (if it exists) into the the current level
        self.current_level = self.queued_levels.pop_front();

        // if the current level is None, then the player has completed all the levels and has won the game
        if self.current_level.is_none() {
            run_end_event_writer.send(RunEndEvent {
                outcome: RunOutcomeType::Victory,
            });
        }

        info!("Level cycled");
    }

    pub fn init_current_level(
        &mut self,
        run_end_event_writer: &mut EventWriter<RunEndEvent>,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
    ) {
        if let Some(current_level) = &mut self.current_level {
            let level_completed = current_level.cycle_phase();

            if !level_completed {
                current_level.init_phase(change_bg_music_event_writer);
            } else {
                self.cycle_level(run_end_event_writer);
            }
        }
    }

    pub fn tick(
        &mut self,
        time: &Time,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        bosses_destroyed_event_reader: &mut EventReader<BossesDestroyedEvent>,
        run_end_event_writer: &mut EventWriter<RunEndEvent>,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
    ) {
        // TODO: handle none case to remove unwrap
        let current_level = self.current_level.as_mut().unwrap();

        // cycle level when done with all phases
        if current_level.tick(
            time,
            spawn_formation_event_writer,
            formations_res,
            spawn_mob_event_writer,
            bosses_destroyed_event_reader,
            change_bg_music_event_writer,
        ) {
            self.cycle_level(run_end_event_writer);
            self.init_current_level(run_end_event_writer, change_bg_music_event_writer);
        }
    }
}

fn init_run_system(
    mut run_res: ResMut<RunResource>,
    premade_runs_res: Res<PremadeRunsResource>,
    premade_levels_res: Res<PremadeLevelsResource>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut run_end_event_writer: EventWriter<RunEndEvent>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    // generate the run
    run_res.generate_premade(
        "test_run".to_string(),
        &premade_runs_res,
        &premade_levels_res,
    );

    // cycle to set the current level to the first level
    run_res.cycle_level(&mut run_end_event_writer);

    // initialize the current level
    run_res.init_current_level(&mut run_end_event_writer, &mut change_bg_music_event_writer);

    next_app_state.set(AppStates::Game);

    info!("Run initialized");
}

fn tick_run_system(
    mut run_res: ResMut<RunResource>,
    time: Res<Time>,
    mut spawn_formation_event_writer: EventWriter<SpawnFormationEvent>,
    formations_res: Res<FormationPoolsResource>,
    mut spawn_mob_event_writer: EventWriter<SpawnMobEvent>,
    mut bosses_destroyed_event_reader: EventReader<BossesDestroyedEvent>,
    mut run_end_event_writer: EventWriter<RunEndEvent>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
) {
    run_res.tick(
        &time,
        &mut spawn_formation_event_writer,
        &formations_res,
        &mut spawn_mob_event_writer,
        &mut bosses_destroyed_event_reader,
        &mut run_end_event_writer,
        &mut change_bg_music_event_writer,
    );
}

fn handle_objective_system(
    mut run_res: ResMut<RunResource>,
    mut bottom_gate_event: EventReader<MobReachedBottomGateEvent>,
    mut run_end_event: EventWriter<RunEndEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
) {
    if let Some(current_level) = &mut run_res.current_level {
        let objective = &mut current_level.objective;

        match objective {
            Objective::Defense(defense_data) => {
                for event in bottom_gate_event.iter() {
                    match event.0 {
                        DefenseInteraction::Heal(value) => {
                            // heal defense objective
                            defense_data.gain_defense(value);

                            // play heal sound effect
                            sound_effect_event_writer.send(PlaySoundEffectEvent {
                                sound_effect_type: SoundEffectType::DefenseHeal,
                            });
                        }
                        DefenseInteraction::Damage(value) => {
                            // damage defense objective
                            defense_data.take_damage(value);

                            //play damage sound effect
                            sound_effect_event_writer.send(PlaySoundEffectEvent {
                                sound_effect_type: SoundEffectType::DefenseDamage,
                            });
                        }
                    };
                }

                if defense_data.is_failed() {
                    run_end_event.send(RunEndEvent {
                        outcome: RunOutcomeType::Defeat(RunDefeatType::DefenseDestroyed),
                    });
                }
            }
        }
    }
}

fn run_end_system(
    mut run_end_event_reader: EventReader<RunEndEvent>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    for event in run_end_event_reader.iter() {
        match &event.outcome {
            RunOutcomeType::Victory => {
                next_app_state.set(AppStates::Victory);
            }
            RunOutcomeType::Defeat(defeat_type) => {
                next_app_state.set(AppStates::GameOver);

                match defeat_type {
                    RunDefeatType::PlayersDestroyed => info!("Players destroyed"),
                    RunDefeatType::DefenseDestroyed => info!("Defense objective failed"),
                };
            }
        }
    }
}

fn run_reset_system(mut run_resource: ResMut<RunResource>) {
    *run_resource = RunResource::default();
}
