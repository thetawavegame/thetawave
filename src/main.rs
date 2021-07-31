use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::*};

/// Component for managing core attributes of the player
pub struct PlayerComponent {
    pub acceleration: f32,
    pub deceleration: f32,
    // Amount of money the player has
    //pub money: i32,
    // Amount of collision damage the player deals
    //pub collision_damage: f32,
    // All the items the player has collected
    //pub items: Vec<ItemType>,
}

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
        .add_startup_system(spawn_player.system())
        .add_system(player_movement.system())
        //.add_system(print_keyboard_input.system())
        .add_system(print_player_position.system())
        .run();
}

fn setup_game(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.gravity = Vector2::zeros();

    let sprite_size_x: f32 = 48.0;
    let sprite_size_y: f32 = 104.0;

    rapier_config.scale = 10.0;
    let collider_size_x = sprite_size_x / 2.0;
    let collider_size_y = sprite_size_y / 2.0;

    let texture_handle = asset_server.load("texture/player.png");

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            //sprite: Sprite::new(Vec2::new(sprite_size_x, sprite_size_y)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            position: Vec2::new(0.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            position: [collider_size_x / 2.0, collider_size_y / 2.0].into(),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(PlayerComponent {
            acceleration: 30.0,
            deceleration: 2.0,
        });
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    rapier_parameters: Res<RapierConfiguration>,
    mut player_info: Query<(&PlayerComponent, &mut RigidBodyVelocity)>,
) {
    for (player, mut rb_vels) in player_info.iter_mut() {
        let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
        let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
        let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
        let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;

        let mut move_delta: Vector2<f32> = [x_axis as f32, y_axis as f32].into();
        if move_delta != Vector2::zeros() {
            // Note that the RapierConfiguration::Scale factor is also used here to transform
            // the move_delta from: 'pixels/second' to 'physics_units/second'
            move_delta /= move_delta.magnitude() * rapier_parameters.scale;
        }

        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel = move_delta * player.acceleration;
    }
}

fn print_keyboard_input(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

fn print_player_position(player_info: Query<(&PlayerComponent, &RigidBodyPosition)>) {
    for (player, rb_pos) in player_info.iter() {
        println!("{:?}", rb_pos.position.translation.vector.data);
    }
}
