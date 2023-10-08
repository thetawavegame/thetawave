//! `thetawave` background module

use std::fs;

use bevy::prelude::Commands;
use bevy::prelude::*;
use rand::{seq::IteratorRandom, Rng};
use ron::de::from_bytes;
use serde::Deserialize;
use std::ops::Range;
use thetawave_interface::{
    run::{RunDefeatType, RunEndEvent, RunOutcomeType},
    states::GameCleanup,
};
use thiserror::Error;

use crate::{
    states::{self},
    GameEnterSet,
};

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StarExplodeResource::default());
        app.insert_resource(
            from_bytes::<BackgroundsResource>(include_bytes!("../../assets/data/backgrounds.ron"))
                .unwrap(),
        );

        app.add_systems(
            OnEnter(states::AppStates::Game),
            create_background_system.in_set(GameEnterSet::BuildLevel),
        );

        app.add_systems(
            Update,
            (rotate_planet_system, on_defeat_star_explode_system)
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Parameters for procedurally generated 3D level backgrounds
#[derive(Resource, Deserialize)]
pub struct BackgroundsResource {
    /// Intensity increase rate for star explosion effect
    pub star_explode_intensity: f32,
    /// Position of the quad with the background image
    pub background_transation: Vec3,
    /// Range of x coordinates of star position
    pub star_position_x_range: Range<f32>,
    /// Range of z coordinates of star position
    pub star_position_z_range: Range<f32>,
    /// Position of the planet
    pub planet_translation: Vec3,
    /// Range of colors for the star
    pub star_color_range: Range<f32>,
    /// Width of the background quad mesh
    pub background_quad_width: f32,
    /// Height of the background quad mesh
    pub background_quad_height: f32,
    /// Alpha channel value of the background
    pub background_alpha: f32,
    /// Radius of the star's icosphere mesh
    pub star_radius: f32,
    /// Subdivisions of the star's icosphere mesh
    pub star_subdivisions: usize,
    /// Subdivisions of a fallback planet's icosphere mesh
    pub planet_subdivisions: usize,
    /// Range of rotation speeds for the planet
    pub rotation_speed_range: Range<f32>,
    /// Intensity of the point light child of the star
    pub star_light_intensity: f32,
    /// Range of the point light child of the star
    pub star_light_range: f32,
}

/// Resource to track if star explosion is happening
#[derive(Resource, Default)]
pub struct StarExplodeResource {
    pub started: bool,
}

/// Component to manage movement of planets
#[derive(Reflect, Default, Component)]
#[reflect(Component)]
pub struct PlanetComponent {
    /// Speed of rotation about the z axis
    pub rotation_speed: f32,
}

/// Component to tag star point light
#[derive(Component)]
pub struct StarLightComponent;

/// Rotate planets about their z axis
pub fn rotate_planet_system(mut query: Query<(&mut Transform, &PlanetComponent)>, time: Res<Time>) {
    for (mut transform, planet) in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(planet.rotation_speed * time.delta_seconds());
    }
}

/// Execute the exploding star effect if the game is lost through defense being destroyed
pub fn on_defeat_star_explode_system(
    mut run_end_event_reader: EventReader<RunEndEvent>,
    mut point_light_query: Query<&mut PointLight, With<StarLightComponent>>,
    mut star_explode_res: ResMut<StarExplodeResource>,
    time: Res<Time>,
    backgrounds_res: Res<BackgroundsResource>,
) {
    // Check for loss condition from defense objective
    for event in run_end_event_reader.iter() {
        if let RunOutcomeType::Defeat(RunDefeatType::DefenseDestroyed) = event.outcome {
            star_explode_res.started = true;
        }
    }

    // Update star point light intensity if star explosion active
    if star_explode_res.started {
        for mut point_light in point_light_query.iter_mut() {
            point_light.intensity *= backgrounds_res.star_explode_intensity * time.delta_seconds();
        }
    }
}

#[derive(Error, Debug)]
enum OurGetRandomAssetError {
    #[error("Path does not exist.")]
    NoPathFound,
    #[error("No files found to choose in path.")]
    NoFilesInPath,
    #[error("Invalid file name.")]
    InvalidFileName,
}

fn get_random_asset_file(path: String) -> Result<String, OurGetRandomAssetError> {
    let mut rng = rand::thread_rng();

    let read_dir = fs::read_dir(path).map_err(|_e| OurGetRandomAssetError::NoPathFound)?;
    let random_asset = read_dir
        .choose(&mut rng)
        .ok_or(OurGetRandomAssetError::NoFilesInPath)?;
    let chosen_filename = random_asset
        .map_err(|_e| OurGetRandomAssetError::InvalidFileName)?
        .path()
        .file_name()
        .ok_or(OurGetRandomAssetError::InvalidFileName)?
        .to_string_lossy()
        .to_string();

    Ok(chosen_filename)
}

/// Create a procedurally generated 3D background for a level
pub fn create_background_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut star_explode_res: ResMut<StarExplodeResource>,
    backgrounds_res: Res<BackgroundsResource>,
) {
    // reset the star explode reource
    *star_explode_res = StarExplodeResource::default();

    let mut rng = rand::thread_rng();

    // Choose random positions for the bodies
    let background_transform = Transform::from_translation(backgrounds_res.background_transation);
    let star_transform = Transform::from_xyz(
        rng.gen_range(backgrounds_res.star_position_x_range.clone()),
        0.0,
        rng.gen_range(backgrounds_res.star_position_z_range.clone()),
    );
    let planet_transform = Transform::from_translation(backgrounds_res.planet_translation);

    // Choose a random planet or fallback to a white icosphere
    {
        let mut planet_commands = commands.spawn_empty();
        planet_commands
            .insert(PlanetComponent {
                rotation_speed: rng.gen_range(backgrounds_res.rotation_speed_range.clone()),
            })
            .insert(GameCleanup)
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .insert(Name::new("Planet"));

        match get_random_asset_file("./assets/models/planets".to_string()) {
            Ok(file_name) => {
                let planet_model_handle: Handle<Scene> =
                    asset_server.load(format!("models/planets/{file_name}#Scene0"));

                planet_commands.insert(SceneBundle {
                    scene: planet_model_handle,
                    transform: planet_transform,
                    ..default()
                });
            }
            Err(_) => {
                error!("Failed to get random model from ./assets/models/planets. Using fallback model instead.");

                let maybe_icosphere = Mesh::try_from(shape::Icosphere {
                    radius: 10.0,
                    subdivisions: backgrounds_res.planet_subdivisions,
                });

                match maybe_icosphere {
                    Ok(icosphere) => {
                        planet_commands.insert(PbrBundle {
                            mesh: meshes.add(icosphere),
                            material: materials.add(StandardMaterial {
                                base_color: Color::WHITE,
                                ..default()
                            }),
                            transform: planet_transform,
                            ..default()
                        });
                    }
                    Err(e) => {
                        error!("{e}\nCould not construct icosphere for planet. No planet model will be spawned.");
                    }
                };
            }
        }
    }

    // Spawn a quad textured with a random background image
    // Create a quad mesh for the background
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        backgrounds_res.background_quad_width,
        backgrounds_res.background_quad_height,
    ))));

    // Choose a random background or fallback to a black color
    let mut background_commands = commands.spawn_empty();
    background_commands
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Space Background"))
        .insert(
            match get_random_asset_file("./assets/texture/backgrounds".to_string()) {
                Ok(file_name) => {
                    let background_texture_handle = asset_server.load(format!("texture/backgrounds/{file_name}"));

                    let material_handle = materials.add(StandardMaterial {
                        base_color_texture: Some(background_texture_handle),
                        alpha_mode: AlphaMode::Blend,
                        base_color: Color::default().with_a(backgrounds_res.background_alpha),
                        unlit: true,
                        ..default()
                    });

                    PbrBundle {
                        mesh: quad_handle,
                        material: material_handle,
                        transform: background_transform,
                        ..default()
                    }
                }
                Err(_) => {
                    error!("Failed to get random background texture from ./assets/texture/backgrounds. Using fallback material instead.");

                    let material_handle = materials.add(StandardMaterial {
                        base_color: Color::BLACK,
                        unlit: true,
                        ..default()
                    });

                    PbrBundle {
                        mesh: quad_handle,
                        material: material_handle,
                        transform: background_transform,
                        ..default()
                    }
                }
            }
        );

    // Spawn a star with a random color
    let star_color = Color::rgb_linear(
        rng.gen_range(backgrounds_res.star_color_range.clone()),
        rng.gen_range(backgrounds_res.star_color_range.clone()),
        rng.gen_range(backgrounds_res.star_color_range.clone()),
    );

    // Emissive colored star material for bloom
    let star_material = materials.add(StandardMaterial {
        emissive: star_color,
        ..default()
    });

    // Spherical star mesh
    let star_mesh = meshes.add(
        shape::Icosphere {
            radius: backgrounds_res.star_radius,
            subdivisions: backgrounds_res.star_subdivisions,
        }
        .try_into()
        .unwrap(),
    );

    // Spawn the star with a child point light of the same color
    commands
        .spawn((PbrBundle {
            mesh: star_mesh,
            material: star_material,
            transform: star_transform,
            ..default()
        },))
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Star"))
        .with_children(|parent| {
            parent
                .spawn(PointLightBundle {
                    point_light: PointLight {
                        color: star_color,
                        intensity: backgrounds_res.star_light_intensity,
                        range: backgrounds_res.star_light_range,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(StarLightComponent)
                .insert(Name::new("Star Point Light"));
        });
}
