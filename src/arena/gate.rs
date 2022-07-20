use crate::{
    spawnable::{MobComponent, SpawnableComponent},
    states::{AppStateComponent, AppStates},
    SoundEffectsAudioChannel,
};
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;
use bevy_rapier2d::{prelude::*, rapier::prelude::CollisionEventFlags};

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
        .spawn()
        .insert_bundle(TransformBundle::from_transform(
            Transform::from_translation(position.extend(0.0)),
        ))
        //.insert(Transform::from_translation(position.extend(0.0)))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Sensor(true))
        .insert(DespawnGateComponent)
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new("Despawn Gate"));
}

/// Despawn spawnables when they intersect with despawn gates
pub fn despawn_gates_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    despawn_gate_query: Query<Entity, With<DespawnGateComponent>>,
    spawnable_query: Query<Entity, With<SpawnableComponent>>,
    mob_query: Query<(Entity, &MobComponent)>,
    mut enemy_bottom_event: EventWriter<EnemyReachedBottomGateEvent>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    'event_loop: for collision_event in collision_events.iter() {
        for despawn_gate_entity in despawn_gate_query.iter() {
            if let CollisionEvent::Started(
                collider1_entity,
                collider2_entity,
                CollisionEventFlags::SENSOR,
            ) = collision_event
            {
                let other_entity = if despawn_gate_entity == *collider1_entity {
                    collider2_entity
                } else if despawn_gate_entity == *collider2_entity {
                    collider1_entity
                } else {
                    continue 'event_loop;
                };

                if spawnable_query
                    .iter()
                    .any(|spawnable_entity| spawnable_entity == *other_entity)
                {
                    commands.entity(*other_entity).despawn_recursive();

                    for (mob_entity, mob_component) in mob_query.iter() {
                        if mob_entity == *other_entity {
                            enemy_bottom_event
                                .send(EnemyReachedBottomGateEvent(mob_component.defense_damage));
                            if mob_component.defense_damage > 0.0 {
                                audio_channel.play(asset_server.load("sounds/defense_damage.wav"));
                            } else if mob_component.defense_damage < -0.5 {
                                audio_channel.play(asset_server.load("sounds/defense_heal.wav"));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct EnemyReachedBottomGateEvent(pub f32);
