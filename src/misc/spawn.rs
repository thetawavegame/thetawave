use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawns arena barriers
pub fn spawn_barrier_system(mut commands: Commands) {
    spawn_barrier(&mut commands, 0.0, 38.0, 96.0, 4.0);
    spawn_barrier(&mut commands, 0.0, -38.0, 96.0, 4.0);
    spawn_barrier(&mut commands, 50.0, 0.0, 4.0, 72.0);
    spawn_barrier(&mut commands, -50.0, 0.0, 4.0, 72.0);
}

/// Spawns an arena barrier
fn spawn_barrier(commands: &mut Commands, x: f32, y: f32, width: f32, height: f32) {
    commands
        .spawn()
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Static,
            position: Vec2::new(x, y).into(),
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
        .insert(ColliderDebugRender::with_id(0));
}
