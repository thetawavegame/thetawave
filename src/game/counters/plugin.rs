/// Expose all of the mutations for the within-game metric counters via a bevy plugin.
use crate::{
    collision::SortedCollisionEvent,
    player::PlayerComponent,
    spawnable::{MobDestroyedEvent, SpawnProjectileEvent},
};
use bevy::prelude::{debug, App, Entity, EventReader, OnEnter, Plugin, Query, ResMut, Update};

use std::collections::HashMap;
use thetawave_interface::game::counters::{EnemiesKilledCounter, ShotCounters};
/// Expose all of the mutations for the within-game metric counters via a bevy plugin.
use thetawave_interface::spawnable::MobType;
/// Expose all of the mutations for the within-game metric counters via a bevy plugin.
use thetawave_interface::states::AppStates;

/// Maintains/mutates singleton resources that keep track of metrics for the current game. These are reset on each new game.
pub struct CurrentGameMetricsPlugin;

impl Plugin for CurrentGameMetricsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemiesKilledCounter::default());
        app.insert_resource(ShotCounters::default());
        app.add_systems(
            Update,
            (
                count_enemies_destroyed_system,
                count_shots_fired_by_player_1_system,
                inc_p1_projectile_hits_counter_system,
            ),
        );
        app.add_systems(OnEnter(AppStates::Game), reset_game_metric_counters);
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

fn count_enemies_destroyed_system(
    mut mobs_destroyed_counters: ResMut<EnemiesKilledCounter>,
    mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>,
) {
    for event in mob_destroyed_event_reader.iter() {
        if let MobType::Enemy(enemy_type) = &event.mob_type {
            inc_usize_map(&mut mobs_destroyed_counters.as_mut().0, enemy_type.clone());
        }
    }
}
fn inc_p1_projectile_hits_counter_system(
    mut shot_counters: ResMut<ShotCounters>,
    mut collision_event_reader: EventReader<SortedCollisionEvent>,
    player_query: Query<(Entity, &PlayerComponent)>,
) {
    if let Some(player_1_entity_id) = player_query
        .iter()
        .find(|(_, pc)| pc.player_index == 0)
        .map(|(x, _)| x)
    {
        let n_player_1_hit_shots = collision_event_reader
            .iter()
            .filter(|c| {
                if let Some(projectile_source_id) = match c {
                    SortedCollisionEvent::MobToProjectileIntersection {
                        projectile_source, ..
                    } => Some(projectile_source),
                    SortedCollisionEvent::MobToProjectileContact {
                        projectile_source, ..
                    } => Some(projectile_source),
                    _ => None,
                } {
                    return *projectile_source_id == player_1_entity_id;
                }
                false
            })
            .count();
        shot_counters.n_shots_hit += n_player_1_hit_shots;
    }
}

fn count_shots_fired_by_player_1_system(
    mut shots_fired_event_reader: EventReader<SpawnProjectileEvent>,
    query: Query<(Entity, &PlayerComponent)>,
    mut shot_counters: ResMut<ShotCounters>,
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
        shot_counters.n_shots_fired += n_p1_shots_fired;
    }
}
/// Zero-out all within-game/run metric counters to prepare for the next game.
fn reset_game_metric_counters(
    mut shot_counters: ResMut<ShotCounters>,
    mut mobs_descroyed_counters: ResMut<EnemiesKilledCounter>,
) {
    debug!("mobs killed: {:?}", &shot_counters);
    debug!(
        "mobs_descroyed_counters killed: {:?}",
        &mobs_descroyed_counters
    );
    *shot_counters = ShotCounters::default();
    *mobs_descroyed_counters = EnemiesKilledCounter::default();
}
