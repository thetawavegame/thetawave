use crate::{HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct ArenaBarrierComponent;

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
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Restitution::new(1.0))
        .insert(ArenaBarrierComponent)
        .insert(Name::new("Barrier"));
}

/// Spawns a barrier that allows for spawnables to pass
fn spawn_spawnables_pass_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Transform::from_translation(position.extend(0.0)))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Restitution::new(1.0))
        .insert(CollisionGroups {
            memberships: HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
            filters: u32::MAX ^ SPAWNABLE_COL_GROUP_MEMBERSHIP,
        })
        .insert(Name::new("Spawnables-Pass Barrier"));
}
