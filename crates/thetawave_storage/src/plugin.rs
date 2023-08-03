/// Exposes a single Plugin that links the game and our persistence layer.
use bevy::prelude::*;

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
    if let Some(user_stats) = (*shot_counters_for_current_game).0.get(&DEFAULT_USER_ID) {
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
            (load_user_stats_cache_from_db, load_mob_kills_cache_from_db),
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
fn load_user_stats_cache_from_db(
    mut user_stats_cache: ResMut<UserStatsByPlayerForCompletedGamesCache>,
) {
    if !user_stats_cache.0.is_empty() {
        warn!(
            "evicting data from the in-memory cache. Is this right? {:?}",
            user_stats_cache
        );
    }
    user_stats_cache.0 =
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
    use crate::user_stats::{get_mob_killed_counts_for_user, get_user_stats};
    use bevy::prelude::{apply_state_transition, App, NextState};
    use thetawave_interface::game::historical_metrics::{
        MobKillsByPlayerForCompletedGames, MobsKilledBy1PlayerCacheT, UserStat,
        UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
    };
    use thetawave_interface::spawnable::EnemyMobType;
    use thetawave_interface::states::AppStates;

    #[test]
    fn test_recover_resources_from_db_after_mock_program_restart() {
        let mut app = App::new();
        app.add_state::<AppStates>(); // start game in the main menu state

        app.insert_resource(MobKillsByPlayerForCompletedGames::default());
        app.insert_resource(UserStatsByPlayerForCompletedGamesCache::default());
        assert_eq!(
            get_mob_killed_counts_for_user(DEFAULT_USER_ID),
            MobsKilledBy1PlayerCacheT::default()
        );
        assert_eq!(
            get_user_stats(DEFAULT_USER_ID).unwrap(),
            UserStat::default()
        );
        let mut state = NextState::<AppStates>::default();
        state.set(AppStates::LoadingAssets); // Trigger db init/setup
        apply_state_transition(&mut app.world);
        app.update();

        let mut some_mob_kills_after_1_game = MobKillsByPlayerForCompletedGames::default();
        some_mob_kills_after_1_game.0.insert(
            DEFAULT_USER_ID,
            MobsKilledBy1PlayerCacheT::from([(EnemyMobType::Drone, 15)]),
        );
        app.insert_resource(some_mob_kills_after_1_game.clone());
        state.set(AppStates::LoadingAssets); // repull "forgotten" data from db

        apply_state_transition(&mut app.world);
        app.update();

        assert_eq!(
            &get_mob_killed_counts_for_user(DEFAULT_USER_ID),
            &some_mob_kills_after_1_game
                .0
                .get(&DEFAULT_USER_ID)
                .unwrap()
                .clone()
        );
        // By clearing the resource, we can only recover it if it was remembered somehow.

        // pretend that we just restarted the game.
        app.insert_resource(MobKillsByPlayerForCompletedGames::default());
        state.set(AppStates::LoadingAssets); // repull "forgotten" data from db
        apply_state_transition(&mut app.world);
        app.update();

        assert_eq!(
            &get_mob_killed_counts_for_user(DEFAULT_USER_ID),
            &some_mob_kills_after_1_game
                .0
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
