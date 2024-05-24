use std::{collections::VecDeque, default};

use bevy_math::Vec2;
use serde::Deserialize;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    abilities::{SlotOneAbilityType, SlotTwoAbilityType},
    health::HealthComponent,
    spawnable::SpawnPosition,
};

/// The playable character types. To a player, these will have different appearances and abilities.
#[derive(Deserialize, Clone, Debug, Hash, PartialEq, Eq, EnumIter, Default, Copy)]
pub enum CharacterType {
    #[default]
    Captain,
    Juggernaut,
}

// Stats used to give the player a rough idea of the strengths and weaknesses of the character
#[derive(EnumIter)]
pub enum CharacterStatType {
    Health,
    Damage,
    Speed,
    FireRate,
    Range,
    Size,
}

impl CharacterStatType {
    fn get_divisor(&self) -> f32 {
        match self {
            CharacterStatType::Damage => 50.0,
            CharacterStatType::Health => 160.0,
            CharacterStatType::Range => 1.0,
            CharacterStatType::FireRate => 5.0,
            CharacterStatType::Size => 30.0,
            CharacterStatType::Speed => 800.0,
        }
    }
}

impl CharacterType {
    pub fn to_vec() -> VecDeque<CharacterType> {
        CharacterType::iter().collect()
    }
}

/// Contains data necessary to create a player entity.
/// A character is chosen at the beginning of the game.
/// The base stats of the player are provided from the character.
/// Other data such as sprite sheets are also included with the character.
#[derive(Deserialize, Clone)]
pub struct Character {
    /// Name of the character
    pub name: String,
    /// Base acceleration
    pub acceleration: Vec2,
    /// Base deceleration
    pub deceleration: Vec2,
    /// Base speed
    pub speed: Vec2,
    /// Collider size (relative to the sprite size)
    pub collider_dimensions: Vec2,
    /// Density of the collider (mass of collider is proportional to its size)
    pub collider_density: f32,
    /// Character type
    pub character_type: CharacterType,
    /// Health of the player
    pub health: usize,
    /// Shields of the player
    pub shields: usize,
    /// Shields recharging rate
    pub shields_recharge_rate: f32,
    /// Distance to attract items and consumables
    pub attraction_distance: f32,
    /// Acceleration applied to items and consumables in attraction distance
    pub attraction_acceleration: f32,
    /// Amount of money character has collected
    pub money: usize,
    /// Amount of damage dealt on contact
    pub collision_damage: usize,
    /// Base damage dealt by player through weapon abilities
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
    /// Optional ability taking up the first ability slot
    pub slot_1_ability: Option<SlotOneAbilityType>,
    /// Optional ability taking up the second ability slot
    pub slot_2_ability: Option<SlotTwoAbilityType>,
    /// Multiplier for how long abilities take to be ready for use again
    pub cooldown_multiplier: f32,
}

impl From<&Character> for HealthComponent {
    fn from(character: &Character) -> Self {
        HealthComponent::new(
            character.health,
            character.shields,
            character.shields_recharge_rate,
        )
    }
}

impl Character {
    pub fn get_stat_percent(&self, stat: &CharacterStatType) -> f32 {
        100.0
            * match stat {
                CharacterStatType::Damage => {
                    (self.collision_damage as f32
                        + (self.weapon_damage as f32 * self.projectile_count as f32))
                        / stat.get_divisor()
                }
                CharacterStatType::Health => {
                    (self.health as f32 + self.shields as f32) / stat.get_divisor()
                }
                CharacterStatType::Range => self.projectile_despawn_time / stat.get_divisor(),
                CharacterStatType::FireRate => {
                    (stat.get_divisor() - self.cooldown_multiplier) / stat.get_divisor()
                }
                CharacterStatType::Size => {
                    (self.collider_dimensions.x * self.collider_dimensions.y) / stat.get_divisor()
                }
                CharacterStatType::Speed => {
                    (self.acceleration.x
                        + self.acceleration.y
                        + self.deceleration.x
                        + self.deceleration.y
                        + self.speed.x
                        + self.speed.y)
                        / stat.get_divisor()
                }
            }
    }
}
