use bevy::prelude::Commands;
use bevy::prelude::*;

#[derive(Reflect, Default)]
#[reflect(Component)]
pub struct PlanetComponent {
    pub rotation_speed: f32,
}

pub fn rotate_planet_system(mut query: Query<(&mut Transform, &PlanetComponent)>) {
    for (mut transform, planet) in query.iter_mut() {
        //println!("{:?}", transform.translation);
        transform.rotation *= Quat::from_rotation_y(planet.rotation_speed);
    }
}

pub fn create_background_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands
        .spawn_bundle((earth_transform, GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: asset_server.load("models/earth.glb#Mesh0/Primitive0"),
                material: asset_server.load("models/earth.glb#Material0"),
                ..Default::default()
            });
        })
        .insert(PlanetComponent {
            rotation_speed: 0.0002,
        });

    commands
        .spawn_bundle((sun_transform, GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: asset_server.load("models/sun.glb#Mesh0/Primitive0"),
                material: asset_server.load("models/sun.glb#Material0"),
                ..Default::default()
            });
        })
        .insert(PlanetComponent {
            rotation_speed: 0.00005,
        });

    commands.spawn_bundle(LightBundle {
        light: Light {
            color: Color::ORANGE_RED,
            intensity: 20000000.0,
            range: 10000000.0,
            ..Default::default()
        },
        transform: sun_transform,
        ..Default::default()
    });
}

pub fn load_background_scene_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    let scene_handle: Handle<DynamicScene> = asset_server.load("scenes/earth_sun.scn.ron");
    scene_spawner.spawn_dynamic(scene_handle);
    asset_server.watch_for_changes().unwrap();
}
