/// Expose counts about the progress/metrics for the currently running game, along with a plugin
/// for all of the systems that mutate these counts. These counts start at
/// 0 for each new game.

/// Resources/singletons with the within-game/run counts/metrics.
pub mod current_game_metrics;
/// All of the mutations exposed for the singletons in the above module.
pub mod plugin;
