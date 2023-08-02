/// Stats for games _before_ the currently running game. A value of 0 typically means that the
/// corresponding systems are not 'online' to mutate the resources.

/// The 'model' of the UserStat Sqlite table. Persisted user stats about past games.
#[derive(Debug, Default)]
pub struct UserStat {
    pub user_id: usize,
    pub total_shots_fired: usize,
    pub total_games_lost: usize,
}

pub const DEFAULT_USER_ID: usize = 0;
