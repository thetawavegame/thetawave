use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use thetawave_interface::input::{InputsResource, PlayerAction};
use thetawave_interface::player::InputRestrictionsAtSpawn;
use thetawave_interface::{
    health::HealthComponent,
    player::{PlayerComponent, PlayerInput},
    states::GameCleanup,
};

use crate::{
    assets,
    game::GameParametersResource,
    player::{CharactersResource, PlayersResource},
};

/// Spawns player into the game
pub fn spawn_players_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    game_parameters: Res<GameParametersResource>,
    player_assets: Res<assets::PlayerAssets>,
    players_resource: Res<PlayersResource>,
    inputs_res: Res<InputsResource>,
    spawn_params: Res<InputRestrictionsAtSpawn>,
) {
    // check if more than one player is playing
    let is_multiplayer = players_resource.player_data[1].is_some();

    for (player_index, maybe_player_data) in players_resource.player_data.iter().enumerate() {
        if let Some(player_data) = maybe_player_data {
            // choose a character
            let character = &characters.characters[&player_data.character];

            // scale collider to align with the sprite
            let collider_size_hx =
                character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
            let collider_size_hy =
                character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

            // create player component from character
            let mut player_component =
                PlayerComponent::from_character_with_params(character, &spawn_params);
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
                .insert(InputManagerBundle::<PlayerAction> {
                    action_state: ActionState::default(),
                    input_map: match player_data.input {
                        PlayerInput::Keyboard => inputs_res.player_keyboard.clone(),
                        PlayerInput::Gamepad(id) => inputs_res
                            .player_gamepad
                            .clone()
                            .set_gamepad(Gamepad { id })
                            .build(),
                    },
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
