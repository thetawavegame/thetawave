use crate::{spawnable::Faction, states, GameUpdateSet};
use bevy::prelude::*;

mod contact;
mod instersection;

pub use self::{contact::*, instersection::*};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SortedCollisionEvent>();

        app.add_systems(
            Update,
            (
                intersection_collision_system.in_set(GameUpdateSet::IntersectionCollision),
                contact_collision_system.in_set(GameUpdateSet::ContactCollision),
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Types of collisions
#[derive(Debug, Event)]
pub enum SortedCollisionEvent {
    // Player
    PlayerToProjectileIntersection {
        player_entity: Entity,
        projectile_entity: Entity,
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
    PlayerToProjectileContact {
        player_entity: Entity,
        projectile_entity: Entity,
        projectile_faction: Faction,
        player_damage: f32,
        projectile_damage: f32,
    },

    // Mob to projectile
    MobToProjectileIntersection {
        projectile_source: Entity,
        mob_entity: Entity,
        projectile_entity: Entity,
        mob_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    MobToProjectileContact {
        projectile_source: Entity,
        mob_entity: Entity,
        projectile_entity: Entity,
        projectile_faction: Faction,
        mob_faction: Faction,
        projectile_damage: f32,
    },

    // Mob segment to projectile
    MobSegmentToProjectileIntersection {
        mob_segment_entity: Entity,
        projectile_entity: Entity,
        mob_segment_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },
    MobSegmentToProjectileContact {
        mob_segment_entity: Entity,
        projectile_entity: Entity,
        mob_segment_faction: Faction,
        projectile_faction: Faction,
        projectile_damage: f32,
    },

    // Mob to mob
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

    // Mob segment to mob segment
    MobSegmentToMobSegmentContact {
        mob_segment_entity_1: Entity,
        mob_segment_faction_1: Faction,
        mob_segment_damage_1: f32,
        mob_segment_entity_2: Entity,
        mob_segment_faction_2: Faction,
        mob_segment_damage_2: f32,
    },

    // Projectile to projectile
    ProjectileToProjectileContact {
        projectile_entity_1: Entity,
        projectile_faction_1: Faction,
        projectile_entity_2: Entity,
        projectile_faction_2: Faction,
    },

    // Mob to barrier
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
