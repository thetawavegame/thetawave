use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Vector2, prelude::*};
use ron::de::{from_bytes, from_str};
use std::{env::current_dir, fs::read_to_string};

mod background;
mod debug;
mod game;
mod misc;
mod options;
mod player;

fn main() {
    options::generate_config_files();

    let config_path = current_dir().unwrap().join("config");

    let display_config = from_str::<options::DisplayConfig>(
        &read_to_string(config_path.join("display.ron")).unwrap(),
    )
    .unwrap();

    let mut app = App::build();

    app.insert_resource(WindowDescriptor::from(display_config))
        .insert_resource(
            from_bytes::<player::CharactersResource>(include_bytes!("../data/characters.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<game::GameParametersResource>(include_bytes!(
                "../data/game_parameters.ron"
            ))
            .unwrap(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup_game.system().label("init"))
        .add_startup_system(background::create_background_system.system().after("init"))
        .add_startup_system(misc::spawn_barrier_system.system().after("init"))
        .add_startup_system(player::spawn_player_system.system().after("init"))
        .add_system(player::player_movement_system.system())
        .add_system(options::toggle_fullscreen_system.system())
        .add_system(background::rotate_planet_system.system());

    if cfg!(debug_assertions) {
        app.add_system(debug::collider_debug_lines_system.system());
    }

    app.run();
}

fn setup_game(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, 950.0).looking_at(Vec3::ZERO, Vec3::Y),
        perspective_projection: PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });
    // setup rapier
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = game_parameters.physics_scale;
}
