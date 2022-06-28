use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::GameParametersResource,
    player::{CharactersResource, PlayerComponent},
};

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
    let collider_size_hx = character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy = character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // get player texture
    //let texture_handle = asset_server.load(format!("texture/{}", character.sprite_path).as_str());

    // spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            //material: materials.add(texture_handle.into()),
            texture: asset_server.load(format!("texture/{}", character.sprite_path).as_str()),
            transform: Transform::from_scale(Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                0.0,
            )),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform::from_translation(Vec3::ZERO))
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Restitution::new(1.0))
        .insert(MassProperties {
            mass: character.collider_density,
            ..Default::default()
        })
        .insert(PlayerComponent::from(character))
        .insert(Name::new("Player"));
}
