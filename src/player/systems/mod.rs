//! Systems for managing players

mod attacks;
mod movement;

pub use self::attacks::player_fire_weapon_system;
pub use self::movement::player_movement_system;
