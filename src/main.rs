use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

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
        .add_startup_system(setup_game.system())
        .add_startup_system(player::spawn_player_system.system())
        .add_system(player::player_movement_system.system())
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn print_keyboard_input(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

fn print_player_position(player_info: Query<(&player::PlayerComponent, &RigidBodyPosition)>) {
    for (player, rb_pos) in player_info.iter() {
        println!("{:?}", rb_pos.position.translation.vector.data);
    }
}

fn print_player_velocity(player_info: Query<(&player::PlayerComponent, &RigidBodyVelocity)>) {
    for (player, rb_vel) in player_info.iter() {
        println!("{:?}", rb_vel.linvel.data);
    }
}
