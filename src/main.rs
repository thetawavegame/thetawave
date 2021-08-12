use bevy::{
    ecs::entity::Entities,
    pbr::AmbientLight,
    prelude::*,
    reflect::TypeRegistry,
    render::{camera::PerspectiveProjection, pipeline::IndexFormat},
};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Vector2, prelude::*};
use ron::de::{from_bytes, from_str};
use std::{
    env::current_dir,
    fs::{read_to_string, File},
    io::Write,
};

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

    app.register_type::<Option<IndexFormat>>()
        .insert_resource(WindowDescriptor::from(display_config))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
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
        /*
        .add_startup_system(
            background::load_background_scene_system
                .system()
                .after("init"),
        )
        */
        .add_startup_system(background::create_background_system.system().after("init"))
        //.add_startup_system(save_scene_system.exclusive_system().at_end())
        .add_startup_system(misc::spawn_barrier_system.system().after("init"))
        .add_startup_system(player::spawn_player_system.system().after("init"))
        .add_system(player::player_movement_system.system())
        .add_system(options::toggle_fullscreen_system.system())
        .add_system(background::rotate_planet_system.system());
    //.add_system(print_entities.system());

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

fn print_entities(query: Query<Entity, With<background::PlanetComponent>>) {
    for entity in query.iter() {
        println!("{:?}", entity);
    }
}

fn save_scene_system(world: &mut World) {
    let earth_transform = Transform {
        translation: Vec3::new(550.0, -300.0, -775.0),
        scale: Vec3::new(450.0, 450.0, 450.0),
        rotation: Quat::from_rotation_z(0.41),
    };

    let sun_transform = Transform {
        translation: Vec3::new(-1150.0, -300.0, -2000.0),
        scale: Vec3::new(70.0, 70.0, 70.0),
        ..Default::default()
    };

    let mut scene_world = World::new();

    let asset_server = world.get_resource::<AssetServer>().unwrap();
    let earth_mesh = asset_server.load("models/earth.glb#Mesh0/Primitive0");
    let earth_mat = asset_server.load("models/earth.glb#Material0");
    let sun_mesh = asset_server.load("models/sun.glb#Mesh0/Primitive0");
    let sun_mat = asset_server.load("models/sun.glb#Material0");

    // add earth entity
    scene_world
        .spawn()
        .insert_bundle((earth_transform, GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: earth_mesh,
                material: earth_mat,
                ..Default::default()
            });
        })
        .insert(background::PlanetComponent {
            rotation_speed: 0.0002,
        });

    scene_world
        .spawn()
        .insert_bundle((sun_transform, GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: sun_mesh,
                material: sun_mat,
                ..Default::default()
            });
        })
        .insert(background::PlanetComponent {
            rotation_speed: 0.00005,
        });

    scene_world.spawn().insert_bundle(LightBundle {
        light: Light {
            color: Color::ORANGE_RED,
            intensity: 20000000.0,
            range: 10000000.0,
            ..Default::default()
        },
        transform: sun_transform,
        ..Default::default()
    });

    // The TypeRegistry resource contains information about all registered types (including
    // components). This is used to construct scenes.
    let type_registry = world.get_resource::<TypeRegistry>().unwrap();
    let scene = DynamicScene::from_world(&scene_world, &type_registry);

    let mut file = File::create("assets/scenes/earth_sun.scn.ron").unwrap();

    file.write_all(scene.serialize_ron(&type_registry).unwrap().as_bytes())
        .unwrap();

    // Scenes can be serialized like this:
    //info!("{}", scene.serialize_ron(&type_registry).unwrap());
}
