/// Expose all of the mutations for the within-game metric counters via a bevy plugin.
use crate::collision::SortedCollisionEvent;
use crate::spawnable::FireWeaponEvent;
use bevy::prelude::{debug, App, Entity, EventReader, OnEnter, Plugin, Query, ResMut, Update};
use thetawave_interface::player::PlayerIDComponent;

use std::collections::HashMap;
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, MobKillsByPlayerForCurrentGame, UserStat,
    UserStatsByPlayerForCompletedGamesCache, UserStatsByPlayerForCurrentGameCache, DEFAULT_USER_ID,
};
use thetawave_interface::spawnable::{MobDestroyedEvent, MobType};
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
        .or_insert_with(Default::default);
    for event in mob_destroyed_event_reader.read() {
        if let MobType::Enemy(enemy_type) = &event.mob_type {
            inc_usize_map(player_1_mob_counters, *enemy_type);
        }
    }
}
fn find_player_1<'a, 'b: 'a>(
    player_query: &'a Query<(Entity, &'b PlayerIDComponent)>,
) -> Option<Entity> {
    player_query
        .iter()
        .find(|(_, id)| matches!(id, PlayerIDComponent::One))
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
fn inc_in_memory_projectile_hits_counter_system(
    mut current_game_user_stats: ResMut<UserStatsByPlayerForCurrentGameCache>,
    mut collision_event_reader: EventReader<SortedCollisionEvent>,
    player_query: Query<(Entity, &PlayerIDComponent)>,
) {
    if let Some(player_1_entity_id) = find_player_1(&player_query) {
        let n_player_1_hit_shots = collision_event_reader
            .read()
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
    mut fire_weapon_event_reader: EventReader<FireWeaponEvent>,
    query: Query<(Entity, &PlayerIDComponent)>,
) {
    let maybe_player_1_entity_id = query
        .iter()
        .find(|(_, id)| matches!(id, PlayerIDComponent::One))
        .map(|(x, _)| x);
    let n_p1_shots_fired = match maybe_player_1_entity_id {
        Some(player_1) => fire_weapon_event_reader
            .read()
            .filter(|x| x.source_entity == player_1)
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
            .or_insert_with(|| UserStat {
                total_shots_fired: n_p1_shots_fired,
                ..Default::default()
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
                .entry(*user_id)
                .or_default()
                .entry(*mob_type)
                .and_modify(|x| {
                    *x += n_mobs;
                })
                .or_insert(*n_mobs);
        }
    }
    mobs_destroyed_counters_by_player.clear();
    for (user_id, current_game_stats) in (**current_game_user_stats).iter() {
        (*historical_games_shot_counts)
            .entry(*user_id)
            .and_modify(|x| {
                *x += current_game_stats.clone();
            })
            .or_insert(current_game_stats.clone());
    }
    current_game_user_stats.clear();
}

#[cfg(test)]
mod test {
    use std::f32::consts::{FRAC_PI_2, PI};

    use crate::collision::SortedCollisionEvent;
    use crate::game::counters::plugin::CountingMetricsPlugin;
    use crate::player::{CharactersResource, PlayerPlugin};
    use crate::spawnable::FireWeaponEvent;
    use bevy::input::InputPlugin;
    use bevy::math::Vec2;
    use bevy::prelude::{App, Component, Events};
    use bevy::state::app::{AppExtStates, StatesPlugin};
    use bevy::MinimalPlugins;
    use thetawave_interface::audio::SoundEffectType;
    use thetawave_interface::character::{Character, CharacterType};
    use thetawave_interface::game::historical_metrics::{
        MobKillsByPlayerForCurrentGame, UserStatsByPlayerForCurrentGameCache, DEFAULT_USER_ID,
    };
    use thetawave_interface::player::PlayerBundle;
    use thetawave_interface::spawnable::{
        EnemyMobType, Faction, MobDestroyedEvent, MobType, ProjectileType, SpawnPosition,
    };
    use thetawave_interface::states::{AppStates, GameStates};
    use thetawave_interface::weapon::{ArcPatternData, SpreadPattern, WeaponProjectileData};

    fn base_app_required_for_counting_metrics() -> App {
        let mut app = App::new();
        app.add_plugins((
            StatesPlugin,
            InputPlugin,
            PlayerPlugin,
            CountingMetricsPlugin,
            MinimalPlugins,
        ));
        app.init_state::<AppStates>()
            .init_state::<GameStates>()
            .add_event::<SortedCollisionEvent>()
            .add_event::<MobDestroyedEvent>()
            .add_event::<FireWeaponEvent>()
            .insert_resource(UserStatsByPlayerForCurrentGameCache::default());

        app
    }
    #[derive(Component, Default, Copy, Clone)]
    struct NullComponent;
    #[test]
    fn test_increment_player_1_mobs_killed_counter() {
        let mut app = base_app_required_for_counting_metrics();
        app.insert_resource(MobKillsByPlayerForCurrentGame::default());
        app.add_event::<MobDestroyedEvent>();
        // app.add_systems(Update, send_mob_drone_destroyed_by_dummy_entity_event);

        let entity = app.world_mut().spawn(NullComponent::default()).id();
        let mut events = app.world_mut().resource_mut::<Events<MobDestroyedEvent>>();
        events.send(MobDestroyedEvent {
            mob_type: MobType::Enemy(EnemyMobType::Drone),
            entity,
            is_boss: false,
        });
        app.update();
        let got_mob_kills = app
            .world()
            .get_resource::<MobKillsByPlayerForCurrentGame>()
            .unwrap()
            .get(&DEFAULT_USER_ID)
            .unwrap();
        assert_eq!(got_mob_kills.get(&EnemyMobType::Drone).unwrap(), &1);
    }
    #[test]
    fn test_increment_player_1_shot_counter() {
        let mut app = base_app_required_for_counting_metrics();

        let player_1_character: Character = app
            .world()
            .get_resource::<CharactersResource>()
            .unwrap()
            .characters
            .get(&CharacterType::Captain)
            .cloned()
            .unwrap();
        let player_1: PlayerBundle = PlayerBundle::from(&player_1_character);

        let player_1_entity = app.world_mut().spawn(player_1);
        let player_1_projectile_event = FireWeaponEvent {
            weapon_projectile_data: WeaponProjectileData {
                ammunition: ProjectileType::Bullet(Faction::Ally),
                damage: 0,
                position: SpawnPosition::Local(Vec2::new(0.0, 40.0)),
                speed: 1.0,
                direction: FRAC_PI_2,
                despawn_time: 0.0,
                count: 1,
                spread_pattern: SpreadPattern::Arc(ArcPatternData {
                    spread_weights: Vec2::new(0.5, 1.0),
                    max_spread: FRAC_PI_2,
                    projectile_gap: PI,
                }),
                size: 1.0,
                sound: SoundEffectType::PlayerFireBlast,
            },
            source_transform: Default::default(),
            source_entity: player_1_entity.id(),
            initial_motion: Default::default(),
        };
        app.world_mut()
            .send_event(player_1_projectile_event.clone());
        app.update();
        let n_p1_shots_fired = app
            .world()
            .get_resource::<UserStatsByPlayerForCurrentGameCache>()
            .unwrap()
            .0
            .get(&DEFAULT_USER_ID)
            .unwrap()
            .total_shots_fired;
        assert_eq!(n_p1_shots_fired, 1);
        app.world_mut()
            .send_event(player_1_projectile_event.clone());
        app.update();
        // apply_state_transition(&mut app.world);
        let n_p1_shots_fired_2 = app
            .world()
            .get_resource::<UserStatsByPlayerForCurrentGameCache>()
            .unwrap()
            .0
            .get(&DEFAULT_USER_ID)
            .unwrap()
            .total_shots_fired;
        assert_eq!(n_p1_shots_fired_2, 2);
    }
}
