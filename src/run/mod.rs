use bevy::prelude::*;

use leafwing_input_manager::prelude::ActionState;
use ron::de::from_bytes;
use serde::Deserialize;
use std::collections::{HashMap, VecDeque};
use thetawave_interface::input::PlayerAction;
use thetawave_interface::player::InputRestrictionsAtSpawn;
use thetawave_interface::{
    audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent, SoundEffectType},
    objective::{DefenseInteraction, MobReachedBottomGateEvent, Objective},
    player::{PlayerComponent, PlayersResource},
    run::{CyclePhaseEvent, RunDefeatType, RunEndEvent, RunOutcomeType},
    spawnable::{MobDestroyedEvent, MobSegmentDestroyedEvent, SpawnMobEvent},
    states::{AppStates, GameStates},
};

use crate::{spawnable::BossesDestroyedEvent, GameUpdateSet};

mod formation;
mod level;
pub(crate) mod level_phase;
pub(crate) mod tutorial;

use self::level::Level;
pub use self::{
    formation::{spawn_formation_system, FormationPoolsResource, SpawnFormationEvent},
    level::PremadeLevelsResource,
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
        .insert_resource(CurrentRunProgressResource::default());

        app.add_event::<SpawnFormationEvent>()
            .add_event::<RunEndEvent>()
            .add_event::<CyclePhaseEvent>();

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

#[derive(Resource, Debug)]
pub struct CurrentRunProgressResource {
    /// List of string level keys that are matched to values in the levelsresource
    pub queued_levels: VecDeque<Level>,
    pub completed_levels: VecDeque<Level>,
    /// Tracks the level currently being played
    pub current_level: Option<Level>,
    /// If true will append tutorial level to beginning of the run
    pub tutorials_on: bool,
}

impl Default for CurrentRunProgressResource {
    fn default() -> Self {
        CurrentRunProgressResource {
            queued_levels: VecDeque::new(),
            completed_levels: VecDeque::new(),
            current_level: None,
            tutorials_on: true,
        }
    }
}

impl CurrentRunProgressResource {
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
        let mut levels: VecDeque<Level> = level_keys
            .iter()
            .map(|key| Level::from(premade_levels_res.levels_data.get(key).unwrap()))
            .collect();

        // push a tutorial level to be the first level played
        if self.tutorials_on {
            levels.push_front(Level::from(
                premade_levels_res.levels_data.get("tutorial").unwrap(),
            ));
        }

        // set levels in the run resource
        self.queued_levels = levels;

        info!("Generated premade level");
    }

    pub fn cycle_level(&mut self) {
        // clone the current level (if it exists) into the back of the completed levels queue
        if let Some(current_level) = &self.current_level {
            self.completed_levels.push_back(current_level.clone());
            self.current_level = None;
        }

        // pop the next level (if it exists) into the the current level
        self.current_level = self.queued_levels.pop_front();

        info!("Level cycled");
    }

    pub fn init_current_level(
        &mut self,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
        cycle_phase_event_writer: &mut EventWriter<CyclePhaseEvent>,
    ) {
        if let Some(current_level) = &mut self.current_level {
            let level_completed = current_level.cycle_phase(cycle_phase_event_writer);

            if !level_completed {
                current_level.init_phase(change_bg_music_event_writer);
            } else {
                self.cycle_level();
            }
        }
    }

    pub fn tick(
        &mut self,
        time: &Time,
        player_query: &Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
        spawn_formation_event_writer: &mut EventWriter<SpawnFormationEvent>,
        formations_res: &FormationPoolsResource,
        spawn_mob_event_writer: &mut EventWriter<SpawnMobEvent>,
        bosses_destroyed_event_reader: &mut EventReader<BossesDestroyedEvent>,
        run_end_event_writer: &mut EventWriter<RunEndEvent>,
        change_bg_music_event_writer: &mut EventWriter<ChangeBackgroundMusicEvent>,
        cycle_phase_event_writer: &mut EventWriter<CyclePhaseEvent>,
        mob_destroyed_event: &mut EventReader<MobDestroyedEvent>,
        mob_reached_bottom_event: &mut EventReader<MobReachedBottomGateEvent>,
        mob_segment_destroyed_event: &mut EventReader<MobSegmentDestroyedEvent>,
        play_sound_effect_event_writer: &mut EventWriter<PlaySoundEffectEvent>,
        player_component_query: &mut Query<&mut PlayerComponent>,
        player_spawn_params: ResMut<InputRestrictionsAtSpawn>,
    ) {
        if let Some(current_level) = &mut self.current_level {
            // cycle level when done with all phases
            if current_level.tick(
                time,
                player_query,
                spawn_formation_event_writer,
                formations_res,
                spawn_mob_event_writer,
                bosses_destroyed_event_reader,
                change_bg_music_event_writer,
                cycle_phase_event_writer,
                mob_destroyed_event,
                mob_reached_bottom_event,
                mob_segment_destroyed_event,
                play_sound_effect_event_writer,
                player_component_query,
                player_spawn_params,
            ) {
                self.cycle_level();
                self.init_current_level(change_bg_music_event_writer, cycle_phase_event_writer);
            }
        } else {
            run_end_event_writer.send(RunEndEvent {
                outcome: RunOutcomeType::Victory,
            });
        }
    }
}

fn init_run_system(
    mut run_res: ResMut<CurrentRunProgressResource>,
    players: Res<PlayersResource>,
    premade_runs_res: Res<PremadeRunsResource>,
    premade_levels_res: Res<PremadeLevelsResource>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
    mut cycle_phase_event_writer: EventWriter<CyclePhaseEvent>,
) {
    // Enable tutorials if and only if:
    // - It was specifically toggled on when the user was setting up the run
    // - We have exactly 1 player. TODO: Maybe enhance the tutorial to also work for many players.
    run_res.tutorials_on = run_res.tutorials_on
        && ((players.player_data.len() <= 1) || players.player_data[1].is_none());
    info!("Tutorials are on: {}", run_res.tutorials_on);
    // generate the run
    run_res.generate_premade(
        "test_run".to_string(),
        &premade_runs_res,
        &premade_levels_res,
    );

    // cycle to set the current level to the first level
    run_res.cycle_level();

    // initialize the current level
    run_res.init_current_level(
        &mut change_bg_music_event_writer,
        &mut cycle_phase_event_writer,
    );

    next_app_state.set(AppStates::Game);

    info!("Run initialized");
}

fn tick_run_system(
    mut run_res: ResMut<CurrentRunProgressResource>,
    time: Res<Time>,
    player_query: Query<&ActionState<PlayerAction>, With<PlayerComponent>>,
    mut spawn_formation_event_writer: EventWriter<SpawnFormationEvent>,
    formations_res: Res<FormationPoolsResource>,
    mut spawn_mob_event_writer: EventWriter<SpawnMobEvent>,
    mut bosses_destroyed_event_reader: EventReader<BossesDestroyedEvent>,
    mut run_end_event_writer: EventWriter<RunEndEvent>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
    mut cycle_phase_event_writer: EventWriter<CyclePhaseEvent>,
    mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>,
    mut mob_reached_bottom_event_reader: EventReader<MobReachedBottomGateEvent>,
    mut mob_segment_destroyed_event_reader: EventReader<MobSegmentDestroyedEvent>,
    mut play_sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    mut player_component_query: Query<&mut PlayerComponent>,
    player_spawn_params: ResMut<InputRestrictionsAtSpawn>,
) {
    run_res.tick(
        &time,
        &player_query,
        &mut spawn_formation_event_writer,
        &formations_res,
        &mut spawn_mob_event_writer,
        &mut bosses_destroyed_event_reader,
        &mut run_end_event_writer,
        &mut change_bg_music_event_writer,
        &mut cycle_phase_event_writer,
        &mut mob_destroyed_event_reader,
        &mut mob_reached_bottom_event_reader,
        &mut mob_segment_destroyed_event_reader,
        &mut play_sound_effect_event_writer,
        &mut player_component_query,
        player_spawn_params,
    );
}

fn handle_objective_system(
    mut run_res: ResMut<CurrentRunProgressResource>,
    mut bottom_gate_event: EventReader<MobReachedBottomGateEvent>,
    mut run_end_event: EventWriter<RunEndEvent>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
) {
    if let Some(current_level) = &mut run_res.current_level {
        if let Some(objective) = &mut current_level.objective {
            match objective {
                Objective::Defense(defense_data) => {
                    for event in bottom_gate_event.iter() {
                        match event.defense_interaction {
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

/// clear/reset various globals to the defaults to prepare for playing another run/game
fn run_reset_system(
    mut run_resource: ResMut<CurrentRunProgressResource>,
    mut spawn_restrictions: ResMut<InputRestrictionsAtSpawn>,
) {
    *run_resource = CurrentRunProgressResource::default();
    *spawn_restrictions = InputRestrictionsAtSpawn::default();
}

#[cfg(test)]
mod test {
    use crate::run::{RunPlugin, SpawnFormationEvent};
    use crate::spawnable::{BossesDestroyedEvent, SpawnConsumableEvent};
    use bevy::app::App;
    use bevy::log::{Level, LogPlugin};
    use bevy::prelude::{NextState, State};
    use bevy::MinimalPlugins;
    use rstest::rstest;
    use thetawave_interface::audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent};
    use thetawave_interface::objective::{DefenseInteraction, MobReachedBottomGateEvent};
    use thetawave_interface::player::{InputRestrictionsAtSpawn, PlayersResource};
    use thetawave_interface::spawnable::{
        MobDestroyedEvent, MobSegmentDestroyedEvent, SpawnMobEvent,
    };
    use thetawave_interface::states::{AppStates, GameStates};

    use super::CurrentRunProgressResource;

    fn _minimal_app_for_run_progression_defend_gate_objective() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin {
                filter: "".to_string(),
                level: Level::TRACE,
            })
            .add_state::<AppStates>()
            .add_state::<GameStates>()
            .add_event::<MobReachedBottomGateEvent>()
            .add_event::<ChangeBackgroundMusicEvent>()
            .add_event::<PlaySoundEffectEvent>()
            .add_event::<SpawnConsumableEvent>()
            .add_event::<BossesDestroyedEvent>()
            .add_event::<SpawnFormationEvent>()
            .add_event::<SpawnMobEvent>()
            .add_event::<MobDestroyedEvent>()
            .add_event::<MobSegmentDestroyedEvent>()
            .insert_resource(PlayersResource::default())
            .insert_resource(InputRestrictionsAtSpawn::default())
            .add_plugins(RunPlugin);
        app.world
            .get_resource_mut::<CurrentRunProgressResource>()
            .unwrap()
            .tutorials_on = false; // We have the gate defense objective after tutorials
        app
    }

    #[rstest]
    #[case::large_gate_damage_triggers_game_over(101, AppStates::GameOver)]
    #[case::small_gate_damage_keeps_game_going(10, AppStates::Game)]
    fn test_gate_health_transitions_app_state(
        #[case] damage_amount: usize,
        #[case] want_end_state: AppStates,
    ) {
        // Defense starts with 100 HP
        let mut app = _minimal_app_for_run_progression_defend_gate_objective();
        // triggers => AppStates::Game and starts listening to events
        app.world
            .get_resource_mut::<NextState<AppStates>>()
            .unwrap()
            .set(AppStates::InitializeRun);
        app.world
            .get_resource_mut::<NextState<GameStates>>()
            .unwrap()
            .set(GameStates::Playing);
        app.update();
        app.update();
        // A system in this plugin _should_ kick off the game/run
        assert_eq!(
            &AppStates::Game,
            app.world.get_resource::<State<AppStates>>().unwrap().get()
        );
        // This is the main part of the test
        app.world.send_event(MobReachedBottomGateEvent {
            defense_interaction: DefenseInteraction::Damage(damage_amount),
            mob_type: None,
            mob_segment_type: None,
        });
        app.update();
        app.update();
        app.update();
        assert_eq!(
            &want_end_state,
            app.world.get_resource::<State<AppStates>>().unwrap().get()
        );
    }
}
