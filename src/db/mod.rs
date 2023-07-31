/// CRUD operations to persist data to disk so that it can be safely+portably retrieved across user sessions and
/// thetawave releases. There are public functions to read data (exposing as few db implementation details as possible),
/// while all upserts/mutations/deletions are handled via a Bevy plugin.
pub mod core;
pub mod plugin;
pub mod user_stats;
