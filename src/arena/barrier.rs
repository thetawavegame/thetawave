use crate::collision::{HORIZONTAL_BARRIER_COLLIDER_GROUP, SPAWNABLE_COLLIDER_GROUP};
use crate::{game::GameParametersResource, spawnable::SpawnEffectEvent};
use bevy::prelude::{
    Commands, Component, EventWriter, Name, Quat, Res, Transform, TransformBundle, Vec2, Vec3,
};
use bevy_rapier2d::prelude::*;
use std::f32::consts::FRAC_PI_2;
use thetawave_interface::{spawnable::EffectType, states::GameCleanup};

/// Tag component for arena barriers. During the main game, there should be exactly 4 entities with
/// this component, one for each side of a rectangle.
#[derive(Component)]
pub struct ArenaBarrierComponent;

/// Spawns arena barriers arranged in a centered rectangle
pub(super) fn spawn_barriers_system(
    mut commands: Commands,
    mut spawn_effect: EventWriter<SpawnEffectEvent>,
    game_parameters: Res<GameParametersResource>,
) {
    // TODO: move hard coded values to data file
    // spawn horizontal barriers at top and bottom of arena
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, 360.0), 1000.0, 30.0);
    spawn_spawnables_pass_barrier(&mut commands, Vec2::new(0.0, -360.0), 1000.0, 30.0);

    // spawn vertical barriers at right and left of arena
    spawn_barrier(&mut commands, Vec2::new(500.0, 0.0), 30.0, 10000.0);
    spawn_barrier(&mut commands, Vec2::new(-500.0, 0.0), 30.0, 10000.0);

    // spawn horizontal barriers
    spawn_barrier(&mut commands, Vec2::new(0.0, 2250.0), 3000.0, 30.0);

    // spawn barrier glow effect
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        transform: Transform {
            translation: Vec3::new(0.0, -355.0, 1.0),
            scale: Vec3::new(10.0, game_parameters.sprite_scale, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        transform: Transform {
            translation: Vec3::new(0.0, 355.0, 1.0),
            scale: Vec3::new(10.0, game_parameters.sprite_scale, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        transform: Transform {
            translation: Vec3::new(495.0, 0.0, 1.0),
            scale: Vec3::new(7.3, game_parameters.sprite_scale, 1.0),
            rotation: Quat::from_rotation_z(FRAC_PI_2),
        },
        ..Default::default()
    });
    spawn_effect.send(SpawnEffectEvent {
        effect_type: EffectType::BarrierGlow,
        transform: Transform {
            translation: Vec3::new(-495.0, 0.0, 1.0),
            scale: Vec3::new(7.3, game_parameters.sprite_scale, 1.0),
            rotation: Quat::from_rotation_z(FRAC_PI_2),
        },
        ..Default::default()
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
            memberships: HORIZONTAL_BARRIER_COLLIDER_GROUP,
            filters: Group::ALL ^ SPAWNABLE_COLLIDER_GROUP,
        })
        .insert(GameCleanup)
        .insert(ArenaBarrierComponent)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Spawnables-Pass Barrier"));
}
