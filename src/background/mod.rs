//! `thetawave` background module

use std::fs;

use bevy::prelude::Commands;
use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng};
use ron::de::from_bytes;

use crate::{
    states::{self, GameCleanup},
    GameEnterSet,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(states::AppStates::Game),
            create_background_system.in_set(GameEnterSet::BuildLevel),
        );

        app.add_systems(
            Update,
            rotate_planet_system
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Component to manage movement of planets
#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct PlanetComponent {
    /// Speed of rotation about the z axis
    pub rotation_speed: f32,
}

/// Rotate planets about their z axis
pub fn rotate_planet_system(mut query: Query<(&mut Transform, &PlanetComponent)>) {
    for (mut transform, planet) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(planet.rotation_speed);
    }
}

/// Create background from resource
pub fn create_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // positions
    let background_transform = Transform::from_xyz(0.0, 0.0, -300.0);
    let star_transform = Transform::from_xyz(
        rng.gen_range(-90.0..-30.0),
        0.0,
        rng.gen_range(-250.0..-150.0),
    );
    let planet_transform = Transform::from_xyz(8.0, -8.0, 30.0);

    // randomly generate attributes
    let random_planet_file = fs::read_dir("./assets/models/planets")
        .unwrap()
        .choose(&mut rng)
        .unwrap()
        .unwrap()
        .path();
    let random_planet_filename = random_planet_file.file_name().unwrap().to_str().unwrap();

    let random_background_file = fs::read_dir("./assets/texture/backgrounds")
        .unwrap()
        .choose(&mut rng)
        .unwrap()
        .unwrap()
        .path();
    let random_background_filename = random_background_file
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let random_background_path = format!("texture/backgrounds/{random_background_filename}",);
    let random_planet_path = format!("models/planets/{random_planet_filename}#Scene0");
    let star_color = Color::rgb_linear(
        rng.gen_range(0.0..15.0),
        rng.gen_range(0.0..15.0),
        rng.gen_range(0.0..15.0),
    );

    //background
    let background_texture_handle = asset_server.load(random_background_path);
    let aspect = 1.0;

    let quad_width = 375.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        quad_width,
        quad_width * aspect,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(background_texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        base_color: Color::Rgba {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 0.06,
        },
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands
        .spawn(PbrBundle {
            mesh: quad_handle.clone(),
            material: material_handle,
            transform: background_transform,
            ..default()
        })
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Space Background"));

    // star
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: star_color, // 4. Put something bright in a dark environment to see the effect
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 10.0,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    commands
        .spawn((PbrBundle {
            mesh: mesh.clone(),
            material: material_emissive1,
            transform: star_transform,
            ..default()
        },))
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Star"));

    // planet
    let planet_model_handle: Handle<Scene> = asset_server.load(random_planet_path);

    commands
        .spawn(SceneBundle {
            scene: planet_model_handle,
            transform: planet_transform,
            ..default()
        })
        .insert(PlanetComponent {
            rotation_speed: rng.gen_range(0.0002..0.0006),
        })
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Planet"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                color: star_color,
                intensity: 5000000.0,
                range: 10000.0,
                ..Default::default()
            },
            transform: star_transform,
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(Name::new("Planet Light"));
}
