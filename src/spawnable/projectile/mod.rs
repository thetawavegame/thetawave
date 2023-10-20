use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use thetawave_interface::spawnable::ProjectileType;

use crate::{animation::AnimationData, spawnable::InitialMotion, spawnable::SpawnableBehavior};

mod behavior;
mod spawn;

pub use self::behavior::ProjectileBehavior;

use super::ColliderData;

/// Event for spawning projectiles
#[derive(Event, Clone)]
pub struct SpawnProjectileEvent {
    /// Type of projectile to spawn
    pub projectile_type: ProjectileType,
    /// Position to spawn
    pub transform: Transform,
    /// Damage of projectile
    pub damage: usize,
    /// Time until projectile despawns
    pub despawn_time: f32,
    /// Initial motion of the projectile
    pub initial_motion: InitialMotion,
    pub source: Entity,
}

/// Core component for projectiles
#[derive(Component)]
pub struct ProjectileComponent {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// Damage dealt to target
    pub damage: usize,
    /// Time the projectile has existed
    pub time_alive: f32,
    /// Entity that fired the projectile
    pub source: Entity,
}

/// Data about mob entities that can be stored in data ron file
#[derive(Deserialize)]
pub struct ProjectileData {
    /// Type of projectile
    pub projectile_type: ProjectileType,
    /// List of spawnable behaviors that are performed
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    /// List of projectile behaviors that are performed
    pub projectile_behaviors: Vec<ProjectileBehavior>,
    /// Animation (currently loops single animation in specified direction)
    pub animation: AnimationData,
    /// Z level of transform of projectile
    pub z_level: f32,
    /// Collider
    pub collider: ColliderData,
    /// If it has a contact collider
    pub is_solid: bool,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            behavior::ProjectileBehaviorPlugin,
            spawn::SpawnProjectilePlugin,
        ));
    }
}

/// Stores data about mob entities
#[derive(Resource)]
pub struct ProjectileResource {
    /// Projectile types mapped to projectile data
    pub projectiles: HashMap<ProjectileType, ProjectileData>,
}
