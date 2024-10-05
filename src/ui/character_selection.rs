//! Systems to spawn and style the character selection screen, where each player picks a character
//! //! from one of a few options, and possibly enables/diables the tutorial.

use super::button::{
    ButtonActionComponent, ButtonActionEvent, ButtonActionType, UiButtonChildBuilderExt,
};
use crate::{game::GameParametersResource, player::CharactersResource};
use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Handle},
    color::{palettes::css::GOLD, Alpha, Color, Srgba},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, Local, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children, DespawnRecursiveExt},
    input::gamepad::{Gamepad, GamepadButtonChangedEvent},
    prelude::{in_state, NextState, OnEnter},
    text::{Font, Text, TextStyle},
    ui::{
        node_bundles::{ImageBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignContent, AlignItems, AlignSelf, FlexDirection, Interaction, JustifyContent, Style,
        UiImage, UiRect, Val,
    },
    utils::default,
};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use std::collections::VecDeque;
use strum::IntoEnumIterator;
use thetawave_assets::{PlayerAssets, UiAssets};
use thetawave_interface::{
    abilities::AbilityDescriptionsResource,
    character::{Character, CharacterStatType},
    input::{InputsResource, MainMenuExplorer, MenuAction, MenuExplorer},
    states::{self, AppStates},
};
use thetawave_interface::{
    character::CharacterType,
    character_selection::PlayerJoinEvent,
    player::{PlayerData, PlayerInput, PlayersResource},
    states::CharacterSelectionCleanup,
};

pub(super) struct CharacterSelectionPlugin;
impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerJoinEvent>();

        app.add_systems(
            Update,
            (
                player_join_system,
                mouse_click_input_system,
                keyboard_and_gamepad_input_system,
                update_ui_system,
                carousel_ui_system,
                init_carousel_ui_system,
                player_ready_system,
                check_players_ready_system,
            )
                .run_if(in_state(AppStates::CharacterSelection)),
        );

        app.add_systems(
            OnEnter(states::AppStates::CharacterSelection),
            setup_character_selection_system,
        );
    }
}

/// Component for tagging a ui node entity as the center
/// of a player's character selection ui.
///
/// u8 value represents a player index
#[derive(Component, Debug)]
struct CharacterSelectionCenter(u8);

/// Component for tagging a ui node entity as the right
/// side of a player's character selection ui.
///
/// u8 value represents a player index.
#[derive(Component)]
struct CharacterSelectionRight(u8);

/// Component for tagging a ui node entity as the left
/// side of a player's character selection ui.
///
/// u8 value represents a player index.
#[derive(Component)]
struct CharacterSelectionLeft(u8);

/// Component representing a carousel of playable characters
#[derive(Component)]
struct CharacterCarousel {
    player_idx: u8,
    characters: VecDeque<CharacterType>,
}

#[derive(Component)]
struct CharacterDescription(u8);

/// Tag component for character info ui entity
///
/// The entity containing this component should be the parent of
/// the `CharacteAbilityDescriptions` and `CharacterStatsDescriptions`
#[derive(Component)]
struct CharacterInfo;

/// Tag component for character name ui entity
///
/// The entity containing this component is a text ui
/// node containing the name of the selected character
#[derive(Component)]
struct CharacterName;

/// Tag component for ability description ui entity
///
/// This ui node contains all of the ability description rows
#[derive(Component)]
struct CharacterAbilityDescriptions;

/// Tag component for node entity that is a parent to the
/// `CharacterInfo` and `CharacterName` entities
#[derive(Component)]
struct CharacterStatsDescriptions;

/// Component for tagging a visible character option
/// in the carousel.
///
/// u8 value represents a player index.
#[derive(Component)]
struct CharacterCarouselSlot(u8);

/// Component for tracking if a player has toggled the ready button
#[derive(Component)]
struct PlayerReadyNode {
    player_idx: u8,
    is_ready: bool,
}

impl CharacterCarousel {
    /// Creates a new `CharacterCarousel` for a given player index.
    fn new(player_idx: u8) -> Self {
        CharacterCarousel {
            player_idx,
            characters: CharacterType::iter().collect(),
        }
    }

    /// Rotates the character list to the right.
    ///
    /// This function moves the last character in the list to the front.
    fn rotate_right(&mut self) {
        if let Some(last_element) = self.characters.pop_back() {
            self.characters.push_front(last_element);
        }
    }

    /// Rotates the character list to the left.
    ///
    /// This function moves the first character in the list to the back.
    fn rotate_left(&mut self) {
        if let Some(first_element) = self.characters.pop_front() {
            self.characters.push_back(first_element);
        }
    }

    /// Gets the three visible characters in the carousel.
    fn get_visible_characters(&self) -> Option<[CharacterType; 3]> {
        if let (Some(left), Some(middle), Some(right)) = (
            self.characters.back(),
            self.characters.front(),
            self.characters.get(1),
        ) {
            Some([*left, *middle, *right])
        } else {
            None
        }
    }
}

/// Extension trait for `CharacterStatType` to provide additional methods.
trait CharacterStatTypeExt {
    /// Returns the divisor associated with a particular `CharacterStatType`.
    /// This is an arbitrary large value representing 100% skill in that stat.
    fn get_divisor(&self) -> f32;
}

impl CharacterStatTypeExt for CharacterStatType {
    fn get_divisor(&self) -> f32 {
        match self {
            CharacterStatType::Damage => 50.0,
            CharacterStatType::Health => 160.0,
            CharacterStatType::Range => 1.0,
            CharacterStatType::FireRate => 5.0,
            CharacterStatType::Size => 30.0,
            CharacterStatType::Speed => 800.0,
        }
    }
}

/// Extension trait for `Character` to provide additional methods.
trait CharacterExt {
    /// Calculates the percentage value of a given character stat.
    ///
    /// The percentage is calculated based on the specific formula for each `CharacterStatType`.
    /// The result is capped at a maximum of 100.0.
    fn get_stat_percent(&self, stat: &CharacterStatType) -> f32;
}

impl CharacterExt for Character {
    fn get_stat_percent(&self, stat: &CharacterStatType) -> f32 {
        // Calculate the percentage value for each stat type.
        100.0
            * match stat {
                CharacterStatType::Damage => {
                    (self.collision_damage as f32
                        + (self.weapon_damage as f32 * self.projectile_count as f32))
                        / stat.get_divisor()
                }
                CharacterStatType::Health => {
                    (self.health as f32 + self.shields as f32) / stat.get_divisor()
                }
                CharacterStatType::Range => self.projectile_despawn_time / stat.get_divisor(),
                CharacterStatType::FireRate => {
                    (stat.get_divisor() - self.cooldown_multiplier) / stat.get_divisor()
                }
                CharacterStatType::Size => {
                    (self.collider_dimensions.x * self.collider_dimensions.y) / stat.get_divisor()
                }
                CharacterStatType::Speed => {
                    (self.acceleration.x
                        + self.acceleration.y
                        + self.deceleration.x
                        + self.deceleration.y
                        + self.speed.x
                        + self.speed.y)
                        / stat.get_divisor()
                }
            }
            .min(100.0)
    }
}

trait UiPlayerJoinChildBuilderExt {
    fn spawn_player_join_row(&mut self, ui_assets: &UiAssets, font: Handle<Font>, players: Vec<u8>);
    fn spawn_ability_descriptions(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        character: &Character,
        abilities_desc_res: &AbilityDescriptionsResource,
    );
    fn spawn_stats(&mut self, ui_assets: &UiAssets, character: &Character);
}

impl UiPlayerJoinChildBuilderExt for ChildBuilder<'_> {
    fn spawn_stats(&mut self, ui_assets: &UiAssets, character: &Character) {
        // Iterate over each character stat type
        for stat in CharacterStatType::iter() {
            // Spawn a node for the stat bar container
            self.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(12.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Spawn an image for the stat icon
                parent.spawn(ImageBundle {
                    image: ui_assets.get_stat_icon(&stat).into(),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

                // Spawn a node for the stat bar background
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            width: Val::Percent(100.0),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // Spawn a node to represent the stat percentage
                        parent.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(character.get_stat_percent(&stat)),
                                height: Val::Percent(50.0),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: Srgba::WHITE.into(),
                            ..default()
                        });
                    });
            });
        }
    }

    fn spawn_ability_descriptions(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        character: &Character,
        abilities_desc_res: &AbilityDescriptionsResource,
    ) {
        // Check if the character has a slot 1 ability
        if let Some(slot_1_ability_type) = &character.slot_1_ability {
            // Spawn a node for the slot 1 ability description container
            self.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    align_content: AlignContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Spawn an image for the ability slot
                parent
                    .spawn(ImageBundle {
                        image: ui_assets.get_ability_slot_image(false).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Spawn an image for the slot 1 ability
                        parent.spawn(ImageBundle {
                            image: ui_assets
                                .get_slot_1_ability_image(slot_1_ability_type)
                                .into(),
                            ..default()
                        });
                    });

                // Check if there is a description for the slot 1 ability
                if let Some(ability_desc) = abilities_desc_res.slot_one.get(slot_1_ability_type) {
                    // Spawn text for the slot 1 ability description
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            ability_desc,
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        ),
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(5.0),
                                right: Val::Px(5.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    });
                }
            });
        }

        // Check if the character has a slot 2 ability
        if let Some(slot_2_ability_type) = &character.slot_2_ability {
            // Spawn a node for the slot 2 ability description container
            self.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(25.0),
                    align_content: AlignContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // Spawn an image for the ability slot
                parent
                    .spawn(ImageBundle {
                        image: ui_assets.get_ability_slot_image(false).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Spawn an image for the slot 2 ability
                        parent.spawn(ImageBundle {
                            image: ui_assets
                                .get_slot_2_ability_image(slot_2_ability_type)
                                .into(),
                            ..default()
                        });
                    });

                // Check if there is a description for the slot 2 ability
                if let Some(ability_desc) = abilities_desc_res.slot_two.get(slot_2_ability_type) {
                    // Spawn text for the slot 2 ability description
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            ability_desc,
                            TextStyle {
                                font: font.clone(),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        ),
                        style: Style {
                            margin: UiRect {
                                left: Val::Px(5.0),
                                right: Val::Px(5.0),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    });
                }
            });
        }
    }

    fn spawn_player_join_row(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        players: Vec<u8>,
    ) {
        // Spawn a node for the player join row container
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(50.0),
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            // Iterate over each player index to create join UI elements
            for player_idx in players {
                // Spawn a container for each player's join UI
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(49.0),
                            max_height: Val::Percent(100.0),
                            min_height: Val::Percent(90.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Row,
                            margin: UiRect {
                                left: Val::Vw(0.0),
                                right: Val::Vw(0.5),
                                top: Val::Vh(0.0),
                                bottom: Val::Vh(0.5),
                            },
                            ..Default::default()
                        },
                        background_color: Color::BLACK.with_alpha(0.6).into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // Left side of player join UI
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(18.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(CharacterSelectionLeft(player_idx));

                        // Center of player join UI
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    flex_direction: FlexDirection::Column,
                                    width: Val::Percent(64.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                // Center container for join button and readiness indicator
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(80.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(CharacterSelectionCenter(player_idx))
                                    .with_children(|parent| {
                                        // Spawn join button for the first player
                                        if player_idx == 0 {
                                            parent.spawn_button(
                                                ui_assets,
                                                font.clone(),
                                                ButtonActionComponent::from(
                                                    ButtonActionType::CharacterSelectJoin,
                                                ),
                                                None,
                                            );
                                        }
                                    });

                                // Readiness indicator for the player
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(20.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(PlayerReadyNode {
                                        player_idx,
                                        is_ready: false,
                                    });
                            });

                        // Right side of player join UI
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(18.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(CharacterSelectionRight(player_idx));
                    });
            }
        });
    }
}

/// Setup the character selection UI
///
/// This function initializes the UI for character selection by spawning the main UI node
/// and creating rows for player join elements based on the maximum number of players.
fn setup_character_selection_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_params_res: Res<GameParametersResource>,
    ui_assets: Res<UiAssets>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // Main node containing all character selection UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect {
                    left: Val::Vw(0.5),
                    right: Val::Vw(0.5),
                    top: Val::Vh(0.5),
                    bottom: Val::Vh(0.5),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CharacterSelectionCleanup)
        .with_children(|parent| {
            // Create vectors of player indices to use for spawning player join rows
            let mut top_row_player_idxs = vec![];
            let mut bottom_row_player_idxs = vec![];

            // Distribute player indices between top and bottom rows
            for i in 0..game_params_res.get_max_players() {
                if (top_row_player_idxs.len() as u8) <= 1 {
                    top_row_player_idxs.push(i);
                } else {
                    bottom_row_player_idxs.push(i)
                }
            }

            // Spawn player join row for the top row players if not empty
            if !top_row_player_idxs.is_empty() {
                parent.spawn_player_join_row(&ui_assets, font.clone(), top_row_player_idxs);
            }

            // Spawn player join row for the bottom row players if not empty
            if !bottom_row_player_idxs.is_empty() {
                parent.spawn_player_join_row(&ui_assets, font.clone(), bottom_row_player_idxs);
            }
        });
}

/// Handles player joining events in the game.
///
/// This function detects player join actions through keyboard, gamepad, or mouse inputs,
/// updates the players resource, and sends appropriate events.
fn player_join_system(
    button_mouse_movements: Query<(&ButtonActionComponent, &Interaction, Entity), With<Button>>,
    menu_explorer_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    mut mouse_interaction: Local<Interaction>,
    game_params_res: Res<GameParametersResource>,
    mut players_resource: ResMut<PlayersResource>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut player_join_event: EventWriter<PlayerJoinEvent>,
) {
    // Check if the join button was pressed
    if let Some(button) = button_mouse_movements.iter().find(|(button_action, _, _)| {
        matches!(button_action.action, ButtonActionType::CharacterSelectJoin)
    }) {
        // Get the list of currently used inputs
        let used_inputs = players_resource.get_used_inputs();

        // Check if the maximum number of players have already joined
        if used_inputs.len() < game_params_res.get_max_players() as usize {
            // Detect if a player is joining through a keyboard button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(action) => {
                    if action
                        .get_just_released()
                        .iter()
                        .any(|action_| match action_ {
                            MenuAction::JoinKeyboard => {
                                !used_inputs.contains(&PlayerInput::Keyboard)
                            }
                            _ => false,
                        })
                    {
                        Some(PlayerInput::Keyboard)
                    } else {
                        None
                    }
                }
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Detect if a player is joining through a gamepad button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(action) => {
                    if let Some(gamepad_event) = gamepad_events.read().next() {
                        if action
                            .get_just_released()
                            .iter()
                            .any(|action| match action {
                                MenuAction::JoinGamepad => !used_inputs
                                    .contains(&PlayerInput::Gamepad(gamepad_event.gamepad.id)),
                                _ => false,
                            })
                        {
                            Some(PlayerInput::Gamepad(gamepad_event.gamepad.id))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Detect if a player is joining through a mouse button release
            if let Some(player_input) = match button.1 {
                // Check if mouse interaction changed from Pressed to Hovered
                // which means the player just released the mouse button over the UI button
                Interaction::Hovered => match *mouse_interaction {
                    Interaction::Pressed => {
                        if !used_inputs.contains(&PlayerInput::Keyboard) {
                            Some(PlayerInput::Keyboard)
                        } else {
                            None
                        }
                    }
                    _ => None,
                },
                _ => None,
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: CharacterType::default(),
                    input: player_input,
                }));

                // Send player join event and button action event
                button_event_writer.send(ButtonActionEvent::from(
                    ButtonActionType::CharacterSelectJoin,
                ));
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Track the current mouse interaction in a local variable
            *mouse_interaction = *button.1;
        }
    }
}

/// Updates the UI based on player join events.
///
/// This function handles the updating of the UI when a player joins by modifying
/// character selection elements, removing the join button from the previous selection,
/// and adding it to the next available slot. It also sets up the character selection carousel,
/// arrow buttons, and ready button.
fn update_ui_system(
    mut commands: Commands,
    mut player_join_event: EventReader<PlayerJoinEvent>,
    asset_server: Res<AssetServer>,
    character_selection_center: Query<(&CharacterSelectionCenter, Entity)>,
    character_selection_right: Query<(&CharacterSelectionRight, Entity)>,
    character_selection_left: Query<(&CharacterSelectionLeft, Entity)>,
    player_ready: Query<(&PlayerReadyNode, Entity)>,
    buttons: Query<(&ButtonActionComponent, Entity), With<Button>>,
    ui_assets: Res<UiAssets>,
    inputs_res: Res<InputsResource>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // Read all player join events
    for PlayerJoinEvent { player_idx, input } in player_join_event.read() {
        if let Some((_, button_entity)) = buttons
            .iter()
            .find(|(action, _)| matches!(action.action, ButtonActionType::CharacterSelectJoin))
        {
            // Get center UI for the current player slot and the next player slot
            let prev_character_selection_ui = character_selection_center
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let current_character_selection_ui = character_selection_center
                .iter()
                .find(|x| x.0 .0 == player_idx + 1);

            // Remove the join button from the previous character selection
            if let Some((_, entity)) = prev_character_selection_ui {
                commands.entity(entity).remove_children(&[button_entity]);

                // Spawn a character selection carousel
                commands.entity(entity).with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(20.0),
                                margin: UiRect::all(Val::Px(15.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CharacterCarousel::new(*player_idx));

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(94.0),
                                height: Val::Percent(80.0),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(3.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CharacterDescription(*player_idx))
                        .with_children(|parent| {
                            // Spawn character name text
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::Srgba(GOLD),
                                        },
                                    ),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(10.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(CharacterName);

                            // Spawn container for character info
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(90.0),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(CharacterInfo)
                                .with_children(|parent| {
                                    // Spawn container for ability descriptions
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width: Val::Percent(75.0),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(CharacterAbilityDescriptions);

                                    // Spawn container for stat descriptions
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                height: Val::Percent(100.0),
                                                width: Val::Percent(25.0),
                                                flex_direction: FlexDirection::Column,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(CharacterStatsDescriptions);
                                });
                        });
                });
            }

            // Add the join button to the new character selection
            if let Some((_, entity)) = current_character_selection_ui {
                commands.entity(entity).add_child(button_entity);
            } else {
                // If entity was not found for new character selection, despawn the button
                // This means all of the available player slots have been used up
                commands.entity(button_entity).despawn_recursive();
            }

            // Spawn right and left arrow buttons for the previous character selection
            let prev_character_selection_right_arrow_ui = character_selection_right
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let prev_character_selection_left_arrow_ui = character_selection_left
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let player_ready_node = player_ready.iter().find(|x| x.0.player_idx == *player_idx);

            if let Some((_, entity)) = prev_character_selection_right_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::from(ButtonActionType::CharacterSelectRight(
                            *player_idx,
                        )),
                        None,
                    )
                });
            };

            if let Some((_, entity)) = prev_character_selection_left_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::from(ButtonActionType::CharacterSelectLeft(
                            *player_idx,
                        )),
                        None,
                    )
                });
            };

            if let Some((_, entity)) = player_ready_node {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::from(ButtonActionType::CharacterSelectReady(
                            *player_idx,
                        )),
                        Some(input),
                    );
                });
            }

            // Spawn a menu explorer with the new player index
            let mut input_map = inputs_res.menu.clone();

            if let PlayerInput::Gamepad(id) = *input {
                input_map.set_gamepad(Gamepad { id });
            }

            commands
                .spawn(InputManagerBundle::<MenuAction> {
                    action_state: ActionState::default(),
                    input_map,
                })
                .insert(MenuExplorer(*player_idx))
                .insert(CharacterSelectionCleanup);
        }
    }
}

/// System to handle mouse click inputs for character selection in a game.
///
/// This function checks for player inputs using the keyboard and processes
/// mouse interactions for character selection buttons (left, right, ready).
/// It tracks the interaction states and sends events when relevant buttons are released.
fn mouse_click_input_system(
    button_mouse_movements: Query<(&ButtonActionComponent, &Interaction), With<Button>>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    mut stored_right_mouse_interaction: Local<[Interaction; 4]>,
    mut stored_left_mouse_interaction: Local<[Interaction; 4]>,
    mut stored_player_ready_interaction: Local<[Interaction; 4]>,
    players_resource: Res<PlayersResource>,
) {
    // Check if there is a player using a keyboard input
    if let Some(keyboard_idx) = players_resource
        .get_used_inputs()
        .iter()
        .enumerate()
        .find_map(|(idx, input)| {
            if matches!(input, PlayerInput::Keyboard) {
                Some(idx as u8)
            } else {
                None
            }
        })
    {
        // Filter and iterate over button actions that match the keyboard player's index
        for (button_action, mouse_interaction) in
            button_mouse_movements.iter().filter(|(button_action, _)| {
                matches!(
                    button_action.action,
                    ButtonActionType::CharacterSelectLeft(i) |
                    ButtonActionType::CharacterSelectRight(i) |
                    ButtonActionType::CharacterSelectReady(i) if i == keyboard_idx
                )
            })
        {
            // Determine the appropriate stored interaction based on the button action
            let stored_interaction = match button_action.action {
                ButtonActionType::CharacterSelectReady(i) => {
                    &mut stored_player_ready_interaction[i as usize]
                }
                ButtonActionType::CharacterSelectRight(i) => {
                    &mut stored_right_mouse_interaction[i as usize]
                }
                ButtonActionType::CharacterSelectLeft(i) => {
                    &mut stored_left_mouse_interaction[i as usize]
                }
                _ => continue,
            };

            // Check if the button was released (hovered after being pressed)
            let button_released = match mouse_interaction {
                Interaction::Hovered => matches!(*stored_interaction, Interaction::Pressed),
                _ => false,
            };

            // Update the stored interaction state
            *stored_interaction = *mouse_interaction;

            // Send event if the button was released
            if button_released {
                button_event_writer.send(ButtonActionEvent::from(button_action.action));
            }
        }
    }
}

/// System to handle keyboard and gamepad inputs for character selection in a game.
///
/// This function processes the input actions from gamepads and keyboards,
/// and sends the appropriate button action events based on the player's interactions.
fn keyboard_and_gamepad_input_system(
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    menu_input_query: Query<(&ActionState<MenuAction>, &MenuExplorer)>,
    players_resource: Res<PlayersResource>,
) {
    // Collect the indices of gamepad inputs into a vector
    let gamepad_idxs: Vec<u8> = players_resource
        .get_used_inputs()
        .iter()
        .enumerate()
        .filter_map(|(idx, input)| {
            if matches!(input, PlayerInput::Gamepad(_)) {
                Some(idx as u8)
            } else {
                None
            }
        })
        .collect();

    // Iterate over each player's action state and menu explorer
    for (action_state, MenuExplorer(player_idx)) in menu_input_query.iter() {
        // Iterate over each action that was just released
        for action in action_state.get_just_released().iter() {
            // Check if the player index is in the gamepad indices
            if gamepad_idxs.contains(player_idx) {
                // Match and send the appropriate event for gamepad actions
                match action {
                    MenuAction::PlayerReadyGamepad => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectReady(*player_idx),
                        ));
                    }
                    MenuAction::NavigateLeftGamepad => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectLeft(*player_idx),
                        ));
                    }
                    MenuAction::NavigateRightGamepad => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectRight(*player_idx),
                        ));
                    }
                    _ => {}
                }
            } else {
                // Match and send the appropriate event for keyboard actions
                match action {
                    MenuAction::PlayerReadyKeyboard => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectReady(*player_idx),
                        ));
                    }
                    MenuAction::NavigateLeftKeyboard => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectLeft(*player_idx),
                        ));
                    }
                    MenuAction::NavigateRightKeyboard => {
                        button_event_writer.send(ButtonActionEvent::from(
                            ButtonActionType::CharacterSelectRight(*player_idx),
                        ));
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Initializes the character carousel UI elements.
///
/// This function sets up the initial characters in the carousel for each player,
/// updates character descriptions, abilities, and stats, and sets the player's
/// selected character in the players resource.
fn init_carousel_ui_system(
    mut commands: Commands,
    character_carousels: Query<(Entity, &CharacterCarousel), Without<Children>>,
    player_assets: Res<PlayerAssets>,
    character_descriptions: Query<(&CharacterDescription, &Children)>,
    characters_res: Res<CharactersResource>,
    mut character_names: Query<&mut Text, With<CharacterName>>,
    character_info: Query<&Children, With<CharacterInfo>>,
    character_abilities: Query<Entity, With<CharacterAbilityDescriptions>>,
    character_stats: Query<Entity, With<CharacterStatsDescriptions>>,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
    mut players_res: ResMut<PlayersResource>,
    abilities_desc_res: Res<AbilityDescriptionsResource>,
) {
    // Load the font for UI text elements
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // Iterate over each character carousel entity
    for (carousel_entity, carousel) in character_carousels.iter() {
        let carousel_player_idx = carousel.player_idx;
        if let Some(visible_characters) = carousel.get_visible_characters() {
            // Spawn initial characters as children of the carousel
            commands.entity(carousel_entity).with_children(|parent| {
                parent
                    .spawn(ImageBundle {
                        image: UiImage::new(player_assets.get_asset(&visible_characters[0]))
                            .with_color(Color::srgba(0.60, 0.60, 0.60, 0.60)),
                        style: Style {
                            height: Val::Percent(80.0),
                            margin: UiRect {
                                left: Val::Vw(0.5),
                                right: Val::Vw(0.5),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(CharacterCarouselSlot(0));

                parent
                    .spawn(ImageBundle {
                        image: player_assets.get_asset(&visible_characters[1]).into(),
                        style: Style {
                            height: Val::Percent(100.0),
                            margin: UiRect {
                                left: Val::Vw(0.5),
                                right: Val::Vw(0.5),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(CharacterCarouselSlot(1));

                parent
                    .spawn(ImageBundle {
                        image: UiImage::new(player_assets.get_asset(&visible_characters[2]))
                            .with_color(Color::srgba(0.60, 0.60, 0.60, 0.60)),
                        style: Style {
                            height: Val::Percent(80.0),
                            margin: UiRect {
                                left: Val::Vw(0.5),
                                right: Val::Vw(0.5),
                                ..default()
                            },
                            ..default()
                        },
                        ..default()
                    })
                    .insert(CharacterCarouselSlot(2));
            });

            // Set the character description to the middle character
            if let Some(character) = characters_res.characters.get(&visible_characters[1]) {
                if let Some(char_desc_children) =
                    character_descriptions
                        .iter()
                        .find_map(|(character_description, children)| {
                            if character_description.0 == carousel_player_idx {
                                Some(children)
                            } else {
                                None
                            }
                        })
                {
                    for char_desc_child in char_desc_children.iter() {
                        // Update the character name text
                        if let Ok(mut character_name_text) =
                            character_names.get_mut(*char_desc_child)
                        {
                            character_name_text.sections[0]
                                .value
                                .clone_from(&character.name);
                        }

                        // Update character abilities and stats
                        if let Ok(char_info_children) = character_info.get(*char_desc_child) {
                            for char_info_child in char_info_children {
                                // Update character ability descriptions
                                if let Ok(char_ability_entity) =
                                    character_abilities.get(*char_info_child)
                                {
                                    commands
                                        .entity(char_ability_entity)
                                        .with_children(|parent| {
                                            parent.spawn_ability_descriptions(
                                                &ui_assets,
                                                font.clone(),
                                                character,
                                                &abilities_desc_res,
                                            );
                                        });
                                // Update character stat descriptions
                                } else if let Ok(char_stats_entity) =
                                    character_stats.get(*char_info_child)
                                {
                                    commands.entity(char_stats_entity).with_children(|parent| {
                                        parent.spawn_stats(&ui_assets, character);
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // Set the character in the players resource to the middle visible character
            if let Some(Some(player_data)) = players_res
                .player_data
                .get_mut(carousel.player_idx as usize)
            {
                player_data.character = visible_characters[1];
            }
        }
    }
}

/// Updates the character carousel UI based on button actions.
///
/// This function handles the rotation of the character carousel when left or right buttons
/// are pressed, updates the visible characters, and sets the character description, abilities,
/// and stats accordingly. It also updates the player's selected character in the players resource.
fn carousel_ui_system(
    mut commands: Commands,
    mut character_carousels: Query<(&mut CharacterCarousel, &Children)>,
    character_descriptions: Query<(&CharacterDescription, &Children)>,
    mut character_names: Query<&mut Text, With<CharacterName>>,
    character_info: Query<&Children, With<CharacterInfo>>,
    character_abilities: Query<Entity, With<CharacterAbilityDescriptions>>,
    character_stats: Query<Entity, With<CharacterStatsDescriptions>>,
    mut carousel_slots: Query<(&mut UiImage, &CharacterCarouselSlot)>,
    player_assets: Res<PlayerAssets>,
    mut button_reader: EventReader<ButtonActionEvent>,
    mut players_res: ResMut<PlayersResource>,
    characters_res: Res<CharactersResource>,
    abilities_desc_res: Res<AbilityDescriptionsResource>,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
    player_ready_node: Query<&PlayerReadyNode>,
) {
    // Load the font for UI text elements
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // Collect all button action events
    let button_events: Vec<&ButtonActionEvent> = button_reader.read().collect();

    // Iterate over each character carousel
    for (mut carousel, carousel_children) in character_carousels.iter_mut() {
        let carousel_player_idx = carousel.player_idx;

        // Only allow the player to change characters if they are not ready
        let player_ready_node = player_ready_node
            .iter()
            .find(|node| node.player_idx == carousel_player_idx);

        if player_ready_node.is_some_and(|node| !node.is_ready) {
            // Filter relevant button events for the current carousel
            for button in button_events.iter().filter(|action| match action.action {
                ButtonActionType::CharacterSelectLeft(i) => i == carousel_player_idx,
                ButtonActionType::CharacterSelectRight(i) => i == carousel_player_idx,
                _ => false,
            }) {
                // Rotate the carousel based on the button action and get the new visible characters
                let new_characters =
                    if let ButtonActionType::CharacterSelectRight(_) = button.action {
                        carousel.rotate_right();
                        carousel.get_visible_characters()
                    } else if let ButtonActionType::CharacterSelectLeft(_) = button.action {
                        carousel.rotate_left();
                        carousel.get_visible_characters()
                    } else {
                        None
                    };

                // Set the correct image of each of the visible characters in the carousel
                if let Some(visible_characters) = new_characters {
                    for (idx, carousel_child) in carousel_children.iter().enumerate() {
                        if let Ok((mut ui_image, slot)) = carousel_slots.get_mut(*carousel_child) {
                            *ui_image = player_assets
                                .get_asset(&visible_characters[slot.0 as usize])
                                .into();

                            if idx != 1 {
                                ui_image.color = Color::srgba(0.60, 0.60, 0.60, 0.60);
                            }
                        }
                    }

                    // Set the character description to the middle character
                    if let Some(character) = characters_res.characters.get(&visible_characters[1]) {
                        if let Some(char_desc_children) = character_descriptions.iter().find_map(
                            |(character_description, children)| {
                                if character_description.0 == carousel_player_idx {
                                    Some(children)
                                } else {
                                    None
                                }
                            },
                        ) {
                            for char_desc_child in char_desc_children.iter() {
                                // Check if child entity is for the character name or for the character info
                                if let Ok(mut character_name_text) =
                                    character_names.get_mut(*char_desc_child)
                                {
                                    // Replace the character name with the name of the new middle character
                                    character_name_text.sections[0]
                                        .value
                                        .clone_from(&character.name);
                                } else if let Ok(char_info_children) =
                                    character_info.get(*char_desc_child)
                                {
                                    for char_info_child in char_info_children {
                                        // Check if the child is for the character abilities or the character stats
                                        if let Ok(char_abilities_entity) =
                                            character_abilities.get(*char_info_child)
                                        {
                                            // Despawn the existing ability descriptions
                                            commands
                                                .entity(char_abilities_entity)
                                                .despawn_descendants();

                                            // Spawn ability descriptions as children
                                            commands.entity(char_abilities_entity).with_children(
                                                |parent| {
                                                    parent.spawn_ability_descriptions(
                                                        &ui_assets,
                                                        font.clone(),
                                                        character,
                                                        &abilities_desc_res,
                                                    );
                                                },
                                            );
                                        } else if let Ok(char_stats_entity) =
                                            character_stats.get(*char_info_child)
                                        {
                                            // Despawn all of the existing stats
                                            commands
                                                .entity(char_stats_entity)
                                                .despawn_descendants();

                                            // Spawn character stats as children
                                            commands.entity(char_stats_entity).with_children(
                                                |parent| {
                                                    parent.spawn_stats(&ui_assets, character);
                                                },
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Set the character in the players resource to the middle visible character
                    // This value is later read to spawn the correct player in the game state
                    if let Some(Some(player_data)) = players_res
                        .player_data
                        .get_mut(carousel.player_idx as usize)
                    {
                        player_data.character = visible_characters[1];
                    }
                }
            }
        }
    }
}

/// Updates the player ready state and button color based on player actions.
///
/// This function listens for player ready events and updates the player ready state and the color
/// of the ready button to indicate the player is ready.
fn player_ready_system(
    mut button_reader: EventReader<ButtonActionEvent>,
    mut player_ready: Query<(&mut PlayerReadyNode, &Children)>,
    ready_button_parents: Query<&Children>,
    mut ready_button_images: Query<&mut UiImage>,
) {
    // Iterate over each button action event
    for event in button_reader.read() {
        if let ButtonActionType::CharacterSelectReady(player_idx) = event.action {
            // Iterate over player ready nodes
            for (mut player_ready_node, ready_node_children) in player_ready.iter_mut() {
                if player_idx == player_ready_node.player_idx {
                    // If the player is not ready, update the state to ready and change button color
                    if !player_ready_node.is_ready {
                        player_ready_node.is_ready = true;

                        // Get the first child of the ready node, which is the parent entity of the ready button
                        if let Some(ready_button_parent_entity) = ready_node_children.first() {
                            // Get the children of the ready button parent entity
                            if let Ok(ready_button_parent_children) =
                                ready_button_parents.get(*ready_button_parent_entity)
                            {
                                // Get the first child, which is the ready button entity
                                if let Some(ready_button_entity) =
                                    ready_button_parent_children.first()
                                {
                                    // Update the background color of the ready button
                                    if let Ok(mut ui_image) =
                                        ready_button_images.get_mut(*ready_button_entity)
                                    {
                                        ui_image.color = Color::srgba(0.2, 1.0, 0.4, 1.0);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Checks if all players are ready and updates the application state accordingly.
///
/// This function iterates through all player ready nodes to determine if all players
/// have marked themselves as ready. If all players are ready, it transitions the application
/// state to `AppStates::InitializeRun`.
fn check_players_ready_system(
    players_res: Res<PlayersResource>,
    player_ready_nodes: Query<&PlayerReadyNode>,
    mut next_app_state: ResMut<NextState<AppStates>>,
) {
    let mut all_players_ready = true;

    // Check if every node matching a used player slot is ready
    for node in player_ready_nodes.iter() {
        let node_ready = node.is_ready;
        let corresponding_player_exists = players_res
            .player_data
            .get(node.player_idx as usize)
            .is_some();

        // If any node is not ready and corresponds to an existing player, set flag to false
        if !node_ready && corresponding_player_exists {
            all_players_ready = false;
        }
    }

    // If all players are ready and there are players present, update the application state
    if !players_res.player_data.is_empty() && all_players_ready {
        next_app_state.set(AppStates::InitializeRun);
    }
}
