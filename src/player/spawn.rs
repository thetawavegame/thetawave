use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    assets,
    game::GameParametersResource,
    misc::HealthComponent,
    player::{CharactersResource, PlayerComponent, PlayersResource},
    states::GameCleanup,
};

/// Spawns player into the game
pub fn spawn_players_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    game_parameters: Res<GameParametersResource>,
    player_assets: Res<assets::PlayerAssets>,
    players_resource: Res<PlayersResource>,
) {
    // check if more than one player is playing
    let is_multiplayer = players_resource.player_characters[0].is_some()
        && players_resource.player_characters[1].is_some();

    for (player_index, player_character) in players_resource.player_characters.iter().enumerate() {
        if let Some(character_type) = player_character {
            // choose a character
            let character = &characters.characters[character_type];

            // scale collider to align with the sprite
            let collider_size_hx =
                character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
            let collider_size_hy =
                character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

            // create player component from character
            let mut player_component = PlayerComponent::from(character);
            player_component.player_index = player_index;

            // spawn the player
            let mut player_entity = commands.spawn_empty();
            player_entity
                .insert(SpriteBundle {
                    texture: player_assets.get_asset(&character.character_type),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Transform {
                    translation: if is_multiplayer {
                        Vec3::new(
                            if player_index == 0 {
                                -game_parameters.player_spawn_distance
                            } else {
                                game_parameters.player_spawn_distance
                            },
                            0.0,
                            if player_index == 0 { 0.0 } else { 0.2 },
                        )
                    } else {
                        Vec3::ZERO
                    },
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
                .insert(player_component)
                .insert(HealthComponent::from(character))
                .insert(GameCleanup)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ExternalImpulse::default())
                .insert(Name::new("Player"));

            // add colored outline to player if multiplayer
            if is_multiplayer {
                player_entity.with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: player_assets.get_outline_asset(&character.character_type),
                            sprite: Sprite {
                                color: if player_index == 0 {
                                    Color::rgb(0.7, 0.0, 0.0)
                                } else {
                                    Color::rgb(0.0, 0.0, 1.0)
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)));
                });
            }
        }
    }
}
