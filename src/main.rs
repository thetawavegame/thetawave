use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::*};

mod player;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Theta Wave".to_string(),
            width: 960.0,
            height: 720.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugin(RapierRenderPlugin)
        .add_startup_system(setup_game.system())
        .add_startup_system(player::spawn_player_system.system())
        .add_system(player::player_movement_system.system())
        .add_system(print_player_position.system())
        .run();
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
