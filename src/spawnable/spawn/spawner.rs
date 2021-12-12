use crate::{
    game::GameParametersResource, level::SpawnFormationEvent, spawnable::MobsResource,
    tools::weighted_rng,
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use core::time::Duration;
use serde::Deserialize;

pub type FormationPool = Vec<super::formation::Formation>;

/// Spawner resource data in spawner.ron data file
#[derive(Deserialize)]
pub struct SpawnerResourceData {
    /// Pool of formations that can be spawned
    pub formation_pool: super::FormationPoolType,
    /// Delay before first formation is spawned
    pub initial_duration: f32,
}

/// Spawner resource for managing formations and spawning
pub struct SpawnerResource {
    /// Pool of formations that can be spawned
    pub formation_pool: super::FormationPoolType,
    /// Tracks time until next formation is spawned
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
    fn set_spawn_duration(
        &mut self,
        formation_idx: usize,
        formation_pools: &super::FormationPoolsResource,
    ) {
        let new_duration = formation_pools[&self.formation_pool][formation_idx].period;

        self.spawn_timer
            .set_duration(Duration::from_secs_f32(new_duration));
    }

    /// Spawn a random formation
    pub fn spawn_random_formation(
        &mut self,
        commands: &mut Commands,
        mobs: &MobsResource,
        formation_pools: &super::FormationPoolsResource,
        rapier_config: &RapierConfiguration,
        game_parameters: &GameParametersResource,
    ) {
        let weights = formation_pools[&self.formation_pool]
            .iter()
            .map(|x| x.weight)
            .collect();

        let random_idx = weighted_rng(weights);

        formation_pools[&self.formation_pool][random_idx].spawn_formation(
            mobs,
            commands,
            rapier_config,
            game_parameters,
        );
        self.set_spawn_duration(random_idx, formation_pools);
    }
}

/// Manage regular spawning of entities
pub fn spawner_system(
    mut commands: Commands,
    mut spawner_resource: ResMut<SpawnerResource>,
    mobs: Res<MobsResource>,
    formation_pools: Res<super::FormationPoolsResource>,
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
            &formation_pools,
            &rapier_config,
            &game_parameters,
        );
    }
}

pub fn spawn_formation_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnFormationEvent>,
    mobs: Res<MobsResource>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    for event in event_reader.iter() {
        event
            .formation
            .spawn_formation(&mobs, &mut commands, &rapier_config, &game_parameters);
        //self.set_spawn_duration(random_idx, formation_pools);
    }
}
