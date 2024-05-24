//! Systems to spawn and style the character selection screen, where each player picks a character
//! from one of a few options, and possibly enables/diables the tutorial.
use std::collections::VecDeque;

use crate::{
    assets::{PlayerAssets, UiAssets},
    game::GameParametersResource,
    player::CharactersResource,
};

use super::button::{ButtonActionComponent, ButtonActionEvent, UiButtonChildBuilderExt};
use bevy::{
    app::{App, Plugin, Update},
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Local, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children, DespawnRecursiveExt},
    input::gamepad::{Gamepad, GamepadButtonChangedEvent},
    render::color::Color,
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
use strum::IntoEnumIterator;
use thetawave_interface::{
    abilities::AbilityDescriptionsResource,
    audio::PlaySoundEffectEvent,
    character::{Character, CharacterStatType},
    game,
    input::{InputsResource, MainMenuExplorer, MenuAction, MenuExplorer},
    states::{self, AppStates},
};
use thetawave_interface::{
    character::CharacterType,
    character_selection::PlayerJoinEvent,
    player::{PlayerData, PlayerInput, PlayersResource},
    states::CharacterSelectionCleanup,
};

mod player_join;

const DEFAULT_CHARACTER: CharacterType = CharacterType::Captain;

pub(super) struct CharacterSelectionPlugin;
impl Plugin for CharacterSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerJoinEvent>();

        app.add_systems(
            Update,
            (
                player_join_system,
                character_selection_mouse_click_system,
                character_selection_keyboard_gamepad_system,
                update_ui_system,
                carousel_ui_system,
            )
                .run_if(in_state(AppStates::CharacterSelection)),
        );

        app.add_systems(
            OnEnter(states::AppStates::CharacterSelection),
            setup_character_selection_system,
        );
    }
}

#[derive(Component, Debug)]
pub(super) struct PlayerCharacterSelection(u8);

#[derive(Component)]
struct PlayerCharacterSelectionRight(u8);

#[derive(Component)]
struct PlayerCharacterSelectionLeft(u8);

#[derive(Component)]
struct CharacterCarousel {
    pub player_idx: u8,
    pub characters: VecDeque<CharacterType>,
}

#[derive(Component)]
struct CharacterDescription(u8);

#[derive(Component)]
struct CharacterInfo;

#[derive(Component)]
struct CharacterName;

#[derive(Component)]
struct CharacterAbilityDescriptions;

#[derive(Component)]
struct CharacterStatsDescriptions;

#[derive(Component)]
struct CharacterCarouselSlot(u8);

#[derive(Component)]
struct PlayerReadyNode(u8);

impl CharacterCarousel {
    fn new(player_idx: u8) -> Self {
        CharacterCarousel {
            player_idx,
            characters: CharacterType::to_vec(),
        }
    }

    fn rotate_right(&mut self) {
        if let Some(last_element) = self.characters.pop_back() {
            self.characters.push_front(last_element);
        }
    }

    fn rotate_left(&mut self) {
        if let Some(first_element) = self.characters.pop_front() {
            self.characters.push_back(first_element);
        }
    }

    fn get_visible_characters(&self) -> [CharacterType; 3] {
        if let (Some(left), Some(middle), Some(right)) = (
            self.characters.back(),
            self.characters.front(),
            self.characters.get(1),
        ) {
            return [*left, *middle, *right];
        }
        [
            CharacterType::default(),
            CharacterType::default(),
            CharacterType::default(),
        ]
    }
}

pub trait UiPlayerJoinChildBuilderExt {
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
        for stat in CharacterStatType::iter() {
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
                parent.spawn(ImageBundle {
                    image: ui_assets.get_stat_icon(&stat).into(),
                    style: Style {
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    ..default()
                });

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
                        parent.spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(character.get_stat_percent(&stat)),
                                height: Val::Percent(50.0),
                                align_self: AlignSelf::Center,
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
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
        if let Some(slot_1_ability_type) = &character.slot_1_ability {
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
                parent
                    .spawn(ImageBundle {
                        image: ui_assets.get_ability_slot_image(false).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: ui_assets
                                .get_slot_1_ability_image(slot_1_ability_type)
                                .into(),
                            ..default()
                        });
                    });

                if let Some(ability_desc) = abilities_desc_res.slot_one.get(slot_1_ability_type) {
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

        if let Some(slot_2_ability_type) = &character.slot_2_ability {
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
                parent
                    .spawn(ImageBundle {
                        image: ui_assets.get_ability_slot_image(false).into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            image: ui_assets
                                .get_slot_2_ability_image(slot_2_ability_type)
                                .into(),
                            ..default()
                        });
                    });

                if let Some(ability_desc) = abilities_desc_res.slot_two.get(slot_2_ability_type) {
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
            for player_idx in players {
                // Top left player join
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
                        background_color: Color::BLACK.with_a(0.6).into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        // Left side of player join
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
                            .insert(PlayerCharacterSelectionLeft(player_idx));

                        // Center of player join
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
                                    .insert(PlayerCharacterSelection(player_idx))
                                    .with_children(|parent| {
                                        if player_idx == 0 {
                                            parent.spawn_button(
                                                ui_assets,
                                                font.clone(),
                                                ButtonActionComponent::CharacterSelectJoin,
                                                None,
                                            );
                                        }
                                    });

                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(20.0),
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .insert(PlayerReadyNode(player_idx));
                            });

                        // Right side of player join
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
                            .insert(PlayerCharacterSelectionRight(player_idx));
                    });
            }
        });
    }
}

/// Setup the character selection UI
pub(super) fn setup_character_selection_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_params_res: Res<GameParametersResource>,
    ui_assets: Res<UiAssets>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // Main node containing all character selection ui
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
            // create vectors of player idxs to use for spawning player join rows
            let mut top_row_player_idxs = vec![];
            let mut bottom_row_player_idxs = vec![];

            for i in 0..game_params_res.get_max_players() {
                if (top_row_player_idxs.len() as u8) < game_params_res.get_max_players() / 2 {
                    top_row_player_idxs.push(i);
                } else {
                    bottom_row_player_idxs.push(i)
                }
            }

            // spawn player join rows
            if !top_row_player_idxs.is_empty() {
                parent.spawn_player_join_row(&ui_assets, font.clone(), top_row_player_idxs);
            }

            if !bottom_row_player_idxs.is_empty() {
                parent.spawn_player_join_row(&ui_assets, font.clone(), bottom_row_player_idxs);
            }
        });
}

fn player_join_system(
    button_mouse_movements: Query<(&ButtonActionComponent, &Interaction, Entity), With<Button>>,
    menu_explorer_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut sound_effect: EventWriter<PlaySoundEffectEvent>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    mut mouse_interaction: Local<Interaction>,
    game_params_res: Res<GameParametersResource>,
    mut players_resource: ResMut<PlayersResource>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut player_join_event: EventWriter<PlayerJoinEvent>,
) {
    if let Some(button) = button_mouse_movements.iter().find(|(button_action, _, _)| {
        matches!(button_action, ButtonActionComponent::CharacterSelectJoin)
    }) {
        // check if input was already used
        let used_inputs = players_resource.get_used_inputs();

        // check if the maximum amount of players have already joined
        if used_inputs.len() < game_params_res.get_max_players() as usize {
            // Detect if a player is joining through a keyboard button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(x) => {
                    if x.get_just_released()
                        .iter()
                        .find(|action_| match action_ {
                            MenuAction::JoinKeyboard => {
                                !used_inputs.contains(&PlayerInput::Keyboard)
                            }
                            _ => false,
                        })
                        .is_some()
                    {
                        Some(PlayerInput::Keyboard)
                    } else {
                        None
                    }
                }
            } {
                // Push the new player to the players resource
                players_resource.player_data.push(Some(PlayerData {
                    character: DEFAULT_CHARACTER,
                    input: player_input,
                }));

                // Send events player join event and button event
                button_event_writer.send(ButtonActionEvent::CharacterSelectJoin);
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            };

            // Detect if a player is joining through a gamepad button press
            if let Some(player_input) = match menu_explorer_query.get_single() {
                Err(_) => None,
                Ok(x) => {
                    if let Some(gamepad_event) = gamepad_events.read().next() {
                        if x.get_just_released()
                            .iter()
                            .find(|action_| match action_ {
                                MenuAction::JoinGamepad => !used_inputs
                                    .contains(&PlayerInput::Gamepad(gamepad_event.gamepad.id)),
                                _ => false,
                            })
                            .is_some()
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
                    character: DEFAULT_CHARACTER,
                    input: player_input,
                }));

                // Send events player join event and button event
                button_event_writer.send(ButtonActionEvent::CharacterSelectJoin);
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // Detect if a player is joining through a mouse button release
            if let Some(player_input) = match button.1 {
                // check if mouse interaction changed from Pressed to Hovered
                // which means the player just released the mouse button over the ui button
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
                    character: DEFAULT_CHARACTER,
                    input: player_input,
                }));

                // Send events player join event and button event
                button_event_writer.send(ButtonActionEvent::CharacterSelectJoin);
                player_join_event.send(PlayerJoinEvent {
                    player_idx: players_resource.player_data.len() as u8 - 1,
                    input: player_input,
                });
            }

            // track the current mouse interaction in a local variable
            *mouse_interaction = *button.1;
        }
    }
}

fn update_ui_system(
    mut commands: Commands,
    mut player_join_event: EventReader<PlayerJoinEvent>,
    asset_server: Res<AssetServer>,
    character_selection_center: Query<(&PlayerCharacterSelection, Entity)>,
    character_selection_right: Query<(&PlayerCharacterSelectionRight, Entity)>,
    character_selection_left: Query<(&PlayerCharacterSelectionLeft, Entity)>,
    player_ready: Query<(&PlayerReadyNode, Entity)>,
    buttons: Query<(&ButtonActionComponent, Entity), With<Button>>,
    ui_assets: Res<UiAssets>,
    inputs_res: Res<InputsResource>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    // read all player join events
    for PlayerJoinEvent { player_idx, input } in player_join_event.read() {
        if let Some((_, button_entity)) = buttons
            .iter()
            .find(|(action, _)| matches!(action, ButtonActionComponent::CharacterSelectJoin))
        {
            // get center ui for the current player slot and the next player slot
            let prev_character_selection_ui = character_selection_center
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let current_character_selection_ui = character_selection_center
                .iter()
                .find(|x| x.0 .0 == player_idx + 1);

            // remove the join button from the previous character selection
            if let Some((_, entity)) = prev_character_selection_ui {
                commands.entity(entity).remove_children(&[button_entity]);

                // spawn a character selection carousel
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
                            parent
                                .spawn(TextBundle {
                                    text: Text::from_section(
                                        "",
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 20.0,
                                            color: Color::GOLD,
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

            // add the join button to the the new character selection
            if let Some((_, entity)) = current_character_selection_ui {
                commands.entity(entity).add_child(button_entity);
            } else {
                // if entity was not found for new character selection despawn the button
                // this means all of the available players have been used up
                commands.entity(button_entity).despawn_recursive();
            }

            // spawn right and left arrow buttons for the previous character selection
            let prev_character_selection_right_arrow_ui = character_selection_right
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let prev_character_selection_left_arrow_ui = character_selection_left
                .iter()
                .find(|x| x.0 .0 == *player_idx);

            let player_ready_node = player_ready.iter().find(|x| x.0 .0 == *player_idx);

            if let Some((_, entity)) = prev_character_selection_right_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::CharacterSelectRight(*player_idx),
                        None,
                    )
                });
            };

            if let Some((_, entity)) = prev_character_selection_left_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::CharacterSelectLeft(*player_idx),
                        None,
                    )
                });
            };

            if let Some((_, entity)) = player_ready_node {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::CharacterSelectReady(*player_idx),
                        Some(input),
                    );
                });
            }

            // spawn a menu explorer with the new player idx
            let mut input_map = inputs_res.menu.clone();

            if let PlayerInput::Gamepad(id) = *input {
                input_map.set_gamepad(Gamepad { id });
            }

            commands
                .spawn(InputManagerBundle::<MenuAction> {
                    action_state: ActionState::default(),
                    input_map,
                })
                .insert(MenuExplorer(*player_idx));
        }
    }
}

fn character_selection_mouse_click_system(
    button_mouse_movements: Query<(&ButtonActionComponent, &Interaction), With<Button>>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    mut stored_right_mouse_interaction: Local<[Interaction; 4]>,
    mut stored_left_mouse_interaction: Local<[Interaction; 4]>,
    players_resource: Res<PlayersResource>,
) {
    // check for a player using a keyboard input
    let maybe_keyboard_idx = players_resource
        .get_used_inputs()
        .iter()
        .enumerate()
        .find_map(|(idx, input)| {
            if matches!(input, PlayerInput::Keyboard) {
                Some(idx as u8)
            } else {
                None
            }
        });

    if let Some(keyboard_idx) = maybe_keyboard_idx {
        for (button_action, mouse_interaction) in
            button_mouse_movements.iter().filter(|(button_action, _)| {
                if let ButtonActionComponent::CharacterSelectLeft(i) = button_action {
                    return *i == keyboard_idx;
                } else if let ButtonActionComponent::CharacterSelectRight(i) = button_action {
                    return *i == keyboard_idx;
                }
                false
            })
        {
            let button_released = match button_action {
                ButtonActionComponent::CharacterSelectRight(i) => {
                    let result = {
                        match mouse_interaction {
                            Interaction::Hovered => {
                                match stored_right_mouse_interaction[*i as usize] {
                                    Interaction::Pressed => true,
                                    _ => false,
                                }
                            }
                            _ => false,
                        }
                    };
                    stored_right_mouse_interaction[*i as usize] = *mouse_interaction;

                    result
                }
                ButtonActionComponent::CharacterSelectLeft(i) => {
                    let result = {
                        match mouse_interaction {
                            Interaction::Hovered => {
                                match stored_left_mouse_interaction[*i as usize] {
                                    Interaction::Pressed => true,
                                    _ => false,
                                }
                            }
                            _ => false,
                        }
                    };

                    stored_left_mouse_interaction[*i as usize] = *mouse_interaction;

                    result
                }
                _ => false,
            };

            // send event if any join input was detected
            if button_released {
                button_event_writer.send(*button_action);
            }
        }
    }
}

fn character_selection_keyboard_gamepad_system(
    mut button_event_writer: EventWriter<ButtonActionEvent>,
    menu_input_query: Query<(&ActionState<MenuAction>, &MenuExplorer)>,
    players_resource: Res<PlayersResource>,
) {
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

    for (action_state, MenuExplorer(player_idx)) in menu_input_query.iter() {
        for action in action_state.get_just_released().iter() {
            if gamepad_idxs.contains(player_idx) {
                match action {
                    MenuAction::NavigateLeftGamepad => {
                        button_event_writer
                            .send(ButtonActionEvent::CharacterSelectLeft(*player_idx));
                    }
                    MenuAction::NavigateRightGamepad => {
                        button_event_writer
                            .send(ButtonActionEvent::CharacterSelectRight(*player_idx));
                    }
                    _ => {}
                };
            } else {
                match action {
                    MenuAction::NavigateLeftKeyboard => {
                        button_event_writer
                            .send(ButtonActionEvent::CharacterSelectLeft(*player_idx));
                    }
                    MenuAction::NavigateRightKeyboard => {
                        button_event_writer
                            .send(ButtonActionEvent::CharacterSelectRight(*player_idx));
                    }
                    _ => {}
                }
            }
        }
    }
}

fn carousel_ui_system(
    mut commands: Commands,
    mut character_carousels: Query<(Entity, &mut CharacterCarousel, Option<&Children>)>,
    mut character_descriptions: Query<(&CharacterDescription, &Children)>,
    mut character_names: Query<&mut Text, With<CharacterName>>,
    character_info: Query<&Children, With<CharacterInfo>>,
    mut character_abilities: Query<Entity, With<CharacterAbilityDescriptions>>,
    mut character_stats: Query<Entity, With<CharacterStatsDescriptions>>,
    mut carousel_slots: Query<(&mut UiImage, &CharacterCarouselSlot)>,
    player_assets: Res<PlayerAssets>,
    mut button_reader: EventReader<ButtonActionEvent>,
    mut players_res: ResMut<PlayersResource>,
    characters_res: Res<CharactersResource>,
    abilities_desc_res: Res<AbilityDescriptionsResource>,
    ui_assets: Res<UiAssets>,
    asset_server: Res<AssetServer>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

    let button_events: Vec<&ButtonActionEvent> = button_reader.read().collect();

    for (entity, mut carousel, maybe_children) in character_carousels.iter_mut() {
        let carousel_player_idx = carousel.player_idx;
        if let Some(carousel_children) = maybe_children {
            for button in button_events.iter().filter(|action| match action {
                ButtonActionEvent::CharacterSelectLeft(i) => *i == carousel_player_idx,
                ButtonActionEvent::CharacterSelectRight(i) => *i == carousel_player_idx,
                _ => false,
            }) {
                // rotate the carousel if correseponding button input is detected
                let new_characters = if let ButtonActionEvent::CharacterSelectRight(_) = button {
                    carousel.rotate_right();
                    Some(carousel.get_visible_characters())
                } else if let ButtonActionEvent::CharacterSelectLeft(_) = button {
                    carousel.rotate_left();
                    Some(carousel.get_visible_characters())
                } else {
                    None
                };

                if let Some(visible_characters) = new_characters {
                    for child in carousel_children.iter() {
                        if let Ok((mut ui_image, slot)) = carousel_slots.get_mut(*child) {
                            *ui_image = player_assets
                                .get_asset(&visible_characters[slot.0 as usize])
                                .into();
                        }
                    }

                    if let Some(character) = characters_res.characters.get(&visible_characters[1]) {
                        // set the character description to the middle character
                        if let Some(children_1) = character_descriptions.iter_mut().find_map(
                            |(character_description, text)| {
                                if character_description.0 == carousel_player_idx {
                                    Some(text)
                                } else {
                                    None
                                }
                            },
                        ) {
                            for child_1 in children_1.iter() {
                                if let Ok(mut character_name_text) =
                                    character_names.get_mut(*child_1)
                                {
                                    character_name_text.sections[0]
                                        .value
                                        .clone_from(&character.name);
                                }

                                if let Ok(children_2) = character_info.get(*child_1) {
                                    for child_2 in children_2 {
                                        if let Ok(entity) = character_abilities.get(*child_2) {
                                            commands.entity(entity).despawn_descendants();

                                            commands.entity(entity).with_children(|parent| {
                                                parent.spawn_ability_descriptions(
                                                    &ui_assets,
                                                    font.clone(),
                                                    character,
                                                    &abilities_desc_res,
                                                );
                                            });
                                        } else if let Ok(entity) = character_stats.get(*child_2) {
                                            commands.entity(entity).despawn_descendants();

                                            commands.entity(entity).with_children(|parent| {
                                                parent.spawn_stats(&ui_assets, character);
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // set the character in the players resource to the middle visible character
                    if let Some(Some(player_data)) = players_res
                        .player_data
                        .get_mut(carousel.player_idx as usize)
                    {
                        player_data.character = visible_characters[1];
                    }
                }
            }
        } else {
            let visible_characters = carousel.get_visible_characters();

            // spawn initial children
            commands.entity(entity).with_children(|parent| {
                parent
                    .spawn(ImageBundle {
                        image: player_assets.get_asset(&visible_characters[0]).into(),
                        background_color: Color::rgba(0.60, 0.60, 0.60, 0.60).into(),
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
                        image: player_assets.get_asset(&visible_characters[2]).into(),
                        background_color: Color::rgba(0.60, 0.60, 0.60, 0.60).into(),
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

            if let Some(character) = characters_res.characters.get(&visible_characters[1]) {
                // set the character description to the middle character
                if let Some(children_1) =
                    character_descriptions
                        .iter_mut()
                        .find_map(|(character_description, text)| {
                            if character_description.0 == carousel_player_idx {
                                Some(text)
                            } else {
                                None
                            }
                        })
                {
                    for child_1 in children_1.iter() {
                        if let Ok(mut character_name_text) = character_names.get_mut(*child_1) {
                            character_name_text.sections[0]
                                .value
                                .clone_from(&character.name);
                        }

                        if let Ok(children_2) = character_info.get(*child_1) {
                            for child_2 in children_2 {
                                if let Ok(entity) = character_abilities.get(*child_2) {
                                    commands.entity(entity).with_children(|parent| {
                                        parent.spawn_ability_descriptions(
                                            &ui_assets,
                                            font.clone(),
                                            character,
                                            &abilities_desc_res,
                                        );
                                    });
                                } else if let Ok(entity) = character_stats.get(*child_2) {
                                    commands.entity(entity).with_children(|parent| {
                                        parent.spawn_stats(&ui_assets, character);
                                    });
                                }
                            }
                        }
                    }
                }
            }

            // set the character in the players resource to the middle visible character
            if let Some(Some(player_data)) = players_res
                .player_data
                .get_mut(carousel.player_idx as usize)
            {
                player_data.character = visible_characters[1];
            }
        }
    }
}
