//! Resources for managing players

mod character;
mod players;

pub use self::character::{Character, CharacterType, CharactersResource};
pub use self::players::{PlayerInput, PlayersResource};
