use crate::character::{Character, CharacterType};
use bevy_ecs::prelude::Component;
use bevy_ecs::system::Resource;
use bevy_math::Vec2;
use bevy_time::{Timer, TimerMode};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

/// Parameters for how to spawn new players. By default, the player can do anything.
#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct InputRestrictionsAtSpawn(InputRestrictions);

/// Things the player is not allowed to do.
#[derive(Resource, Debug, Default)]
pub struct InputRestrictions {
    pub forbid_main_attack_reason: Option<String>,
    pub forbid_special_attack_reason: Option<String>,
}
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

/// Component for managing core attributes of the player
#[derive(Component, Debug, Clone)]
pub struct PlayerComponent {
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
    /// Collider dimensions
    pub collider_dimensions: Vec2,
    /// Collider density
    pub collider_density: f32,
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and consumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    /// Timer for ability cooldown
    pub ability_cooldown_timer: Timer,
    /// Timer for ability action
    pub ability_action_timer: Option<Timer>,
    /// Type of ability
    pub ability_type: AbilityType,
    pub megablast_offset_position: Vec2,
    /// Whether the player responds to move inputs
    pub movement_enabled: bool,
    /// Multiplier for incoming damage
    pub incoming_damage_multiplier: f32,
    /// Index of the player
    pub player_index: usize,
}

impl From<&Character> for PlayerComponent {
    fn from(character: &Character) -> Self {
        PlayerComponent {
            acceleration: character.acceleration,
            deceleration: character.deceleration,
            speed: character.speed,
            collider_dimensions: character.collider_dimensions,
            collider_density: character.collider_density,
            collision_damage: character.collision_damage,
            attraction_distance: character.attraction_distance,
            attraction_acceleration: character.attraction_acceleration,
            money: character.money,
            ability_cooldown_timer: Timer::from_seconds(character.ability_period, TimerMode::Once),
            ability_action_timer: None,
            ability_type: character.ability_type.clone(),
            movement_enabled: true,
            incoming_damage_multiplier: 1.0,
            player_index: 0,
            megablast_offset_position: character.megablast_offset_position,
        }
    }
}
impl PlayerComponent {
    pub fn from_character_with_params(
        character: &Character,
        spawn_params: &InputRestrictionsAtSpawn,
    ) -> Self {
        let mut res = Self::from(character);
        if spawn_params.forbid_special_attack_reason.is_some() {
            res.disable_special_attacks();
        }
        res
    }
    pub fn disable_special_attacks(&mut self) {
        self.ability_cooldown_timer.pause();
    }
    pub fn ability_is_enabled(&self) -> bool {
        !self.ability_cooldown_timer.paused()
    }
    pub fn enable_special_attacks(&mut self) {
        self.ability_cooldown_timer.unpause();
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum AbilityType {
    Charge(f32),    // ability duration
    MegaBlast(f32), // scale and damage multiplier
}
