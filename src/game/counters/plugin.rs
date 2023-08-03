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
            ),
        );
        app.add_systems(
            OnEnter(AppStates::Game),
            roll_current_game_counters_into_completed_game_metrics,
        );
        app.add_systems(
            OnEnter(AppStates::GameOver),
            inc_completed_games_played_counter,
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

#[cfg(test)]
mod test {
    use crate::collision::SortedCollisionEvent;
    use crate::game::counters::plugin::CountingMetricsPlugin;
    use crate::player::{
        Character, CharacterType, CharactersResource, PlayerComponent, PlayerPlugin,
    };
    use crate::spawnable::{MobDestroyedEvent, SpawnProjectileEvent};
    use bevy::prelude::{App, Commands, Component, EventWriter, Update};
    use thetawave_interface::game::historical_metrics::{
        MobKillsByPlayerForCurrentGame, UserStatsByPlayerForCurrentGameCache, DEFAULT_USER_ID,
    };
    use thetawave_interface::spawnable::{EnemyMobType, Faction, MobType, ProjectileType};
    use thetawave_interface::states::{AppStates, GameStates};

    fn base_app_required_for_counting_metrics() -> App {
        let mut app = App::new();
        app.add_state::<AppStates>()
            .add_state::<GameStates>()
            .add_event::<SortedCollisionEvent>()
            .add_event::<MobDestroyedEvent>()
            .add_event::<SpawnProjectileEvent>()
            .insert_resource(UserStatsByPlayerForCurrentGameCache::default())
            .add_plugins((PlayerPlugin, CountingMetricsPlugin));
        app
    }
    #[derive(Component, Default, Copy, Clone)]
    struct NullComponent;
    fn send_mob_drone_destroyed_by_dummy_entity_event(
        mut commands: Commands,
        mut mob_destroyed_event_writer: EventWriter<MobDestroyedEvent>,
    ) {
        let entity = commands.spawn(NullComponent::default()).id();

        mob_destroyed_event_writer.send(MobDestroyedEvent {
            mob_type: MobType::Enemy(EnemyMobType::Drone),
            entity,
        });
    }
    #[test]
    fn test_increment_player_1_mobs_killed_counter() {
        let mut app = base_app_required_for_counting_metrics();
        app.insert_resource(MobKillsByPlayerForCurrentGame::default());
        app.add_event::<MobDestroyedEvent>();
        app.add_systems(Update, send_mob_drone_destroyed_by_dummy_entity_event);
        app.update();
        let got_mob_kills = app
            .world
            .get_resource::<MobKillsByPlayerForCurrentGame>()
            .unwrap()
            .get(&DEFAULT_USER_ID)
            .unwrap();
        assert_eq!(got_mob_kills.get(&EnemyMobType::Drone).unwrap(), &1);
        app.update();
    }
    #[test]
    fn test_increment_player_1_shot_counter() {
        let mut app = base_app_required_for_counting_metrics();

        let player_1_character: Character = app
            .world
            .get_resource::<CharactersResource>()
            .unwrap()
            .characters
            .get(&CharacterType::Captain)
            .cloned()
            .unwrap();
        let player_1: PlayerComponent = PlayerComponent::from(&player_1_character);

        let player_1_entity = app.world.spawn(player_1.clone());
        let player_1_projectile_event = SpawnProjectileEvent {
            projectile_type: ProjectileType::Bullet(Faction::Ally),
            transform: Default::default(),
            damage: 0.0,
            despawn_time: 0.0,
            initial_motion: Default::default(),
            health: None,
            source: player_1_entity.id(),
        };
        app.world.send_event(player_1_projectile_event.clone());
        app.update();
        let n_p1_shots_fired = app
            .world
            .get_resource::<UserStatsByPlayerForCurrentGameCache>()
            .unwrap()
            .0
            .get(&DEFAULT_USER_ID)
            .unwrap()
            .total_shots_fired;
        assert_eq!(n_p1_shots_fired, 1);
        app.world.send_event(player_1_projectile_event.clone());
        app.update();
        // apply_state_transition(&mut app.world);
        let n_p1_shots_fired_2 = app
            .world
            .get_resource::<UserStatsByPlayerForCurrentGameCache>()
            .unwrap()
            .0
            .get(&DEFAULT_USER_ID)
            .unwrap()
            .total_shots_fired;
        assert_eq!(n_p1_shots_fired_2, 2);
    }
}
