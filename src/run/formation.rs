use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::spawnable::{SpawnMobEvent, SpawnableType};

use crate::{spawnable::SpawnConsumableEvent, tools::weighted_rng};

/// Resource for storing collections of formations of spawnables
#[derive(Resource, Deserialize)]
pub struct FormationPoolsResource {
    pub formation_pools: HashMap<String, FormationPool>,
}

impl FormationPoolsResource {
    pub fn get_random_formation(&self, pool_key: String) -> Option<Formation> {
        let formation_pool = match self.formation_pools.get(&pool_key) {
            Some(pool) => pool,
            None => {
                error!("No formation pool found for given key: {}", pool_key);
                return None;
            }
        };

        let weights = formation_pool.iter().map(|x| x.weight).collect();

        let random_idx = weighted_rng(weights);

        formation_pool.get(random_idx).cloned()
    }
}

/// Collection of formations that can be chosen to be spawned
pub type FormationPool = Vec<Formation>;

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
        spawn_consumable: &mut EventWriter<SpawnConsumableEvent>,
        spawn_mob: &mut EventWriter<SpawnMobEvent>,
    ) {
        // iterate through all spawnables in the formation and spawn at given position
        for formation_spawnable in self.formation_spawnables.iter() {
            // TODO: add cases for items, consumables, etc, as they are added
            // call the appropriate spawn function for the spawnable
            match &formation_spawnable.spawnable_type {
                thetawave_interface::spawnable::SpawnableType::Mob(mob_type) => {
                    spawn_mob.send(SpawnMobEvent {
                        mob_type: mob_type.clone(),
                        position: formation_spawnable.position,
                        rotation: Quat::default(),
                        boss: false,
                    })
                }

                SpawnableType::Consumable(consumable_type) => {
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
#[derive(Event)]
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
