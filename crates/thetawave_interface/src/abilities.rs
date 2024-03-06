use bevy_ecs::{bundle::Bundle, component::Component, system::Resource};
use bevy_time::Timer;
use serde::Deserialize;

use crate::{audio::SoundEffectType, spawnable::ProjectileType, weapon::SpreadPattern};

#[derive(Clone, Deserialize)]
pub enum SlotOneAbilityType {
    StandardBlast,
    StandardBullet,
}

#[derive(Clone, Deserialize)]
pub enum SlotTwoAbilityType {
    Charge,
    MegaBlast,
}

#[derive(Resource, Deserialize)]
pub struct AbilitiesResource {
    pub charge_ability: ChargeAbilityBundle,
    pub mega_blast_ability: StandardWeaponAbilityBundle,
    pub standard_blast_ability: StandardWeaponAbilityBundle,
    pub standard_bullet_ability: StandardWeaponAbilityBundle,
}

#[derive(Component, Deserialize, Clone, Copy)]
pub enum AbilitySlotIDComponent {
    One,
    Two,
}

#[derive(Component, Deserialize, Clone)]
pub struct AbilityCooldownComponent(pub Timer);

/// Charge ability bundle for spawning entity as a child of player component
#[derive(Bundle, Deserialize, Clone)]
pub struct ChargeAbilityBundle {
    slot: AbilitySlotIDComponent,
    cooldown: AbilityCooldownComponent,
    ability: ChargeAbilityComponent,
}

#[derive(Component, Deserialize, Clone)]
pub struct ChargeAbilityComponent {
    pub action_timer: Timer,
    pub incoming_damage_multiplier: f32,
    pub impulse: f32,
}

/// Standard weapon bundle for spawning entity as a child of player component
#[derive(Bundle, Deserialize, Clone)]
pub struct StandardWeaponAbilityBundle {
    slot: AbilitySlotIDComponent,
    cooldown: AbilityCooldownComponent,
    ability: StandardWeaponAbilityComponent,
}

#[derive(Component, Deserialize, Clone)]
pub struct StandardWeaponAbilityComponent {
    pub spread_pattern: SpreadPattern,
    pub damage_multiplier: f32,
    pub ammunition: ProjectileType,
    pub speed_multiplier: f32,
    pub direction: f32,
    pub despawn_time_multiplier: f32,
    pub size_multiplier: f32,
    pub count_multiplier: f32,
    pub sound: SoundEffectType,
}
