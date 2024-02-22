//! Stats for games _before_ the currently running game. A value of 0 typically means that the
//! corresponding systems are not 'online' to mutate the resources.
use crate::spawnable::EnemyMobType;
use bevy_ecs_macros::Resource;
use std::collections::HashMap;

/// The 'model' of the UserStat Sqlite table. Persisted user stats about past games.
#[derive(Debug, Default, Clone, derive_more::AddAssign, PartialEq, Eq, Hash)]
pub struct UserStat {
    pub total_shots_fired: usize,
    pub total_shots_hit: usize,
    pub total_games_lost: usize,
}
pub type UserStatsByPlayerCacheT = HashMap<usize, UserStat>;
pub type MobsKilledBy1PlayerCacheT = HashMap<EnemyMobType, usize>;
pub type MobsKilledByPlayerCacheT = HashMap<usize, MobsKilledBy1PlayerCacheT>;
/// An in-memory cache of stats for games that have been completed. Keys are "user ids"
#[derive(Debug, Default, Eq, PartialEq, Resource, derive_more::Deref, derive_more::DerefMut)]
pub struct UserStatsByPlayerForCompletedGamesCache(pub UserStatsByPlayerCacheT);

/// An in-memory cache of stats for games that have been completed. Keys are "user ids"
#[derive(Debug, Default, Eq, PartialEq, Resource, derive_more::Deref, derive_more::DerefMut)]
pub struct UserStatsByPlayerForCurrentGameCache(pub UserStatsByPlayerCacheT);

/// An in-memory cache of stats for games that have been completed. Keys are "user ids"
#[derive(
    Debug,
    Default,
    Eq,
    PartialEq,
    Resource,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
)]
pub struct MobKillsByPlayerForCompletedGames {
    pub cache: MobsKilledByPlayerCacheT,
}

/// An in-memory cache of stats for the currently running game. Keys are "user ids"
#[derive(Debug, Default, Eq, PartialEq, Resource, derive_more::Deref, derive_more::DerefMut)]
pub struct MobKillsByPlayerForCurrentGame(pub MobsKilledByPlayerCacheT);

/// The user id of the anonymous/"main" player. IOW "player 1".
pub const DEFAULT_USER_ID: usize = 0;
