/// Expose all of the mutations for the within-game metric counters via a bevy plugin.
use crate::{
    collision::SortedCollisionEvent,
    player::PlayerComponent,
    spawnable::{MobDestroyedEvent, SpawnProjectileEvent},
};
use bevy::prelude::{debug, App, Entity, EventReader, OnEnter, Plugin, Query, ResMut, Update};

use std::collections::HashMap;
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, MobKillsByPlayerForCurrentGame, UserStat,
    UserStatsByPlayerForCompletedGamesCache, UserStatsByPlayerForCurrentGameCache, DEFAULT_USER_ID,
};
use thetawave_interface::spawnable::MobType;
use thetawave_interface::states::AppStates;

/// Maintains/mutates singleton resources that keep track of metrics for the current game. Mostly
/// incrementing a reseting counters.
pub struct CountingMetricsPlugin;

impl Plugin for CountingMetricsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MobKillsByPlayerForCompletedGames::default());
        app.insert_resource(MobKillsByPlayerForCurrentGame::default());
        app.insert_resource(UserStatsByPlayerForCompletedGamesCache::default());
        app.insert_resource(UserStatsByPlayerForCurrentGameCache::default());
        app.add_systems(
            Update,
            (
                inc_in_memory_mob_destroyed_for_current_game_cache,
                count_shots_fired_by_player_1_system,
                inc_in_memory_projectile_hits_counter_system,
                inc_completed_games_played_counter,
            ),
        );
        app.add_systems(
            OnEnter(AppStates::Game),
            roll_current_game_counters_into_completed_game_metrics,
        );
    }
}

fn inc_usize_map<T: std::hash::Hash + std::cmp::Eq>(map: &mut HashMap<T, usize>, key: T) {
    match map.get(&key) {
        Some(v) => {
            map.insert(key, v + 1);
        }
        None => {
            map.insert(key, 1);
        }
    }
}
fn inc_completed_games_played_counter(
    mut user_stats: ResMut<UserStatsByPlayerForCompletedGamesCache>,
) {
    (**user_stats)
        .entry(DEFAULT_USER_ID)
        .or_default()
        .total_games_lost += 1;
}

fn inc_in_memory_mob_destroyed_for_current_game_cache(
    mut mobs_destroyed_counters_by_player: ResMut<MobKillsByPlayerForCurrentGame>,
    mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>,
) {
    let player_1_mob_counters = (**mobs_destroyed_counters_by_player)
        .entry(DEFAULT_USER_ID)
        .or_insert_with(|| Default::default());
    for event in mob_destroyed_event_reader.iter() {
        if let MobType::Enemy(enemy_type) = &event.mob_type {
            inc_usize_map(player_1_mob_counters, enemy_type.clone());
        }
    }
}
fn find_player_1<'a, 'b: 'a>(
    player_query: &'a Query<(Entity, &'b PlayerComponent)>,
) -> Option<Entity> {
    player_query
        .iter()
        .find(|(_, pc)| pc.player_index == 0)
        .map(|(entity, _)| entity)
}
fn mob_projectile_collision_originates_from_entity(
    collision: &SortedCollisionEvent,
    entity: &Entity,
) -> bool {
    if let Some(projectile_source_id) = match collision {
        SortedCollisionEvent::MobToProjectileIntersection {
            projectile_source, ..
        } => Some(projectile_source),
        SortedCollisionEvent::MobToProjectileContact {
            projectile_source, ..
        } => Some(projectile_source),
        _ => None,
    } {
        return *projectile_source_id == *entity;
    }
    false
}
fn inc_in_memory_projectile_hits_counter_system<'b>(
    mut current_game_user_stats: ResMut<UserStatsByPlayerForCurrentGameCache>,
    mut collision_event_reader: EventReader<SortedCollisionEvent>,
    player_query: Query<(Entity, &'b PlayerComponent)>,
) {
    if let Some(player_1_entity_id) = find_player_1(&player_query) {
        let n_player_1_hit_shots = collision_event_reader
            .iter()
            .filter(|c| mob_projectile_collision_originates_from_entity(c, &player_1_entity_id))
            .count();
        if let Some(ref mut player_1_user_stats) =
            (**current_game_user_stats).get_mut(&DEFAULT_USER_ID)
        {
            player_1_user_stats.total_shots_hit += n_player_1_hit_shots;
        }
    }
}

fn count_shots_fired_by_player_1_system(
    mut current_game_user_stats: ResMut<UserStatsByPlayerForCurrentGameCache>,
    mut shots_fired_event_reader: EventReader<SpawnProjectileEvent>,
    query: Query<(Entity, &PlayerComponent)>,
) {
    let maybe_player_1_entity_id = query
        .iter()
        .find(|(_, pc)| pc.player_index == 0)
        .map(|(x, _)| x);
    let n_p1_shots_fired = match maybe_player_1_entity_id {
        Some(player_1) => shots_fired_event_reader
            .iter()
            .filter(|x| x.source == player_1)
            .count(),
        None => 0,
    };
    if n_p1_shots_fired > 0 {
        debug!(
            "Incrementing total player 1 shots by {}, n_p1_shots_fired",
            n_p1_shots_fired
        );
        current_game_user_stats
            .entry(DEFAULT_USER_ID)
            .and_modify(|x| {
                x.total_shots_fired += n_p1_shots_fired;
            })
            .or_insert_with(|| {
                let mut stat = UserStat::default();
                stat.total_shots_fired = n_p1_shots_fired;
                stat
            });
    }
}
/// Analagous to "log rolling" except we merge counters and add integers.
fn roll_current_game_counters_into_completed_game_metrics(
    mut current_game_user_stats: ResMut<UserStatsByPlayerForCurrentGameCache>,
    mut mobs_destroyed_counters_by_player: ResMut<MobKillsByPlayerForCurrentGame>,
    mut historical_games_shot_counts: ResMut<UserStatsByPlayerForCompletedGamesCache>,
    mut historical_games_enemy_mob_kill_counts: ResMut<MobKillsByPlayerForCompletedGames>,
) {
    debug!(
        "mobs_destroyed_counters_by_player : {:?}",
        &mobs_destroyed_counters_by_player
    );
    for (user_id, current_game_mob_kills) in (**mobs_destroyed_counters_by_player).iter() {
        for (mob_type, n_mobs) in current_game_mob_kills.iter() {
            (*historical_games_enemy_mob_kill_counts)
                .entry(user_id.clone())
                .or_default()
                .entry(mob_type.clone())
                .and_modify(|x| {
                    *x += n_mobs;
                })
                .or_insert(*n_mobs);
        }
    }
    mobs_destroyed_counters_by_player.clear();
    for (user_id, current_game_stats) in (**current_game_user_stats).iter() {
        (*historical_games_shot_counts)
            .entry(user_id.clone())
            .and_modify(|x| {
                *x += current_game_stats.clone();
            })
            .or_insert(current_game_stats.clone());
    }
    current_game_user_stats.clear();
}
