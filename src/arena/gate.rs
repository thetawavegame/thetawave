use crate::spawnable::{MobComponent, MobSegmentComponent, SpawnableComponent};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};
use thetawave_interface::{objective::MobReachedBottomGateEvent, states::GameCleanup};

/// Despawn gate tag
#[derive(Component)]
pub struct DespawnGateComponent;

/// Spawn gates for despawning entities
pub fn spawn_despawn_gates_system(mut commands: Commands) {
    spawn_despawn_gate(&mut commands, Vec2::new(0.0, -500.0), 1000.0, 50.0);
}

/// Spawn a despawn gate
fn spawn_despawn_gate(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn_empty()
        .insert(TransformBundle::from_transform(
            Transform::from_translation(position.extend(0.0)),
        ))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Sensor)
        .insert(DespawnGateComponent)
        .insert(GameCleanup)
        .insert(Name::new("Despawn Gate"));
}

/// Despawn spawnables when they intersect with despawn gates
#[allow(clippy::too_many_arguments)]
pub fn despawn_gates_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    despawn_gate_query: Query<Entity, With<DespawnGateComponent>>,
    spawnable_query: Query<Entity, With<SpawnableComponent>>,
    mob_query: Query<(Entity, &MobComponent)>,
    mob_segment_query: Query<(Entity, &MobSegmentComponent)>,
    mut enemy_bottom_event: EventWriter<MobReachedBottomGateEvent>,
) {
    // loop through all collision events
    'event_loop: for collision_event in collision_events.read() {
        for despawn_gate_entity in despawn_gate_query.iter() {
            if let CollisionEvent::Started(
                collider1_entity,
                collider2_entity,
                CollisionEventFlags::SENSOR,
            ) = collision_event
            {
                // identify what is the gate entity, and what is the other entity
                let other_entity = if despawn_gate_entity == *collider1_entity {
                    collider2_entity
                } else if despawn_gate_entity == *collider2_entity {
                    collider1_entity
                } else {
                    // continue to next collision event if gate entity is not one of the entities
                    continue 'event_loop;
                };

                // verify the other entity is a spawnable
                if spawnable_query
                    .iter()
                    .any(|spawnable_entity| spawnable_entity == *other_entity)
                {
                    // despawn the spawnable entity
                    commands.entity(*other_entity).despawn_recursive();

                    // check if the other entity is a mob
                    for (mob_entity, mob_component) in mob_query.iter() {
                        if mob_entity == *other_entity {
                            // send event for mob reaching bottom of arena
                            if let Some(defense_interaction) =
                                mob_component.defense_interaction.clone()
                            {
                                enemy_bottom_event.send(MobReachedBottomGateEvent {
                                    mob_type: Some(mob_component.mob_type.clone()),
                                    mob_segment_type: None,
                                    defense_interaction,
                                });
                            }
                        }
                    }

                    // check if the other entity is a mob segment
                    for (mob_segment_entity, mob_segment_component) in mob_segment_query.iter() {
                        if mob_segment_entity == *other_entity {
                            // send event for mob segment reaching bottom of arena
                            if let Some(defense_interaction) =
                                mob_segment_component.defense_interaction.clone()
                            {
                                enemy_bottom_event.send(MobReachedBottomGateEvent {
                                    mob_type: None,
                                    mob_segment_type: Some(
                                        mob_segment_component.mob_segment_type.clone(),
                                    ),
                                    defense_interaction,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
}
