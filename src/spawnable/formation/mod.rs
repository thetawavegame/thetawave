use crate::{
    game::GameParametersResource,
    spawnable::{spawn_mob, SpawnableType},
    SpawnableTextureAtlasHandleIds,
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use serde::Deserialize;

use super::MobsResource;

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
        texture_atlases: &Res<Assets<TextureAtlas>>,
        texture_atlas_handle_ids: &Res<SpawnableTextureAtlasHandleIds>,
        rapier_config: &Res<RapierConfiguration>,
        game_parameters: &Res<GameParametersResource>,
    ) {
        for formation_spawnable in self.formation_spawnables.iter() {
            // spawn enemy
            match &formation_spawnable.spawnable_type {
                SpawnableType::Mob(mob_type) => {
                    let mob_data = &mobs.mobs[mob_type];

                    spawn_mob(
                        mob_data,
                        formation_spawnable.position,
                        commands,
                        texture_atlases,
                        texture_atlas_handle_ids,
                        rapier_config,
                        game_parameters,
                    )
                }
                _ => {}
            }
        }
    }
}

pub fn spawn_formation_system(
    mut commands: Commands,
    spawner_resource: Res<SpawnerResource>,
    mobs: Res<MobsResource>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle_ids: Res<SpawnableTextureAtlasHandleIds>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    spawner_resource.formation_pool[0].spawn_formation(
        &mobs,
        &mut commands,
        &texture_atlases,
        &texture_atlas_handle_ids,
        &rapier_config,
        &game_parameters,
    )
}
