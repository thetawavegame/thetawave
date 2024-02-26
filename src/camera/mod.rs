use bevy::{
    core_pipeline::{
        bloom::{BloomPrefilterSettings, BloomSettings},
        clear_color::ClearColorConfig,
        tonemapping::Tonemapping,
    },
    prelude::*,
};

use thetawave_interface::camera::ScreenShakeEvent;

use crate::game;

use self::screen_shake::{add_trauma, shake_screen};

mod screen_shake;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenShakeEvent>();
        app.add_systems(Startup, setup_cameras_system);
        app.add_systems(Update, add_trauma);
        app.add_systems(Update, shake_screen);
    }
}

pub fn setup_cameras_system(
    mut commands: Commands,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup cameras
    // 2d camera for sprites

    let camera_2d = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z),
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        camera: Camera {
            order: 1,
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        ..default()
    };

    commands.spawn((
        camera_2d,
        BloomSettings {
            prefilter_settings: BloomPrefilterSettings {
                threshold: 1.0,
                threshold_softness: 0.2,
            },
            ..BloomSettings::OLD_SCHOOL
        },
        screen_shake::ScreenShakeComponent {
            trauma: 0.0,
            trauma_decay: 1.,
            shake_intensity: Vec3 {
                x: 60.,
                y: 60.,
                z: 0.1,
            },
            running: false,
        },
    ));

    // 3d camera for background objects
    let camera_3d = Camera3dBundle {
        camera: Camera {
            order: 0,
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective(PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        }),
        ..Default::default()
    };
    commands.spawn((camera_3d, BloomSettings::default()));
}
