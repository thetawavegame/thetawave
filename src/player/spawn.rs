use crate::{
    game::GameParametersResource,
    player::{CharactersResource, PlayerComponent},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Spawns player into the game
pub fn spawn_player_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    // choose a character
    let character = &characters.characters["juggernaut"];

    // scale collider to align with the sprite
    let collider_size_hx =
        character.collider_dimensions.x * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy =
        character.collider_dimensions.y * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    // get player texture
    let texture_handle = asset_server.load(format!("texture/{}", character.sprite_path).as_str());

    // spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_scale(Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            )),
            ..Default::default()
        })
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: Vec2::new(0.0, 0.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1))
        .insert(PlayerComponent::from(character));
}
