use bevy_ecs::{bundle::Bundle, component::Component};
use bevy_time::Timer;

use crate::{audio::SoundEffectType, spawnable::ProjectileType, weapon::SpreadPattern};

#[derive(Component)]
pub enum AbilitySlotIDComponent {
    One,
    Two,
}

/// Charge ability bundle for spawning entity as a child of player component
#[derive(Bundle)]
pub struct ChargeAbilityBundle {
    slot: AbilitySlotIDComponent,
    ability: ChargeAbilityComponent,
}

#[derive(Component)]
pub struct ChargeAbilityComponent {
    pub action_timer: Timer,
    pub incoming_damage_multiplier: f32,
    pub impulse: f32,
}

/// Standard weapon bundle for spawning entity as a child of player component
#[derive(Bundle)]
pub struct StandardWeaponAbilityBundle {
    slot: AbilitySlotIDComponent,
    ability: StandardWeaponAbilityComponent,
}

#[derive(Component)]
pub struct StandardWeaponAbilityComponent {
    pub ability_slot: usize,
    pub spread_pattern: SpreadPattern,
    pub damage_multiplier: f32,
    pub ammunition: ProjectileType,
    pub speed_multiplier: f32,
    pub direction: f32,
    pub despawn_time_multiplier: f32,
    pub size_multiplier: f32,
    pub sound: SoundEffectType,
}
