use std::collections::HashMap;

use crate::{
    game::GameParametersResource,
    spawnable::{spawn_mob, FormationPool, MobsResource, SpawnableType},
};
use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use serde::Deserialize;

pub type FormationPoolsResource = HashMap<FormationPoolType, FormationPool>;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FormationPoolType {
    Easy,
    Asteroids,
}

/// Used for storing information about a spawnables in formations
#[derive(Deserialize, Clone)]
pub struct FormationSpawnable {
    /// Type of spawnable in formation
    pub spawnable_type: SpawnableType,
    /// Position of the spawnable
    pub position: Vec2,
}

/// A group of spawnables to be spawned at the same time
#[derive(Deserialize, Clone)]
pub struct Formation {
    /// Vector of spawnables with positions
    pub formation_spawnables: Vec<FormationSpawnable>,
    /// Relative likelihood of spawning
    pub weight: f32,
    /// Time until next spawn
    pub period: f32,
}

impl Formation {
    /// Spawn all spawnables in the formation at once
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
