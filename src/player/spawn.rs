use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    game::GameParametersResource,
    player::{CharactersResource, PlayerComponent},
    states::{AppStateComponent, AppStates},
};

/// Spawns player into the game
pub fn spawn_player_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    asset_server: Res<AssetServer>,
    game_parameters: Res<GameParametersResource>,
) {
    // choose a character
    let character = &characters.characters["juggernaut"];

    // scale collider to align with the sprite
    let collider_size_hx = character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
    let collider_size_hy = character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

    // spawn the player
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load(format!("texture/{}", character.sprite_path).as_str()),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform {
            translation: Vec3::ZERO,
            scale: Vec3::new(
                game_parameters.sprite_scale,
                game_parameters.sprite_scale,
                1.0,
            ),
            ..Default::default()
        })
        .insert(Collider::cuboid(collider_size_hx, collider_size_hy))
        .insert(Velocity::default())
        .insert(Restitution::new(1.0))
        .insert(ColliderMassProperties::Density(character.collider_density))
        /*
        .insert(MassProperties {
            mass: character.collider_density,
            ..Default::default()
        })
        */
        .insert(PlayerComponent::from(character))
        .insert(AppStateComponent(AppStates::Game))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Name::new("Player"));
}
