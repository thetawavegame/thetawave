use std::collections::HashMap;

use bevy::prelude::*;
use serde::Deserialize;

use thetawave_interface::character::{Character, CharacterType};

/// Manages all characters
#[derive(Resource, Deserialize)]
pub struct CharactersResource {
    /// Names mapped to characters for all characters
    pub characters: HashMap<CharacterType, Character>,
}
