use crate::{
    game::GameParametersResource,
    spawnable::{spawn_mob, SpawnableType},
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
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
        mobs: &Res<MobsResource>,
        commands: &mut Commands,
        rapier_config: &Res<RapierConfiguration>,
        game_parameters: &Res<GameParametersResource>,
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
    if timer.0.tick(time.delta()).just_finished() {
        spawner_resource.formation_pool[0].spawn_formation(
            &mobs,
            &mut commands,
            &rapier_config,
            &game_parameters,
        );
    }
}
