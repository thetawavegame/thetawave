use bevy::prelude::*;
use bevy_rapier2d::physics::RapierConfiguration;
use serde::Deserialize;
use std::collections::HashMap;

use crate::{game, spawnable};

pub type FormationPoolsResource = HashMap<FormationPoolType, FormationPool>;
pub type FormationPool = Vec<Formation>;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FormationPoolType {
    Easy,
    Asteroids,
}

/// Used for storing information about a spawnables in formations
#[derive(Deserialize, Clone)]
pub struct FormationSpawnable {
    /// Type of spawnable in formation
    pub spawnable_type: spawnable::SpawnableType,
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
        mobs: &spawnable::MobsResource,
        commands: &mut Commands,
        rapier_config: &RapierConfiguration,
        game_parameters: &game::GameParametersResource,
    ) {
        for formation_spawnable in self.formation_spawnables.iter() {
            // TODO: add cases for items, consumables, etc, as they are added
            // spawn enemy
            match &formation_spawnable.spawnable_type {
                spawnable::SpawnableType::Mob(mob_type) => spawnable::spawn_mob(
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

/// Event for spawning formations
pub struct SpawnFormationEvent {
    //pub formation_pool: FormationPoolType,
    pub formation: Formation,
}

/// Manages spawning of formations
pub fn spawn_formation_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnFormationEvent>,
    mobs: Res<spawnable::MobsResource>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<game::GameParametersResource>,
) {
    for event in event_reader.iter() {
        event
            .formation
            .spawn_formation(&mobs, &mut commands, &rapier_config, &game_parameters);
    }
}
