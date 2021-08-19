use crate::{
    game::GameParametersResource,
    spawnable::{spawn_mob, SpawnableType},
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use rand::{thread_rng, Rng};
use serde::Deserialize;

use super::MobsResource;

pub struct SpawnerTimer(pub Timer);

#[derive(Deserialize)]
pub struct SpawnerResource {
    pub formation_pool: Vec<Formation>,
}

/// Used for storing information about a spawnables in formations
#[derive(Deserialize)]
pub struct FormationSpawnable {
    /// Type of spawnable in formation
    pub spawnable_type: SpawnableType,
    /// Position of the spawnable
    pub position: Vec2,
}

#[derive(Deserialize)]
pub struct Formation {
    pub formation_spawnables: Vec<FormationSpawnable>,
    pub weight: f32,
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

pub fn spawn_formation_system(
    mut commands: Commands,
    spawner_resource: Res<SpawnerResource>,
    mobs: Res<MobsResource>,
    time: Res<Time>,
    mut timer: ResMut<SpawnerTimer>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    let weights = spawner_resource
        .formation_pool
        .iter()
        .map(|x| x.weight)
        .collect();

    let random_idx = weighted_rng(weights);

    if timer.0.tick(time.delta()).just_finished() {
        spawner_resource.formation_pool[random_idx].spawn_formation(
            &mobs,
            &mut commands,
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
