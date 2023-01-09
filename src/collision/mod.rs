use crate::spawnable::Faction;
use bevy::prelude::*;

mod contact;
mod instersection;

pub use self::{contact::*, instersection::*};

/// Types of collisions
#[derive(Debug)]
pub enum SortedCollisionEvent {
    PlayerToProjectileIntersection {
        player_entity: Entity,
        projectile_entity: Entity,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    MobToProjectileIntersection {
        mob_entity: Entity,
        projectile_entity: Entity,
        mob_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    MobSegmentToProjectileIntersection {
        mob_segment_entity: Entity,
        projectile_entity: Entity,
        mob_segment_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    PlayerToConsumableIntersection {
        player_entity: Entity,
        consumable_entity: Entity,
    },
    PlayerToMobContact {
        player_entity: Entity,
        mob_entity: Entity,
        mob_faction: Faction,
        player_damage: f32,
        mob_damage: f32,
    },
    PlayerToMobSegmentContact {
        player_entity: Entity,
        mob_segment_entity: Entity,
        mob_segment_faction: Faction,
        player_damage: f32,
        mob_segment_damage: f32,
    },
    MobToMobContact {
        mob_entity_1: Entity,
        mob_faction_1: Faction,
        mob_damage_1: f32,
        mob_entity_2: Entity,
        mob_faction_2: Faction,
        mob_damage_2: f32,
    },
    MobToMobSegmentContact {
        mob_entity: Entity,
        mob_faction: Faction,
        mob_damage: f32,
        mob_segment_entity: Entity,
        mob_segment_faction: Faction,
        mob_segment_damage: f32,
    },
    MobSegmentToMobSegmentContact {
        mob_segment_entity_1: Entity,
        mob_segment_faction_1: Faction,
        mob_segment_damage_1: f32,
        mob_segment_entity_2: Entity,
        mob_segment_faction_2: Faction,
        mob_segment_damage_2: f32,
    },
    MobToBarrierContact {
        mob_entity: Entity,
        barrier_entity: Entity,
    },
}

/// Stores two colliding entities
#[derive(Clone, Copy, Debug)]
pub struct CollidingEntityPair {
    primary: Entity,
    secondary: Entity,
}
