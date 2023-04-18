use bevy::prelude::*;

use super::CharacterType;

#[derive(Default, Resource)]
pub struct PlayersResource {
    pub player_1_character: Option<CharacterType>,
    pub player_2_character: Option<CharacterType>,
}
