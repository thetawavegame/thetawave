use bevy::{
    core_pipeline::{core_2d::Camera2d, core_3d::Camera3d},
    ecs::{
        component::Component,
        event::EventReader,
        query::{With, Without},
        system::Query,
    },
    log::info,
    transform::components::Transform,
};
use thetawave_interface::camera::ScreenShakeEvent;

#[derive(Component)]
pub struct ScreenShakeComponent {
    // store data like timers in here that is specific to screen shake
}

pub fn screen_shake_system(
    mut screen_shake_event_reader: EventReader<ScreenShakeEvent>,
    mut camera_2d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera2d>, Without<Camera3d>),
    >,
    mut camera_3d_query: Query<
        (&mut ScreenShakeComponent, &mut Transform),
        (With<Camera3d>, Without<Camera2d>),
    >,
) {
    for _event in screen_shake_event_reader.read() {
        info!("Screen shake event read");
        if let Ok((mut camera_2d_screen_shake, mut camera_2d_transform)) =
            camera_2d_query.get_single_mut()
        {
            info!("2d camera found");
            // Modify the transform and screen shake components for the 2d camera
        };
        if let Ok((mut camera_2d_screen_shake, mut camera_3d_transform)) =
            camera_3d_query.get_single_mut()
        {
            info!("3d camera found");
            // Modify the transform and screen shake components for the 3d camera
        };
    }
}
