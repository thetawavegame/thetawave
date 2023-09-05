use crate::character::CharacterType;
use bevy_ecs::system::Resource;

#[derive(Resource, Debug)]
pub struct PlayersResource {
    pub player_characters: Vec<Option<CharacterType>>,
    pub player_inputs: Vec<Option<PlayerInput>>,
}

impl Default for PlayersResource {
    fn default() -> Self {
        PlayersResource {
            player_characters: vec![None, None, None, None],
            player_inputs: vec![None, None, None, None],
        }
    }
}

/// Player input
#[derive(Clone, PartialEq, Debug)]
pub enum PlayerInput {
    Keyboard,
    Gamepad(usize),
}
