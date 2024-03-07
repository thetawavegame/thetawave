use bevy::core::Name;
use bevy::ecs::system::{Commands, Res};
use bevy::hierarchy::{BuildChildren, ChildBuilder};
use bevy::input::gamepad::Gamepad;
use bevy::math::Vec3;
use bevy::render::color::Color;
use bevy::sprite::{Sprite, SpriteBundle};
use bevy::transform::components::Transform;
use bevy_rapier2d::dynamics::{ExternalImpulse, LockedAxes, RigidBody, Velocity};
use bevy_rapier2d::geometry::{ActiveEvents, Collider, ColliderMassProperties, Restitution};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use thetawave_interface::abilities::{AbilitiesResource, SlotOneAbilityType, SlotTwoAbilityType};
use thetawave_interface::input::{InputsResource, PlayerAction};
use thetawave_interface::player::{InputRestrictionsAtSpawn, PlayerBundle, PlayerIDComponent};
use thetawave_interface::{health::HealthComponent, player::PlayerInput, states::GameCleanup};

use crate::{
    assets,
    game::GameParametersResource,
    player::{CharactersResource, PlayersResource},
};

trait PlayerAbilityChildBuilderExt {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    );

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    );
}

impl PlayerAbilityChildBuilderExt for ChildBuilder<'_> {
    fn spawn_slot_1_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotOneAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotOneAbilityType::StandardBlast => {
                    self.spawn(abilities_res.standard_blast_ability.clone())
                }
                SlotOneAbilityType::StandardBullet => {
                    self.spawn(abilities_res.standard_bullet_ability.clone())
                }
            };
        }
    }

    fn spawn_slot_2_ability(
        &mut self,
        abilities_res: &AbilitiesResource,
        ability_type: &Option<SlotTwoAbilityType>,
    ) {
        if let Some(ability_type) = ability_type {
            match ability_type {
                SlotTwoAbilityType::Charge => self.spawn(abilities_res.charge_ability.clone()),
                SlotTwoAbilityType::MegaBlast => {
                    self.spawn(abilities_res.mega_blast_ability.clone())
                }
            };
        }
    }
}

/// Spawns player into the game
pub(super) fn spawn_players_system(
    mut commands: Commands,
    characters: Res<CharactersResource>,
    game_parameters: Res<GameParametersResource>,
    player_assets: Res<assets::PlayerAssets>,
    players_resource: Res<PlayersResource>,
    inputs_res: Res<InputsResource>,
    abilities_res: Res<AbilitiesResource>,
    spawn_params: Res<InputRestrictionsAtSpawn>,
) {
    // check if more than one player is playing
    let is_multiplayer = players_resource.player_data[1].is_some();

    for (player_id, maybe_player_data) in players_resource
        .player_data
        .iter()
        .enumerate()
        .map(|(id, pd)| (PlayerIDComponent::from(id), pd))
    {
        if let Some(player_data) = maybe_player_data {
            // choose a character
            let character = &characters.characters[&player_data.character];

            // scale collider to align with the sprite
            let collider_size_hx =
                character.collider_dimensions.x * game_parameters.sprite_scale / 2.0;
            let collider_size_hy =
                character.collider_dimensions.y * game_parameters.sprite_scale / 2.0;

            // create player component from character
            let player_bundle = PlayerBundle::from(character).with_id(player_id);

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
                            if matches!(player_id, PlayerIDComponent::One) {
                                -game_parameters.player_spawn_distance
                            } else {
                                game_parameters.player_spawn_distance
                            },
                            0.0,
                            if matches!(player_id, PlayerIDComponent::One) {
                                0.0
                            } else {
                                0.2
                            },
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
                .insert(player_bundle)
                .insert(HealthComponent::from(character))
                .insert(GameCleanup)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ExternalImpulse::default())
                .insert(Name::new("Player"))
                .with_children(|parent| {
                    parent.spawn_slot_1_ability(&abilities_res, &character.slot_1_ability);
                    parent.spawn_slot_2_ability(&abilities_res, &character.slot_2_ability);
                });

            // add colored outline to player if multiplayer
            if is_multiplayer {
                player_entity.with_children(|parent| {
                    parent
                        .spawn(SpriteBundle {
                            texture: player_assets.get_outline_asset(&character.character_type),
                            sprite: Sprite {
                                color: if matches!(player_id, PlayerIDComponent::One) {
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
