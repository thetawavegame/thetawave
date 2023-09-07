use crate::character::CharacterType;
use bevy_ecs::system::Resource;

#[derive(Resource, Debug)]
pub struct PlayersResource {
    pub player_data: Vec<Option<PlayerData>>,
}

#[derive(Debug, Clone)]
pub struct PlayerData {
    pub character: CharacterType,
    pub input: PlayerInput,
}

impl Default for PlayersResource {
    fn default() -> Self {
        PlayersResource {
            player_data: vec![None, None, None, None],
        }
    }
}

impl PlayersResource {
    // A method to get a vector of all used inputs
    pub fn get_used_inputs(&self) -> Vec<PlayerInput> {
        self.player_data
            .iter()
            .filter_map(|player_data| player_data.clone().map(|data| data.input))
            .collect()
    }
}

/// Player input
#[derive(Clone, PartialEq, Debug)]
pub enum PlayerInput {
    Keyboard,
    Gamepad(usize),
}
