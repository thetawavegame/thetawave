use crate::player::{CharactersResource, PlayerComponent};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawns player into the game
pub fn spawn_player_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_config: Res<RapierConfiguration>,
) {
    const SPRITE_SCALE: f32 = 3.0; // TODO: move to game parameters resource

    let character = &characters.characters["juggernaut"];

    // scale collider to align with the sprite
    let collider_size_x = 6.0 * SPRITE_SCALE / rapier_config.scale; // TODO: move to character data file
    let collider_size_y = 13.0 * SPRITE_SCALE / rapier_config.scale; // TODO: move to character data file

    // get player texture
    let texture_handle = asset_server.load(format!("texture/{}", character.sprite_path).as_str());

    // spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0)),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            //position: [collider_size_x / 2.0, collider_size_y / 2.0].into(), // may need to adjust position when detecting collisions
            shape: ColliderShape::cuboid(collider_size_x / 2.0, collider_size_y / 2.0),
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(PlayerComponent::from(character));
}
