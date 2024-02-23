use bevy::{
    core_pipeline::{core_2d::Camera2d, core_3d::Camera3d},
    ecs::{
        component::Component,
        event::EventReader,
        query::{With, Without},
        system::Query,
    },
    transform::components::Transform,
};
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Res, Time};

use thetawave_interface::camera::ScreenShakeEvent;

#[derive(Component)]
pub struct ScreenShakeComponent {
    pub trauma: f32,
    pub trauma_decay: f32,
    pub shake_intensity: Vec3,
    pub running: bool,
}

pub fn add_trauma(
    mut screen_shake_event_reader: EventReader<ScreenShakeEvent>,
    mut camera_2d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera2d>, Without<Camera3d>),
    >,
) {
    for _event in screen_shake_event_reader.read() {
        if let Ok((mut screen_shake, _transform)) =
            camera_2d_query.get_single_mut()
        {
            screen_shake.trauma = (screen_shake.trauma + _event.trauma).min(1.0);
            screen_shake.running = true;
        };
    }
}

pub fn shake_screen(
    time: Res<Time>,
    mut camera_2d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera2d>, Without<Camera3d>),
    >,
) {
    for (mut screen_shake, mut transform) in camera_2d_query.iter_mut() {
        if screen_shake.trauma == 0. {
            if screen_shake.running {
                screen_shake.running = false;
                // TODO: change this so it doesn't use hard-coded values
                transform.translation = Vec3::ZERO;
                transform.rotation = Quat::IDENTITY;
            }
            continue;
        }

        transform.translation.x = screen_shake.shake_intensity.x * get_random_shake(&screen_shake);
        transform.translation.y = screen_shake.shake_intensity.y * get_random_shake(&screen_shake);
        transform.rotation.z    = screen_shake.shake_intensity.z * get_random_shake(&screen_shake);

        screen_shake.trauma = (screen_shake.trauma - screen_shake.trauma_decay * time.delta_seconds_f64() as f32).max(0.0);
    }
}

fn get_random_shake(screen_shake: &ScreenShakeComponent) -> f32 {
    screen_shake.trauma.powf(1.5) * (rand::random::<f32>() * 2. - 1.)
}