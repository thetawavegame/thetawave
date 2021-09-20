use crate::{HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawns arena barriers
pub fn spawn_barriers_system(mut commands: Commands) {
    // spawn horizontal barriers at top and bottom of arena
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, 38.0), 96.0, 4.0);
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, -38.0), 96.0, 4.0);
    // spawn vertical barriers at right and left of arena
    spawn_barrier(&mut commands, Vec2::new(50.0, 0.0), 4.0, 72.0);
    spawn_barrier(&mut commands, Vec2::new(-50.0, 0.0), 4.0, 72.0);
}

/// Spawns an arena barrier
fn spawn_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(width / 2.0, height / 2.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(Name::new("Barrier"));
}

/// Spawns a barrier that allows for spawnables to pass
fn spawn_spawnables_pass_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(width / 2.0, height / 2.0),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(
                    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                    u32::MAX ^ SPAWNABLE_COL_GROUP_MEMBERSHIP, // filters out spawnable group
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Name::new("Spawnables-Pass Barrier"));
}
