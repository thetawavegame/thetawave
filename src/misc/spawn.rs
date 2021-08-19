use crate::spawnable::MobComponent;
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::parry::query::details::contact_manifold_pfm_pfm};

use crate::{HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP};

pub struct DespawnGateComponent;

/// Spawns arena barriers
pub fn spawn_barrier_system(mut commands: Commands) {
    spawn_enemy_barrier(&mut commands, Vec2::new(0.0, 38.0), 96.0, 4.0);
    spawn_enemy_barrier(&mut commands, Vec2::new(0.0, -38.0), 96.0, 4.0);
    spawn_barrier(&mut commands, Vec2::new(50.0, 0.0), 4.0, 72.0);
    spawn_barrier(&mut commands, Vec2::new(-50.0, 0.0), 4.0, 72.0);

    //spawn despawn gate
    spawn_despawn_gate(&mut commands, Vec2::new(0.0, -45.0), 96.0, 4.0);
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
        .insert(ColliderDebugRender::with_id(0));
}

fn spawn_enemy_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
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
                    u32::MAX ^ SPAWNABLE_COL_GROUP_MEMBERSHIP,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0));
}

fn spawn_despawn_gate(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn()
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(width / 2.0, height / 2.0),
            collider_type: ColliderType::Sensor,
            position: position.into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::INTERSECTION_EVENTS,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(DespawnGateComponent)
        .insert(ColliderDebugRender::with_id(0));
}

pub fn despawn_spawnables_system(
    mut commands: Commands,
    mut intersection_events: EventReader<IntersectionEvent>,
    despawn_gate_query: Query<Entity, With<DespawnGateComponent>>,
    mob_query: Query<Entity, With<MobComponent>>,
) {
    for despawn_gate_entity in despawn_gate_query.iter() {
        for intersection_event in intersection_events.iter() {
            let collider1_entity = intersection_event.collider1.entity();
            let collider2_entity = intersection_event.collider2.entity();

            if despawn_gate_entity == collider1_entity
                && mob_query
                    .iter()
                    .any(|mob_entity| mob_entity == collider2_entity)
            {
                commands.entity(collider2_entity).despawn();
            }
        }
    }
}
