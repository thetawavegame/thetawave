use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::spawnable::{self, SpawnConsumableEvent, SpawnMobEvent};

/// Resource for storing collections of formations of spawnables
#[derive(Resource, Deserialize)]
pub struct FormationPoolsResource {
    pub formation_pools: HashMap<FormationPoolType, FormationPool>,
}

/// Collection of formations that can be chosen to be spawned
pub type FormationPool = Vec<Formation>;

/// Types of formation pools, describes a set of enemy formations to spawn in phase
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum FormationPoolType {
    Easy,
    Hard,
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
        spawn_consumable: &mut EventWriter<SpawnConsumableEvent>,
        spawn_mob: &mut EventWriter<SpawnMobEvent>,
    ) {
        // iterate through all spawnables in the formation and spawn at given position
        for formation_spawnable in self.formation_spawnables.iter() {
            // TODO: add cases for items, consumables, etc, as they are added
            // call the appropriate spawn function for the spawnable
            match &formation_spawnable.spawnable_type {
                spawnable::SpawnableType::Mob(mob_type) => spawn_mob.send(SpawnMobEvent {
                    mob_type: mob_type.clone(),
                    position: formation_spawnable.position,
                    rotation: Quat::default(),
                }),

                spawnable::SpawnableType::Consumable(consumable_type) => {
                    spawn_consumable.send(SpawnConsumableEvent {
                        consumable_type: consumable_type.clone(),
                        position: formation_spawnable.position,
                    });
                }
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
    mut spawn_formation: EventReader<SpawnFormationEvent>,
    mut spawn_consumable: EventWriter<SpawnConsumableEvent>,
    mut spawn_mob: EventWriter<SpawnMobEvent>,
) {
    for event in spawn_formation.iter() {
        event
            .formation
            .spawn_formation(&mut spawn_consumable, &mut spawn_mob);
    }
}
