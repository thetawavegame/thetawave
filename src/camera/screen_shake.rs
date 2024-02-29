//! Shake the screen when the player receives damange. There is some randomness in the initial
//! jolt, with the camera eventually moving back to the original location (centered)
use bevy::{
    core_pipeline::{core_2d::Camera2d, core_3d::Camera3d},
    ecs::{
        component::Component,
        event::EventReader,
        query::{With, Without},
        system::{Query, Res},
    },
    math::{Quat, Vec3},
    prelude::{Entity, EventWriter},
    time::Time,
    transform::components::Transform,
    utils::tracing::error,
};

use thetawave_interface::camera::ScreenShakeEvent;
use thetawave_interface::health::DamageDealtEvent;
use thetawave_interface::player::PlayerComponent;

/// A component to represent the camera's "current state of shake". Generally, trauma is 0.
#[derive(Component)]
pub(super) struct ScreenShakeComponent {
    /// A number from 0. to 1. representing the camera's "current amount of being damaged." It will
    /// exponentially come back to 0, after being jolted up whenever the player is hit.
    pub trauma: f32,
    /// The exponential factor with which the trauma reverts to zero.
    pub trauma_decay: f32,
    /// The camera's "propensity for shaking"; multiplies with other trauma numbers.
    pub shake_intensity: Vec3,
}

pub(super) fn add_trauma(
    mut screen_shake_event_reader: EventReader<ScreenShakeEvent>,
    mut camera_2d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera2d>, Without<Camera3d>),
    >,
) {
    for _event in screen_shake_event_reader.read() {
        if let Ok((mut screen_shake, _transform)) = camera_2d_query.get_single_mut() {
            screen_shake.trauma = (screen_shake.trauma + _event.trauma).min(1.0);
        };
    }
}

pub(super) fn shake_screen(
    time: Res<Time>,
    mut camera_2d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera2d>, Without<Camera3d>),
    >,
) {
    // There should only ever be 1 camera (for now, multiplayer shares a single screen, rather than
    // split screen.
    match camera_2d_query.get_single_mut() {
        Ok((mut screen_shake, mut transform)) => {
            if screen_shake.trauma == 0. {
                // TODO: change this so it doesn't use hard-coded values
                transform.translation = Vec3::ZERO;
                transform.rotation = Quat::IDENTITY;
            }

            transform.translation.x =
                screen_shake.shake_intensity.x * get_random_shake(&screen_shake);
            transform.translation.y =
                screen_shake.shake_intensity.y * get_random_shake(&screen_shake);
            transform.rotation.z = screen_shake.shake_intensity.z * get_random_shake(&screen_shake);

            screen_shake.trauma = (screen_shake.trauma
                - screen_shake.trauma_decay * time.delta_seconds_f64() as f32)
                .max(0.0);
        }
        Err(e) => {
            error!("Failed to shake camera on player damage. {:?}", e);
        }
    }
}

fn get_random_shake(screen_shake: &ScreenShakeComponent) -> f32 {
    screen_shake.trauma.powf(1.5) * (rand::random::<f32>() * 2. - 1.)
}

/// Trigger a screen shake whenevr the player receives nonzero damage.
pub(super) fn shake_screen_on_player_damage(
    mut damage_dealt_events: EventReader<DamageDealtEvent>,
    player_query: Query<Entity, With<PlayerComponent>>,
    mut screen_shake_event_writer: EventWriter<ScreenShakeEvent>,
) {
    for event in damage_dealt_events.read() {
        if player_query.contains(event.target) && event.damage > 0 {
            screen_shake_event_writer.send(ScreenShakeEvent { trauma: 0.23 });
        }
    }
}
