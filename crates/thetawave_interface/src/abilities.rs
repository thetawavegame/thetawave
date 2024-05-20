use bevy_ecs::{bundle::Bundle, component::Component, event::Event, system::Resource};
use bevy_time::{Timer, TimerMode};
use serde::Deserialize;

use crate::{
    audio::SoundEffectType, player::PlayerIDComponent, spawnable::ProjectileType,
    weapon::SpreadPattern,
};

/// Identifier for slot one abilities
/// One for each unique ability
#[derive(Clone, Deserialize)]
pub enum SlotOneAbilityType {
    StandardBlast,
    StandardBullet,
}

/// Identifier for slot two abilities
/// One for each unique ability
#[derive(Clone, Deserialize)]
pub enum SlotTwoAbilityType {
    Charge,
    MegaBlast,
}

/// Event for triggering ability systems to fire when criteria like inputs and cooldowns are met
#[derive(Event, Debug)]
pub struct ActivateAbilityEvent {
    /// ID of the player that activated the ability
    pub player_id: PlayerIDComponent,
    /// Slot of the ability that was activated
    pub ability_slot_id: AbilitySlotIDComponent,
}

impl ActivateAbilityEvent {
    pub fn new(player_id: PlayerIDComponent, ability_slot_id: AbilitySlotIDComponent) -> Self {
        Self {
            player_id,
            ability_slot_id,
        }
    }
}

/// Stores the attributes for all abilities in the game.
/// Each ability should be directly convertible into a component bundle
#[derive(Resource, Deserialize)]
pub struct AbilitiesResource {
    /// Player charges in a direction, while reducing incoming damage
    pub charge_ability: ChargeAbilityData,
    /// Fires giant blast projectiles
    pub mega_blast_ability: StandardWeaponAbilityData,
    /// Fires standard blast projectiles
    pub standard_blast_ability: StandardWeaponAbilityData,
    /// Fires standard bullet projectiles
    pub standard_bullet_ability: StandardWeaponAbilityData,
}

/// Identifier for ability slots
/// Used for ability entities that are spawned as children of the player
#[derive(Component, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum AbilitySlotIDComponent {
    One,
    Two,
}
/// Component for tracking ability cooldowns
#[derive(Component, Deserialize, Clone)]
pub struct AbilityCooldownComponent {
    /// Stored seperately so that it can used with the player's cooldown multiplier
    /// to set the duration of the cooldown timer
    pub base_cooldown_time: f32,
    /// Tracks a cooldown for an ability
    pub cooldown_timer: Timer,
}

impl AbilityCooldownComponent {
    pub fn new(base_cooldown_time: f32) -> Self {
        Self {
            base_cooldown_time,
            cooldown_timer: Timer::from_seconds(base_cooldown_time, TimerMode::Once),
        }
    }
}

/// Charge ability bundle for spawning entity as a child of player component
#[derive(Bundle, Clone)]
pub struct ChargeAbilityBundle {
    /// Slot ID that that the ability occupies
    slot: AbilitySlotIDComponent,
    /// Tracks cooldown time
    cooldown: AbilityCooldownComponent,
    /// Core attributes of the charge ability, such as impulse, damage reduction
    ability: ChargeAbilityComponent,
}

impl From<&ChargeAbilityData> for ChargeAbilityBundle {
    fn from(data: &ChargeAbilityData) -> Self {
        Self {
            slot: data.slot,
            cooldown: AbilityCooldownComponent::new(data.base_cooldown_time),
            ability: ChargeAbilityComponent::from(data.ability),
        }
    }
}

/// Deserializable data for `ChargeAbilityBundle`
/// Stores minimum data required to instantiate
#[derive(Deserialize, Clone, Copy)]
pub struct ChargeAbilityData {
    /// Slot ID that that the ability occupies
    slot: AbilitySlotIDComponent,
    /// Base cooldown duration, before player's multiplier
    base_cooldown_time: f32,
    /// Core attributes of the charge ability, such as impulse, damage reduction
    ability: ChargeAbilityComponentData,
}

/// Stores ability values unique to the charge ability
/// Which applies an external impulse and damage reduction to the player
#[derive(Component, Deserialize, Clone)]
pub struct ChargeAbilityComponent {
    /// Tracks how long the player has been charging, stops charging when completed
    pub action_timer: Timer,
    /// Player damage reduction multiplier
    pub incoming_damage_multiplier: f32,
    /// External impulse that is applied to the player, in the input direction when used
    pub impulse: f32,
}

impl From<ChargeAbilityComponentData> for ChargeAbilityComponent {
    fn from(data: ChargeAbilityComponentData) -> Self {
        Self {
            action_timer: Timer::from_seconds(data.action_time, TimerMode::Once),
            incoming_damage_multiplier: data.incoming_damage_multiplier,
            impulse: data.impulse,
        }
    }
}

/// Deserializable data for `ChargeAbilityComponent`
/// Stores minimum data required to instantiate
#[derive(Deserialize, Clone, Copy)]
struct ChargeAbilityComponentData {
    /// How long in seconds the player charges when the ability is used
    action_time: f32,
    /// Player damage reduction multiplier
    incoming_damage_multiplier: f32,
    /// External impulse that is applied to the player, in the input direction when used
    impulse: f32,
}

/// Standard weapon bundle for spawning entity as a child of player component
#[derive(Bundle, Clone)]
pub struct StandardWeaponAbilityBundle {
    /// Slot ID that that the ability occupies
    slot: AbilitySlotIDComponent,
    /// Tracks cooldown time
    cooldown: AbilityCooldownComponent,
    /// Core attributes of the standard weapon ability, ammunition, multipliers, etc
    ability: StandardWeaponAbilityComponent,
}

impl From<&StandardWeaponAbilityData> for StandardWeaponAbilityBundle {
    fn from(data: &StandardWeaponAbilityData) -> Self {
        Self {
            slot: data.slot,
            cooldown: AbilityCooldownComponent::new(data.base_cooldown_time),
            ability: data.ability.clone(),
        }
    }
}

/// Deserializable data for `StandardWeaponAbilityBundle`
/// Stores minimum data required to instantiate
#[derive(Deserialize)]
pub struct StandardWeaponAbilityData {
    /// Slot ID that that the ability occupies
    slot: AbilitySlotIDComponent,
    /// Base cooldown duration, before player's multiplier
    base_cooldown_time: f32,
    /// Core attributes of the standard weapon ability, ammunition, multipliers, etc
    ability: StandardWeaponAbilityComponent,
}

/// Stores ability values unique to a standard weapon ability
/// This ability fires a number of projectiles based on many parameters
#[derive(Component, Deserialize, Clone)]
pub struct StandardWeaponAbilityComponent {
    /// How projectiles will spread once fired
    pub spread_pattern: SpreadPattern,
    /// Multiplied by the player's weapon damage, to get damage of the fired projectile
    pub damage_multiplier: f32,
    /// Type of projectile that is fired
    pub ammunition: ProjectileType,
    /// Multiplied by the player's projectile speed, to get speed of the fired projectile
    pub speed_multiplier: f32,
    /// Direction that the projectile is fired in radians
    pub direction: f32,
    /// Multiplied by the player's projectile despawn time, to get despawn time for the fired projectile
    pub despawn_time_multiplier: f32,
    /// Multiplied by the player's projectile size to get size of fired projectile
    pub size_multiplier: f32,
    /// Multiplied by the the player's projectile_count and rounded to nearest integer
    /// to get number of projetctiles fired
    pub count_multiplier: f32,
    /// Sound that plays when the ability is activated
    pub sound: SoundEffectType,
}
