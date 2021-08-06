use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Vector2, prelude::*};

mod debug;
mod misc;
mod player;

fn main() {
    let mut app = App::build();

    app.insert_resource(WindowDescriptor {
        title: "Theta Wave".to_string(),
        width: 960.0,
        height: 720.0,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(DebugLinesPlugin)
    //.add_plugin(RapierRenderPlugin)
    .add_startup_system(setup_game.system().label("init"))
    .add_startup_system(misc::spawn_barrier_system.system().after("init"))
    .add_startup_system(player::spawn_player_system.system().after("init"))
    .add_system(player::player_movement_system.system());
    //.add_system(print_player_position.system())

    if cfg!(debug_assertions) {
        app.add_system(debug::collider_debug_lines_system.system());
    }

    app.run();
}

fn setup_game(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // setup rapier
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = 10.0;
}

// print position of the player
fn print_player_position(query: Query<&RigidBodyPosition, With<player::PlayerComponent>>) {
    for player_pos in query.iter() {
        info!(
            "Player position: {:?}",
            player_pos.position.translation.vector.data
        );
    }
}
