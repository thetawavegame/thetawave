use bevy::prelude::*;
use serde::Deserialize;

use crate::{animation::AnimationData, loot::ConsumableDropListType, misc::Health};

#[derive(Deserialize, Clone)]
pub enum MobSegmentType {
    HaulerCargo,
}

// additional segment of mob that is jointed to a mob
pub struct MobSegmentComponent {
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: f32,
    pub defense_damage: f32,
    pub health: Health,
    pub consumable_drops: ConsumableDropListType,
}

impl From<&MobSegmentData> for MobSegmentComponent {
    fn from(mob_segment_data: &MobSegmentData) -> Self {
        MobSegmentComponent {
            mob_segment_type: mob_segment_data.mob_segment_type.clone(),
            collision_damage: mob_segment_data.collision_damage,
            defense_damage: mob_segment_data.defense_damage,
            health: mob_segment_data.health.clone(),
            consumable_drops: mob_segment_data.consumable_drops.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct MobSegmentData {
    pub animation: AnimationData,
    pub collider_dimensions: Vec2,
    pub mob_segment_type: MobSegmentType,
    pub collision_damage: f32,
    pub defense_damage: f32,
    pub health: Health,
    pub consumable_drops: ConsumableDropListType,
    pub z_level: f32,
}
