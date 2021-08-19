use crate::spawnable::MobComponent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct DespawnGateComponent;

pub fn spawn_despawn_gates_system(mut commands: Commands) {
    spawn_despawn_gate(&mut commands, Vec2::new(0.0, -45.0), 96.0, 4.0);
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

pub fn despawn_gates_system(
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
