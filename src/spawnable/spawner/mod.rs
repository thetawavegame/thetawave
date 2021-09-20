use crate::{game::GameParametersResource, spawnable::MobsResource, tools::weighted_rng};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use core::time::Duration;
use serde::Deserialize;

mod formation;

/// Spawner resource data in spawner.ron data file
#[derive(Deserialize)]
pub struct SpawnerResourceData {
    pub formation_pool: Vec<formation::Formation>,
    pub initial_duration: f32,
}

/// Spawner resource for managing formations and spawning
pub struct SpawnerResource {
    pub formation_pool: Vec<formation::Formation>,
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
