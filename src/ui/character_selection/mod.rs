//! Systems to spawn and style the character selection screen, where each player picks a character
//! from one of a few options, and possibly enables/diables the tutorial.
use crate::{assets::UiAssets, game::GameParametersResource, options::PlayingOnArcadeResource};

use super::{
    button::{self, ButtonActionComponent, ButtonActionEvent, UiButtonChildBuilderExt},
    BouncingPromptComponent,
};
use bevy::{
    app::{App, Plugin, Update},
    asset::{Asset, AssetServer, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::{Changed, With},
        schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter},
        system::{Commands, Local, ParamSet, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children, DespawnRecursiveExt},
    input::gamepad::{Gamepad, GamepadButtonChangedEvent},
    log::info,
    render::{color::Color, view::Visibility},
    sprite::TextureAtlas,
    text::Font,
    time::{Timer, TimerMode},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        widget::Button,
        AlignItems, BackgroundColor, Display, FlexDirection, Interaction, JustifyContent, Style,
        UiRect, Val,
    },
    utils::default,
};
use leafwing_input_manager::{prelude::ActionState, InputManagerBundle};
use thetawave_interface::{
    audio::PlaySoundEffectEvent,
    character,
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
            )
                .run_if(in_state(AppStates::CharacterSelection)),
        );

        app.add_systems(
            OnEnter(states::AppStates::CharacterSelection),
            setup_character_selection_system,
        );
    }
}

#[derive(Component)]
pub(super) struct PlayerJoinPrompt;

#[derive(Component, Debug)]
pub(super) struct PlayerCharacterSelection(u8);

#[derive(Component)]
struct PlayerCharacterSelectionRight(u8);

#[derive(Component)]
struct PlayerCharacterSelectionLeft(u8);

#[derive(Component)]
pub(super) struct Player2JoinPrompt;

#[derive(Component)]
pub(super) struct Player2CharacterSelection;

#[derive(Component)]
pub(super) struct CharacterSelectionChoice {
    pub character: CharacterType,
    pub is_active: bool,
}

#[derive(Component)]
pub(super) struct CharacterDescription {
    pub character: Option<CharacterType>,
}

#[derive(Component)]
pub(super) struct Player1Description;

#[derive(Component)]
pub(super) struct Player2Description;

#[derive(Component)]
pub(super) struct StartGamePrompt;

/// Setup the character selection UI
pub(super) fn setup_character_selection_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_params_res: Res<GameParametersResource>,
    playing_on_arcade: Res<PlayingOnArcadeResource>,
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
                    left: Val::Vw(2.0),
                    right: Val::Vw(2.0),
                    top: Val::Vh(2.0),
                    bottom: Val::Vh(2.0),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CharacterSelectionCleanup)
        .with_children(|parent| {
            // Top row of player joins
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(50.0),
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Row,
                        ..Default::default()
                    },
                    background_color: Color::rgba(1.0, 0.0, 0.0, 0.05).into(), // TODO: remove
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Top left player join
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                max_width: Val::Percent(50.0),
                                min_width: Val::Percent(48.0),
                                max_height: Val::Percent(100.0),
                                min_height: Val::Percent(90.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                flex_direction: FlexDirection::Row,
                                margin: UiRect {
                                    left: Val::Vw(0.0),
                                    right: Val::Vw(2.0),
                                    top: Val::Vh(0.0),
                                    bottom: Val::Vh(2.0),
                                },
                                ..Default::default()
                            },
                            background_color: Color::rgba(0.0, 1.0, 0.0, 0.05).into(), // TODO: remove
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Left side of player join
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(20.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                    ..default()
                                })
                                .insert(PlayerCharacterSelectionLeft(0));

                            // Center of player join
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(60.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgba(1.0, 1.0, 0.0, 0.5).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_button(
                                        &ui_assets,
                                        font.clone(),
                                        ButtonActionComponent::CharacterSelectJoin,
                                    );
                                })
                                .insert(PlayerCharacterSelection(0));

                            // Right side of player join
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(20.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                    ..default()
                                })
                                .insert(PlayerCharacterSelectionRight(0));
                        });

                    // Spawn second player join on the right side if there are at least 2 players
                    if game_params_res.get_max_players() > 1 {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    max_width: Val::Percent(50.0),
                                    min_width: Val::Percent(48.0),
                                    max_height: Val::Percent(100.0),
                                    min_height: Val::Percent(90.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect {
                                        left: Val::Vw(2.0),
                                        right: Val::Vw(0.0),
                                        top: Val::Vh(0.0),
                                        bottom: Val::Vh(2.0),
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgba(0.0, 1.0, 0.0, 0.05).into(), // TODO: remove
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                // Left side of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(20.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelectionLeft(1));

                                // Center of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 1.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelection(1));

                                // Right side of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(20.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelectionRight(1));
                            });
                    }
                });

            // spawn 2 rows if there are 3 or 4 players
            if game_params_res.get_max_players() > 2 {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(50.0),
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.05).into(), // TODO: remove
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    max_width: Val::Percent(50.0),
                                    min_width: Val::Percent(48.0),
                                    max_height: Val::Percent(100.0),
                                    min_height: Val::Percent(90.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    margin: UiRect {
                                        left: Val::Vw(0.0),
                                        right: Val::Vw(2.0),
                                        top: Val::Vh(2.0),
                                        bottom: Val::Vh(0.0),
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgba(0.0, 1.0, 0.0, 0.05).into(), // TODO: remove
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                // Left side of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(20.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelectionLeft(2));

                                // Center of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 1.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelection(2));

                                // Right side of player join
                                parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(20.0),
                                            height: Val::Percent(100.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                        ..default()
                                    })
                                    .insert(PlayerCharacterSelectionRight(2));
                            });

                        if game_params_res.get_max_players() > 3 {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        max_width: Val::Percent(50.0),
                                        min_width: Val::Percent(48.0),
                                        max_height: Val::Percent(100.0),
                                        min_height: Val::Percent(90.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        margin: UiRect {
                                            left: Val::Vw(2.0),
                                            right: Val::Vw(0.0),
                                            top: Val::Vh(2.0),
                                            bottom: Val::Vh(0.0),
                                        },
                                        ..Default::default()
                                    },
                                    background_color: Color::rgba(0.0, 1.0, 0.0, 0.05).into(), // TODO: remove
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    // Left side of player join
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(20.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgba(1.0, 0.0, 0.0, 0.5)
                                                .into(),
                                            ..default()
                                        })
                                        .insert(PlayerCharacterSelectionLeft(3));

                                    // Center of player join
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(60.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgba(1.0, 1.0, 0.0, 0.5)
                                                .into(),
                                            ..default()
                                        })
                                        .insert(PlayerCharacterSelection(3));

                                    // Right side of player join
                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(20.0),
                                                height: Val::Percent(100.0),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            background_color: Color::rgba(1.0, 0.0, 0.0, 0.5)
                                                .into(),
                                            ..default()
                                        })
                                        .insert(PlayerCharacterSelectionRight(3));
                                });
                        }
                    });
            }
        });
}

// handle the character selection for each player
pub(super) fn select_character_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut players_resource: ResMut<PlayersResource>,
    player_1_selection: Query<&Children, With<PlayerCharacterSelection>>,

    mut character_description_queries: ParamSet<(
        Query<(&mut Style, &CharacterDescription), With<Player1Description>>,
        Query<(&mut Style, &CharacterDescription), With<Player2Description>>,
    )>,
    mut selection_choice: Query<(
        &mut CharacterSelectionChoice,
        &mut BouncingPromptComponent,
        &mut BackgroundColor,
    )>,
) {
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
                .find(|x| x.0 .0 == *player_idx as u8);

            let current_character_selection_ui = character_selection_center
                .iter()
                .find(|x| x.0 .0 == (*player_idx as u8) + 1);

            // remove the join button from the previous character selection
            if let Some((_, entity)) = prev_character_selection_ui {
                commands.entity(entity).remove_children(&[button_entity]);
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
                .find(|x| x.0 .0 == *player_idx as u8);

            let prev_character_selection_left_arrow_ui = character_selection_left
                .iter()
                .find(|x| x.0 .0 == *player_idx as u8);

            if let Some((_, entity)) = prev_character_selection_right_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::CharacterSelectRight(*player_idx as u8),
                    )
                });
            };

            if let Some((_, entity)) = prev_character_selection_left_arrow_ui {
                commands.entity(entity).with_children(|parent| {
                    parent.spawn_button(
                        &ui_assets,
                        font.clone(),
                        ButtonActionComponent::CharacterSelectLeft(*player_idx as u8),
                    )
                });
            };

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
                .insert(MenuExplorer(*player_idx as u8));
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
