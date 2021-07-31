use crate::player::PlayerComponent;
use bevy::prelude::*;
use bevy_rapier2d::{na::Vector2, prelude::*};

pub fn spawn_player_system(
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
            acceleration: [0.10, 0.14].into(),
            deceleration: [0.005, 0.010].into(),
            speed: [1.5, 2.5].into(),
        });
}
