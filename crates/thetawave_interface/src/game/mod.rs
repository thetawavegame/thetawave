/// Expose counts about the progress/metrics for the currently running game, along with a plugin
/// for all of the systems that mutate these counts. These counts start at
/// 0 for each new game.

/// Resources/singletons with the within-game/run counts/metrics.
pub mod historical_metrics;
