use crate::player::PlayerComponent;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawns player into the game
pub fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    const SPRITE_SCALE: f32 = 3.0; // TODO: move to game parameters resource

    // scale collider to align with the sprite
    let collider_size_x = 6.0 * SPRITE_SCALE / rapier_config.scale; // TODO: move values game data file/resource for player
    let collider_size_y = 13.0 * SPRITE_SCALE / rapier_config.scale;

    // get player texture
    let texture_handle = asset_server.load("texture/player.png");

    // spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            //position: [collider_size_x / 2.0, collider_size_y / 2.0].into(), // may need to adjust position when detecting collisions
            shape: ColliderShape::cuboid(collider_size_x / 2.0, collider_size_y / 2.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(PlayerComponent {
            // TODO: move values into game data file/resource for player
            acceleration: [0.10, 0.14].into(),
            deceleration: [0.008, 0.012].into(),
            speed: [1.5, 2.5].into(),
        });
}
