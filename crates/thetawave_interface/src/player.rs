use crate::character::{Character, CharacterType};
use crate::spawnable::SpawnPosition;
use bevy_ecs::system::Resource;
use bevy_ecs::{bundle::Bundle, prelude::Component};
use bevy_math::Vec2;
use bevy_time::{Timer, TimerMode};
use derive_more::{Deref, DerefMut};
use serde::Deserialize;

const MAX_PLAYER_ABILITIES: usize = 2;

/// Parameters for how to spawn new players. By default, the player can do anything.
#[derive(Resource, Debug, Default, Deref, DerefMut)]
pub struct InputRestrictionsAtSpawn(InputRestrictions);

/// Things the player is not allowed to do.
#[derive(Resource, Debug, Default)]
pub struct InputRestrictions {
    pub forbid_main_attack_reason: Option<String>,
    pub forbid_special_attack_reason: Option<String>,
}

/// Stores all available player slots
#[derive(Resource, Debug)]
pub struct PlayersResource {
    pub player_data: Vec<Option<PlayerData>>,
}

/// Information about a player slot
#[derive(Debug, Clone)]
pub struct PlayerData {
    pub character: CharacterType,
    pub input: PlayerInput,
}

/// Input method for a player
#[derive(Clone, PartialEq, Debug)]
pub enum PlayerInput {
    Keyboard,
    Gamepad(usize),
}

/// Defaults to all player slots being empty
impl Default for PlayersResource {
    fn default() -> Self {
        PlayersResource {
            player_data: vec![None, None, None, None],
        }
    }
}

impl PlayersResource {
    /// A method to get a vector of all used inputs
    pub fn get_used_inputs(&self) -> Vec<PlayerInput> {
        self.player_data
            .iter()
            .filter_map(|player_data| player_data.clone().map(|data| data.input))
            .collect()
    }
}

/// Component bundle of all player-specific components
#[derive(Bundle)]
pub struct PlayerBundle {
    movement_stats: PlayerMovementComponent,
    id: PlayerIDComponent,
    attraction: PlayerAttractionComponent,
    outgoing_damage: PlayerOutgoingDamageComponent,
    incoming_damage: PlayerIncomingDamageComponent,
    inventory: PlayerInventoryComponent,
    abilities: PlayerAbilitiesComponent,
    flag: PlayerComponent,
    ability_cooldowns: PlayerAbilityCooldownsComponent,
}

impl From<&Character> for PlayerBundle {
    fn from(character: &Character) -> Self {
        Self {
            movement_stats: character.into(),
            abilities: character.into(),
            attraction: character.into(),
            outgoing_damage: character.into(),
            incoming_damage: PlayerIncomingDamageComponent::default(),
            inventory: character.into(),
            id: PlayerIDComponent::One,
            flag: PlayerComponent,
            ability_cooldowns: PlayerAbilityCooldownsComponent::default(),
        }
    }
}

impl PlayerBundle {
    pub fn with_id(self, id: PlayerIDComponent) -> Self {
        Self { id, ..self }
    }
}

/// Identity of a player component, used for syncing UI
#[derive(Component, Clone, Copy, PartialEq)]
pub enum PlayerIDComponent {
    One,
    Two,
}

/// Useful for mapping an index to a PlayerIDComponent
impl From<usize> for PlayerIDComponent {
    fn from(value: usize) -> Self {
        match value {
            0 => PlayerIDComponent::One,
            _ => PlayerIDComponent::Two,
        }
    }
}

/// Useful for positioning UI
impl From<PlayerIDComponent> for usize {
    fn from(value: PlayerIDComponent) -> Self {
        match value {
            PlayerIDComponent::One => 0,
            PlayerIDComponent::Two => 1,
        }
    }
}

/// Component that stores movement properties of player
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

/// Component that stores attraction stats for player
/// Used for attracting items and consumables to the player
#[derive(Component)]
pub struct PlayerAttractionComponent {
    /// Distance from which to apply acceleration to items and consumables
    pub distance: f32,
    /// Acceleration applied to items and consumables in within attraction distance
    pub acceleration: f32,
}

/// Stores outgoing damage stats for player
#[derive(Component)]
pub struct PlayerOutgoingDamageComponent {
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Base damage dealt through weapon abilities
    pub weapon_damage: usize,
    /// Base speed of spawned weapon ability projectiles
    pub projectile_speed: f32,
    /// Spawn position of weapon ability projectiles
    pub projectile_spawn_position: SpawnPosition,
    /// Base despawn time for projectiles
    pub projectile_despawn_time: f32,
    /// Base size of projectiles
    pub projectile_size: f32,
    /// Base projectile count
    pub projectile_count: usize,
}

/// Stores stats that effect damage incoming to the player
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

/// Tracks what the player current has in inventory
/// TODO: track stats of how many of each consumable has been picked up for the run
#[derive(Component)]
pub struct PlayerInventoryComponent {
    pub money: usize,
}

/// Currently just handles the "top" ability
/// TODO: Overhaul this component for slotting any abilities in (including weapons)
#[derive(Component, Debug, Clone)]
pub struct PlayerAbilitiesComponent {
    /// Timer for ability cooldown
    pub ability_cooldown_timer: Timer,
    /// Timer for ability action
    pub ability_action_timer: Option<Timer>,
    /// Type of ability
    pub ability_type: AbilityType,
}

/// Tracks cooldowns for player abilities
#[derive(Component, Default, Debug)]
pub struct PlayerAbilityCooldownsComponent {
    // Should have timers equal to the maximum number of abilities that a player can have
    // If an ability slot is empyty then the value in that slot is none
    pub cooldowns: [Option<Timer>; MAX_PLAYER_ABILITIES],
}

/// Flag for Player Entities
#[derive(Component)]
pub struct PlayerComponent;

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
            weapon_damage: character.weapon_damage,
            projectile_speed: character.projectile_speed,
            projectile_spawn_position: character.projectile_spawn_position.clone(),
            projectile_despawn_time: character.projectile_despawn_time,
            projectile_size: character.projectile_size,
            projectile_count: character.projectile_count,
        }
    }
}

impl From<&Character> for PlayerInventoryComponent {
    fn from(character: &Character) -> Self {
        Self {
            money: character.money,
        }
    }
}

impl From<&Character> for PlayerAbilitiesComponent {
    fn from(character: &Character) -> Self {
        Self {
            ability_cooldown_timer: Timer::from_seconds(character.ability_period, TimerMode::Once),
            ability_action_timer: None,
            ability_type: character.ability_type.clone(),
        }
    }
}

impl PlayerAbilitiesComponent {
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
            res.abilities.disable_special_attacks();
        }
        res
    }
}

#[derive(Deserialize, Clone, Debug)]
pub enum AbilityType {
    Charge(f32),    // ability duration
    MegaBlast(f32), // scale and damage multiplier
}
