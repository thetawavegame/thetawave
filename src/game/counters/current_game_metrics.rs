/// Singletons for the 'current running game'. We assume there is only 1 current running game for
/// 1 invocation of thetawave. Reinitialized for each new game.
use crate::spawnable::{ConsumableType, EnemyMobType};
use bevy::prelude::*;
use std::collections::HashMap;
/// Stats about the ongoing game. Reinitialized for each new game (e.x. after each game over).
#[derive(Resource, Default, Debug)]
pub struct ShotCounters {
    pub n_shots_fired: usize,
    pub n_shots_hit: usize,
}

#[derive(Resource, Default)]
pub struct CollectedConsumableCounters(HashMap<ConsumableType, usize>);
/// The number of enemies that have have been killed by player 1 in the current running game.
#[derive(Resource, Default, Debug)]
pub struct EnemiesKilledCounter(pub HashMap<EnemyMobType, usize>);
