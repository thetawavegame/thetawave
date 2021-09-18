use crate::{
    game::GameParametersResource,
    spawnable::{spawn_mob, SpawnableType},
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use core::time::Duration;
use rand::{thread_rng, Rng};
use serde::Deserialize;

use super::MobsResource;

/// Spawner resource data in spawner.ron data file
#[derive(Deserialize)]
pub struct SpawnerResourceData {
    pub formation_pool: Vec<Formation>,
    pub initial_duration: f32,
}

/// Spawner resource for managing formations and spawning
pub struct SpawnerResource {
    pub formation_pool: Vec<Formation>,
    pub spawn_timer: Timer,
}

impl From<SpawnerResourceData> for SpawnerResource {
    fn from(data: SpawnerResourceData) -> Self {
        SpawnerResource {
            formation_pool: data.formation_pool,
            spawn_timer: Timer::from_seconds(data.initial_duration, true),
        }
    }
}

impl SpawnerResource {
    /// Set time until next spawn
    fn set_spawn_duration(&mut self, formation_idx: usize) {
        let new_duration = self.formation_pool[formation_idx].period;

        self.spawn_timer
            .set_duration(Duration::from_secs_f32(new_duration));
    }

    /// Spawn a random formation
    pub fn spawn_random_formation(
        &mut self,
        commands: &mut Commands,
        mobs: &MobsResource,
        rapier_config: &RapierConfiguration,
        game_parameters: &GameParametersResource,
    ) {
        let weights = self.formation_pool.iter().map(|x| x.weight).collect();

        let random_idx = weighted_rng(weights);

        self.formation_pool[random_idx].spawn_formation(
            mobs,
            commands,
            rapier_config,
            game_parameters,
        );
        self.set_spawn_duration(random_idx)
    }
}

/// Used for storing information about a spawnables in formations
#[derive(Deserialize)]
pub struct FormationSpawnable {
    /// Type of spawnable in formation
    pub spawnable_type: SpawnableType,
    /// Position of the spawnable
    pub position: Vec2,
}

/// A group of spawnables to be spawned at the same time
#[derive(Deserialize)]
pub struct Formation {
    /// Vector of spawnables with positions
    pub formation_spawnables: Vec<FormationSpawnable>,
    /// Relative likelihood of spawning
    pub weight: f32,
    /// Time until next spawn
    pub period: f32,
}

impl Formation {
    pub fn spawn_formation(
        &self,
        mobs: &MobsResource,
        commands: &mut Commands,
        rapier_config: &RapierConfiguration,
        game_parameters: &GameParametersResource,
    ) {
        for formation_spawnable in self.formation_spawnables.iter() {
            // TODO: add cases for items, consumables, etc, as they are added
            // spawn enemy
            match &formation_spawnable.spawnable_type {
                SpawnableType::Mob(mob_type) => spawn_mob(
                    mob_type,
                    mobs,
                    formation_spawnable.position,
                    commands,
                    rapier_config,
                    game_parameters,
                ),
                _ => {}
            }
        }
    }
}

/// Manage regular spawning of entities
pub fn spawner_system(
    mut commands: Commands,
    mut spawner_resource: ResMut<SpawnerResource>,
    mobs: Res<MobsResource>,
    time: Res<Time>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    if spawner_resource
        .spawn_timer
        .tick(time.delta())
        .just_finished()
    {
        spawner_resource.spawn_random_formation(
            &mut commands,
            &mobs,
            &rapier_config,
            &game_parameters,
        );
    }
}

/// Randomly picks index of vector using weights
/// Takes in a vector of weights
pub fn weighted_rng(probs: Vec<f32>) -> usize {
    let prob_space = probs.iter().fold(0.0, |sum, prob| sum + prob);
    let pos = thread_rng().gen::<f32>() * prob_space;
    let mut sum = 0.0;
    for (idx, prob) in probs.iter().enumerate() {
        sum += prob;
        if sum > pos {
            return idx;
        }
    }
    unreachable!("Error in probabilities.");
}
