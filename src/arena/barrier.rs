use crate::{
    spawnable::{EffectType, InitialMotion, SpawnEffectEvent},
    states::{AppStates, GameCleanup},
    HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP, SPAWNABLE_COL_GROUP_MEMBERSHIP,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::FRAC_PI_2;

/// Tag component for arena barriers
#[derive(Component)]
pub struct ArenaBarrierComponent;

/// Spawns arena barriers
pub fn spawn_barriers_system(
    mut commands: Commands,
    mut spawn_effect: EventWriter<SpawnEffectEvent>,
) {
    // TODO: move hard coded values to data file
    // spawn horizontal barriers at top and bottom of arena
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, 360.0), 1000.0, 30.0);
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, -360.0), 1000.0, 30.0);

    // spawn vertical barriers at right and left of arena
    spawn_barrier(&mut commands, Vec2::new(500.0, 0.0), 30.0, 3000.0);
    spawn_barrier(&mut commands, Vec2::new(-500.0, 0.0), 30.0, 3000.0);

    // spawn barrier glow effect
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        position: Vec2::new(0.0, -355.0),
        scale: Vec2::new(7.25, 0.0),
        rotation: 0.0,
        initial_motion: InitialMotion::default(),
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        position: Vec2::new(0.0, 355.0),
        scale: Vec2::new(7.25, 0.0),
        rotation: 0.0,
        initial_motion: InitialMotion::default(),
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        position: Vec2::new(495.0, 0.0),
        scale: Vec2::new(7.25, 0.0),
        rotation: FRAC_PI_2,
        initial_motion: InitialMotion::default(),
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        position: Vec2::new(-495.0, 0.0),
        scale: Vec2::new(7.25, 0.0),
        rotation: FRAC_PI_2,
        initial_motion: InitialMotion::default(),
    });
}

/// Spawns an arena barrier
fn spawn_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn_empty()
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from_transform(
            Transform::from_translation(position.extend(0.0)),
        ))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Restitution::new(1.0))
        .insert(Friction::new(0.0))
        .insert(ArenaBarrierComponent)
        .insert(GameCleanup)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Barrier"));
}

/// Spawns a barrier that allows for spawnables to pass
fn spawn_spawnables_pass_barrier(commands: &mut Commands, position: Vec2, width: f32, height: f32) {
    commands
        .spawn_empty()
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from_transform(
            Transform::from_translation(position.extend(0.0)),
        ))
        .insert(Collider::cuboid(width / 2.0, height / 2.0))
        .insert(Restitution::new(1.0))
        .insert(Friction::new(0.0))
        .insert(CollisionGroups {
            memberships: HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
            filters: Group::ALL ^ SPAWNABLE_COL_GROUP_MEMBERSHIP,
        })
        .insert(GameCleanup)
        .insert(ArenaBarrierComponent)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Spawnables-Pass Barrier"));
}
