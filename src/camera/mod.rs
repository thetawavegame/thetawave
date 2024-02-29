use crate::game;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::core_pipeline::bloom::BloomPrefilterSettings;
use bevy::core_pipeline::core_2d::{Camera2d, Camera2dBundle};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping};
use bevy::ecs::system::{Commands, Res};
use bevy::math::Vec3;
use bevy::render::camera::{Camera, ClearColorConfig, PerspectiveProjection, Projection};
use bevy::transform::components::Transform;
use bevy::utils::default;

use self::screen_shake::{add_trauma, shake_screen, shake_screen_on_player_damage};
use thetawave_interface::camera::ScreenShakeEvent;

mod screen_shake;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenShakeEvent>();
        app.add_systems(Startup, setup_cameras_system);
        app.add_systems(
            Update,
            (shake_screen, add_trauma, shake_screen_on_player_damage),
        );
    }
}

fn setup_cameras_system(
    mut commands: Commands,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup cameras
    // 2d camera for sprites

    let camera_2d = Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z),
        camera_2d: Camera2d,
        camera: Camera {
            order: 1,
            hdr: true,
            clear_color: ClearColorConfig::None,
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
