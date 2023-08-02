//! `thetawave` background module

use std::fs;

use bevy::prelude::Commands;
use bevy::prelude::*;
use rand::{distributions::uniform::SampleRange, seq::IteratorRandom, Rng};
use ron::de::from_bytes;
use serde::Deserialize;
use std::ops::Range;

use crate::{
    run::{RunDefeatType, RunEndEvent, RunOutcomeType},
    states::{self, GameCleanup},
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

#[derive(Resource, Deserialize)]
pub struct BackgroundsResource {
    pub star_explode_intensity: f32,
    pub background_transation: Vec3,
    pub star_position_x_range: Range<f32>,
    pub star_position_z_range: Range<f32>,
    pub planet_translation: Vec3,
    pub color_range: Range<f32>,
    pub background_quad_width: f32,
    pub background_quad_height: f32,
    pub background_alpha: f32,
    pub star_radius: f32,
    pub star_subdivisions: usize,
    pub rotation_speed_range: Range<f32>,
    pub star_light_intensity: f32,
    pub star_light_range: f32,
}

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
    for event in run_end_event_reader.iter() {
        if let RunOutcomeType::Defeat(RunDefeatType::DefenseDestroyed) = event.outcome {
            star_explode_res.started = true;
        }
    }

    if star_explode_res.started {
        for mut point_light in point_light_query.iter_mut() {
            point_light.intensity *= backgrounds_res.star_explode_intensity * time.delta_seconds();
        }
    }
}

/// Create background from resource
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

    // positions
    let background_transform = Transform::from_translation(backgrounds_res.background_transation);
    let star_transform = Transform::from_xyz(
        rng.gen_range(backgrounds_res.star_position_x_range.clone()),
        0.0,
        rng.gen_range(backgrounds_res.star_position_z_range.clone()),
    );
    let planet_transform = Transform::from_translation(backgrounds_res.planet_translation);

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
        rng.gen_range(backgrounds_res.color_range.clone()),
        rng.gen_range(backgrounds_res.color_range.clone()),
        rng.gen_range(backgrounds_res.color_range.clone()),
    );

    //background
    let background_texture_handle = asset_server.load(random_background_path);

    let quad_width = 375.0;
    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        backgrounds_res.background_quad_width,
        backgrounds_res.background_quad_height,
    ))));

    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(background_texture_handle),
        alpha_mode: AlphaMode::Blend,
        base_color: Color::default().with_a(backgrounds_res.background_alpha),
        unlit: true,
        ..default()
    });

    // textured quad - normal
    commands
        .spawn(PbrBundle {
            mesh: quad_handle,
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
            radius: backgrounds_res.star_radius,
            subdivisions: backgrounds_res.star_subdivisions,
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
            rotation_speed: rng.gen_range(backgrounds_res.rotation_speed_range.clone()),
        })
        .insert(GameCleanup)
        .insert(Visibility::default())
        .insert(ComputedVisibility::default())
        .insert(Name::new("Planet"));

    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                color: star_color,
                intensity: backgrounds_res.star_light_intensity,
                range: backgrounds_res.star_light_range,
                ..Default::default()
            },
            transform: star_transform,
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StarLightComponent)
        .insert(Name::new("Star Point Light"));
}
