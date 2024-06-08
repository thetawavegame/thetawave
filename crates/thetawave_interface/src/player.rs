use crate::character::{Character, CharacterType};
use crate::spawnable::SpawnPosition;
use bevy_ecs::system::Resource;
use bevy_ecs::{bundle::Bundle, prelude::Component};
use bevy_math::Vec2;
use derive_more::{Deref, DerefMut};

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
#[derive(Resource, Debug, Default)]
pub struct PlayersResource {
    /// Vec of Optional players, an index is Some if a player has joined for that slot
    pub player_data: Vec<Option<PlayerData>>,
}

/// Information about a player slot
#[derive(Debug, Clone)]
pub struct PlayerData {
    /// The character that a joined player has chosen
    pub character: CharacterType,
    /// Input method of a joined player
    pub input: PlayerInput,
}

/// Input method for a player
/// Gamepad has a usize identifier
#[derive(Clone, PartialEq, Debug, Copy)]
pub enum PlayerInput {
    Keyboard,
    Gamepad(usize),
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

/// Bundle of all player-specific components
#[derive(Bundle)]
pub struct PlayerBundle {
    movement_stats: PlayerMovementComponent,
    id: PlayerIDComponent,
    attraction: PlayerAttractionComponent,
    outgoing_damage: PlayerOutgoingDamageComponent,
    incoming_damage: PlayerIncomingDamageComponent,
    inventory: PlayerInventoryComponent,
    flag: PlayerComponent,
}

impl From<&Character> for PlayerBundle {
    fn from(character: &Character) -> Self {
        Self {
            movement_stats: character.into(),
            attraction: character.into(),
            outgoing_damage: character.into(),
            incoming_damage: PlayerIncomingDamageComponent::default(),
            inventory: character.into(),
            id: PlayerIDComponent::One,
            flag: PlayerComponent,
        }
    }
}

impl PlayerBundle {
    pub fn with_id(self, id: PlayerIDComponent) -> Self {
        Self { id, ..self }
    }
}

/// Identity of a player component, used for syncing UI
#[derive(Component, Clone, Copy, PartialEq, Debug)]
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
    /// Starting cooldown multiplier of the player. Used in calculating the the `cooldown_multiplier`
    pub base_cooldown_multiplier: f32,
    /// Multiplier for how long abilities take to be ready for use again
    pub cooldown_multiplier: f32,
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
            cooldown_multiplier: character.cooldown_multiplier,
            base_cooldown_multiplier: character.cooldown_multiplier,
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
