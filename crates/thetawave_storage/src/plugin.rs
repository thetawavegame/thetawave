/// Exposes a single Plugin that links the game and our persistence layer.
use bevy::prelude::*;
use thetawave_interface::game::options::{GameOptions, DEFAULT_OPTIONS_PROFILE_ID};

use crate::options::get_game_options;
use crate::user_stats::{
    get_mob_killed_counts_for_user, get_user_stats, set_user_stats_for_user_id,
};
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, MobsKilledByPlayerCacheT, UserStatsByPlayerCacheT,
    UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
};
use thetawave_interface::states;

use super::core::{get_db, setup_db};
use super::user_stats::set_mob_killed_count_for_user;

/// Persist some user-specific stats and game state to a local SQLite database.
pub struct DBPlugin;

fn flush_user_stats_for_completed_games_to_db(
    shot_counters_for_current_game: Res<UserStatsByPlayerForCompletedGamesCache>,
) {
    if let Some(user_stats) = (**shot_counters_for_current_game).get(&DEFAULT_USER_ID) {
        set_user_stats_for_user_id(DEFAULT_USER_ID, user_stats).unwrap_or_else(|e| {
            error!(
                "Failed to flush per-run/game metrics to the database. Skipping. {}",
                e
            )
        });
    }
}
fn flush_mobs_killed_for_completed_games_counters_to_db(
    mobs_killed_for_current_game: Res<MobKillsByPlayerForCompletedGames>,
) {
    info!(
        "Flushing mob kills to db {:?}",
        **mobs_killed_for_current_game
    );
    if let Some(mob_kills) = (**mobs_killed_for_current_game).get(&DEFAULT_USER_ID) {
        for (mob_type, n_killed) in mob_kills {
            set_mob_killed_count_for_user(DEFAULT_USER_ID, &mob_type, n_killed.clone())
                .unwrap_or_else(|e| error!("Error incrementing mob kill count: {e}"));
        }
    }
}
impl Plugin for DBPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(states::AppStates::LoadingAssets), db_setup_system);
        app.add_systems(
            OnExit(states::AppStates::LoadingAssets),
            (
                load_user_stats_cache_from_db,
                load_mob_kills_cache_from_db,
                load_game_options_from_db,
            ),
        );
        app.add_systems(
            OnExit(states::AppStates::GameOver),
            (
                flush_user_stats_for_completed_games_to_db,
                flush_mobs_killed_for_completed_games_counters_to_db,
            ),
        );
    }
}

fn load_game_options_from_db(mut game_options: ResMut<GameOptions>) {
    if let Some(db_game_options) = get_game_options(DEFAULT_OPTIONS_PROFILE_ID) {
        *game_options = db_game_options;
    }
}

fn load_user_stats_cache_from_db(
    mut user_stats_cache: ResMut<UserStatsByPlayerForCompletedGamesCache>,
) {
    if !user_stats_cache.is_empty() {
        warn!(
            "evicting data from the in-memory cache. Is this right? {:?}",
            user_stats_cache
        );
    }
    **user_stats_cache =
        UserStatsByPlayerCacheT::from([(0, get_user_stats(DEFAULT_USER_ID).unwrap_or_default())]);
}
fn load_mob_kills_cache_from_db(mut mob_kills_cache: ResMut<MobKillsByPlayerForCompletedGames>) {
    if !(**mob_kills_cache).is_empty() {
        warn!(
            "evicting data from the in-memory mob kills cache. Is this right? {:?}",
            mob_kills_cache
        );
    }
    (**mob_kills_cache) =
        MobsKilledByPlayerCacheT::from([(0, get_mob_killed_counts_for_user(DEFAULT_USER_ID))]);
}
fn db_setup_system() {
    match get_db() {
        Ok(conn) => setup_db(conn).unwrap_or_else(|e| {
            error!("{e}");
        }),
        Err(e) => {
            error!("{e}");
        }
    };
}

#[cfg(test)]
mod test {
    use crate::core::THETAWAVE_DB_PATH_ENVVAR;
    use crate::plugin::DBPlugin;
    use crate::user_stats::{get_mob_killed_counts_for_user, get_user_stats};
    use bevy::log::{Level, LogPlugin};
    use bevy::prelude::{default, App, NextState, OnEnter, ResMut};
    use bevy::MinimalPlugins;
    use std::ffi::{OsStr, OsString};
    use tempdir;
    use thetawave_interface::game::historical_metrics::{
        MobKillsByPlayerForCompletedGames, MobsKilledBy1PlayerCacheT, MobsKilledByPlayerCacheT,
        UserStat, UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
    };
    use thetawave_interface::game::options::GameOptions;
    use thetawave_interface::spawnable::EnemyMobType;
    use thetawave_interface::states::AppStates;

    fn run_with_patched_env<T, V>(test: T, env_vars: Vec<(V, V)>)
    where
        T: FnOnce() -> () + std::panic::UnwindSafe,
        V: AsRef<OsStr>,
    {
        let old_env_vars: Vec<(OsString, OsString)> = env_vars
            .iter()
            .map(|(k, _)| (OsString::from(k), std::env::var_os(k).unwrap_or_default()))
            .collect();
        for (k, v) in env_vars.iter() {
            std::env::set_var(k, v);
        }

        let result = std::panic::catch_unwind(test);

        for (k, v) in old_env_vars.iter() {
            std::env::set_var(k, v);
        }
        if let Err(err) = result {
            std::panic::resume_unwind(err);
        }
    }

    #[test]
    fn test_recover_resources_from_db_after_mock_program_restart() {
        // Use temp paths for an ephemeral db an isolated, reproducible tests
        let base_path = tempdir::TempDir::new("thetawave-tests").unwrap();
        let temp_file_path = base_path.path().join("thetawave_test.sqlite");

        run_with_patched_env(
            _test_can_flush_caches_to_db,
            vec![(
                OsString::from(&THETAWAVE_DB_PATH_ENVVAR),
                OsString::from(temp_file_path),
            )],
        )
    }

    fn set_loading_assets(mut s: ResMut<NextState<AppStates>>) {
        (*s).set(AppStates::LoadingAssets);
    }
    fn set_game_state(mut s: ResMut<NextState<AppStates>>) {
        (*s).set(AppStates::Game);
    }
    fn set_game_over_state(mut s: ResMut<NextState<AppStates>>) {
        (*s).set(AppStates::GameOver);
    }

    fn set_dummy_terminal_game_state(mut s: ResMut<NextState<AppStates>>) {
        (*s).set(AppStates::CharacterSelection);
    }

    fn clear_completed_games_metrics(
        mut historical_games_shot_counts: ResMut<UserStatsByPlayerForCompletedGamesCache>,
        mut historical_games_enemy_mob_kill_counts: ResMut<MobKillsByPlayerForCompletedGames>,
    ) {
        (**historical_games_shot_counts).clear();
        (**historical_games_enemy_mob_kill_counts).clear();
    }

    fn set_n_drones_killed_for_p1_in_completed_games_cache<const N_DRONES: usize>(
        mut historical_games_enemy_mob_kill_counts: ResMut<MobKillsByPlayerForCompletedGames>,
    ) {
        (**historical_games_enemy_mob_kill_counts).insert(
            DEFAULT_USER_ID,
            MobsKilledBy1PlayerCacheT::from([(EnemyMobType::Drone, N_DRONES)]),
        );
    }
    fn set_user_stats_for_completed_games<
        const N_GAMES_LOST: usize,
        const TOTAL_SHOTS_HIT: usize,
        const TOTAL_SHOTS_FIRED: usize,
    >(
        mut historical_user_stats: ResMut<UserStatsByPlayerForCompletedGamesCache>,
    ) {
        (**historical_user_stats).insert(
            DEFAULT_USER_ID,
            UserStat {
                total_shots_fired: TOTAL_SHOTS_FIRED,
                total_shots_hit: TOTAL_SHOTS_HIT,
                total_games_lost: N_GAMES_LOST,
            },
        );
    }
    fn _minimal_app_for_db_plugin_tests() -> App {
        let mut app = App::new();
        app.add_plugins(DBPlugin)
            .init_state::<AppStates>()
            .add_plugins(MinimalPlugins)
            .add_plugins(LogPlugin {
                filter: "".to_string(),
                level: Level::DEBUG,
                ..default()
            })
            .insert_resource(MobKillsByPlayerForCompletedGames::default())
            .insert_resource(UserStatsByPlayerForCompletedGamesCache::default())
            .insert_resource(GameOptions::default());
        app
    }
    fn _test_can_flush_caches_to_db() {
        const N_DRONES_KILLED: usize = 15;
        const N_GAMES_PLAYED: usize = 2;
        const TOTAL_SHOTS_HIT: usize = 10;
        const TOTAL_SHOTS_FIRED: usize = 15;

        let mob_kills_after_1_game =
            MobKillsByPlayerForCompletedGames::from(MobsKilledByPlayerCacheT::from([(
                0,
                MobsKilledBy1PlayerCacheT::from([(EnemyMobType::Drone, N_DRONES_KILLED)]),
            )]));
        let mut app = _minimal_app_for_db_plugin_tests();
        app.add_systems(OnEnter(AppStates::LoadingAssets), set_game_state)
            .add_systems(
                OnEnter(AppStates::Game),
                (
                    set_n_drones_killed_for_p1_in_completed_games_cache::<N_DRONES_KILLED>,
                    set_user_stats_for_completed_games::<
                        N_GAMES_PLAYED,
                        TOTAL_SHOTS_HIT,
                        TOTAL_SHOTS_FIRED,
                    >,
                ),
            )
            .add_systems(OnEnter(AppStates::Game), set_game_over_state)
            .add_systems(OnEnter(AppStates::GameOver), set_dummy_terminal_game_state);

        app.update();
        app.update();
        app.update();

        assert_eq!(
            get_user_stats(DEFAULT_USER_ID).unwrap(),
            UserStat {
                total_shots_fired: TOTAL_SHOTS_FIRED,
                total_shots_hit: TOTAL_SHOTS_HIT,
                total_games_lost: N_GAMES_PLAYED,
            }
        );
        assert_eq!(
            &get_mob_killed_counts_for_user(DEFAULT_USER_ID),
            &mob_kills_after_1_game
                .get(&DEFAULT_USER_ID)
                .unwrap()
                .clone()
        );
        assert_eq!(
            &get_mob_killed_counts_for_user(DEFAULT_USER_ID),
            app.world
                .get_resource::<MobKillsByPlayerForCompletedGames>()
                .unwrap()
                .get(&DEFAULT_USER_ID)
                .unwrap()
        );
    }
}
