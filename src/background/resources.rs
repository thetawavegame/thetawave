use crate::background::PlanetComponent;
use crate::states::{AppStateComponent, AppStates};
use bevy::prelude::Commands;
use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

/// Data describing lights for planets
#[derive(Deserialize)]
pub struct LightData {
    /// Lighting color
    pub color: Color,
    /// Intensity of the light
    pub intensity: f32,
    /// Range of the light
    pub range: f32,
}

/// Data describing planets
#[derive(Deserialize)]
pub struct PlanetData {
    /// 3D position of model
    pub translation: Vec3,
    /// Scale of model
    pub scale: Vec3,
    /// Rotation of model
    pub rotation: Quat,
    /// Speed of axis rotation
    pub rotation_speed: f32,
    /// Path to mesh of model
    pub mesh_path: String,
    /// Path of material of model
    pub material_path: String,
    /// Optional light data
    pub light: Option<LightData>,
}

impl PlanetData {
    /// Spawn planet with optional light
    pub fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        // create transform
        let transform = Transform {
            translation: self.translation,
            scale: self.scale,
            rotation: self.rotation,
        };

        // spawn planet entity
        commands
            .spawn((transform, GlobalTransform::IDENTITY))
            .with_children(|parent| {
                parent.spawn(PbrBundle {
                    mesh: asset_server.load(&self.mesh_path[..]),
                    material: asset_server.load(&self.material_path[..]),
                    ..Default::default()
                });
            })
            .insert(PlanetComponent {
                rotation_speed: self.rotation_speed,
            })
            .insert(AppStateComponent(AppStates::Game))
            .insert(Visibility::default())
            .insert(ComputedVisibility::default())
            .insert(Name::new("Planet"));

        // spawn light entity
        if let Some(light_data) = &self.light {
            commands
                .spawn(PointLightBundle {
                    point_light: PointLight {
                        color: light_data.color,
                        intensity: light_data.intensity,
                        range: light_data.range,
                        ..Default::default()
                    },
                    transform,
                    ..Default::default()
                })
                .insert(AppStateComponent(AppStates::Game))
                .insert(Name::new("Planet Light"));
        }
    }
}

/// A background containing 3D models
#[derive(Deserialize)]
pub struct Background {
    /// Planet models in the background
    pub planets: Vec<PlanetData>,
}

impl Background {
    /// Spawn all of the models for the background
    pub fn spawn(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        for planet in self.planets.iter() {
            planet.spawn(commands, asset_server);
        }
    }
}

/// Resource to store 3D backgrounds of levels
// TODO: replace with loading saved scenes (if possible)
#[derive(Resource, Deserialize)]
pub struct BackgroundsResource {
    /// Names of backgrounds mapped to Background instances
    pub backgrounds: HashMap<String, Background>,
}
