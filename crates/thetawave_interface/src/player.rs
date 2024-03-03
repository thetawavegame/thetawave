use crate::character::{Character, CharacterType};
use bevy_ecs::system::Resource;
use bevy_ecs::{bundle::Bundle, prelude::Component};
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

#[derive(Bundle)]
pub struct PlayerBundle {
    pub movement_stats: PlayerMovementComponent,
    pub id: PlayerIDComponent,
    pub attraction: PlayerAttractionComponent,
    pub outgoing_damage: PlayerOutgoingDamageComponent,
    pub incoming_damage: PlayerIncomingDamageComponent,
    pub core: PlayerComponent, // TODO: Remove
}

impl From<&Character> for PlayerBundle {
    fn from(character: &Character) -> Self {
        Self {
            movement_stats: character.into(),
            core: character.into(),
            attraction: character.into(),
            outgoing_damage: character.into(),
            incoming_damage: PlayerIncomingDamageComponent::default(),
            id: PlayerIDComponent::One,
        }
    }
}

impl PlayerBundle {
    pub fn with_id(self, id: PlayerIDComponent) -> Self {
        Self { id, ..self }
    }
}

#[derive(Component, Clone, Copy, PartialEq)]
pub enum PlayerIDComponent {
    One,
    Two,
}

impl From<usize> for PlayerIDComponent {
    fn from(value: usize) -> Self {
        match value {
            0 => PlayerIDComponent::One,
            _ => PlayerIDComponent::Two,
        }
    }
}

impl From<PlayerIDComponent> for usize {
    fn from(value: PlayerIDComponent) -> Self {
        match value {
            PlayerIDComponent::One => 0,
            PlayerIDComponent::Two => 1,
        }
    }
}

#[derive(Component)]
pub struct PlayerMovementComponent {
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
    /// Whether the player responds to move inputs
    pub movement_enabled: bool,
}

#[derive(Component)]
pub struct PlayerAttractionComponent {
    /// Distance from which to apply acceleration to items and consumables
    pub distance: f32,
    /// Acceleration applied to items and consumables in within attraction distance
    pub acceleration: f32,
}

#[derive(Component)]
pub struct PlayerOutgoingDamageComponent {
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
}

#[derive(Component)]
pub struct PlayerIncomingDamageComponent {
    /// Multiplier for incoming damage
    pub multiplier: f32,
}

impl Default for PlayerIncomingDamageComponent {
    fn default() -> Self {
        Self { multiplier: 1.0 }
    }
}

/// Component for managing core attributes of the player
#[derive(Component, Debug, Clone)]
pub struct PlayerComponent {
    /// Amount of money character has collected
    pub money: usize,
    /// Timer for ability cooldown
    pub ability_cooldown_timer: Timer,
    /// Timer for ability action
    pub ability_action_timer: Option<Timer>,
    /// Type of ability
    pub ability_type: AbilityType,
}

impl From<&Character> for PlayerMovementComponent {
    fn from(character: &Character) -> Self {
        Self {
            acceleration: character.acceleration,
            deceleration: character.deceleration,
            speed: character.speed,
            movement_enabled: true,
        }
    }
}

impl From<&Character> for PlayerAttractionComponent {
    fn from(character: &Character) -> Self {
        Self {
            acceleration: character.attraction_acceleration,
            distance: character.attraction_distance,
        }
    }
}

impl From<&Character> for PlayerOutgoingDamageComponent {
    fn from(character: &Character) -> Self {
        Self {
            collision_damage: character.collision_damage,
        }
    }
}

impl From<&Character> for PlayerComponent {
    fn from(character: &Character) -> Self {
        PlayerComponent {
            money: character.money,
            ability_cooldown_timer: Timer::from_seconds(character.ability_period, TimerMode::Once),
            ability_action_timer: None,
            ability_type: character.ability_type.clone(),
        }
    }
}
impl PlayerComponent {
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

impl PlayerBundle {
    pub fn from_character_with_params(
        character: &Character,
        spawn_params: &InputRestrictionsAtSpawn,
    ) -> Self {
        let mut res = Self::from(character);
        if spawn_params.forbid_special_attack_reason.is_some() {
            res.core.disable_special_attacks();
        }
        res
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum AbilityType {
    Charge(f32),    // ability duration
    MegaBlast(f32), // scale and damage multiplier
}
