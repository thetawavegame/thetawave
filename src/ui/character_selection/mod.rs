//! Systems to spawn and style the character selection screen, where each player picks a character
//! from one of a few options, and possibly enables/diables the tutorial.
use crate::{game::GameParametersResource, options::PlayingOnArcadeResource};

use super::BouncingPromptComponent;
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, ParamSet, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, Children},
    input::gamepad::GamepadButtonChangedEvent,
    render::{color::Color, view::Visibility},
    time::{Timer, TimerMode},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        AlignItems, BackgroundColor, Display, FlexDirection, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::{
    character::CharacterType,
    character_selection::PlayerJoinEvent,
    player::{PlayerData, PlayerInput, PlayersResource},
    states::CharacterSelectionCleanup,
};

mod player_join;

#[derive(Component)]
pub(super) struct Player1JoinPrompt;

#[derive(Component)]
pub(super) struct Player1CharacterSelection;

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
) {
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
                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(20.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                ..default()
                            });

                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgba(1.0, 1.0, 0.0, 0.5).into(),
                                ..default()
                            });

                            parent.spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(20.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::rgba(1.0, 0.0, 0.0, 0.5).into(),
                                ..default()
                            });
                        });
                    if game_params_res.get_max_players() > 1 {
                        parent.spawn(NodeBundle {
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
                        parent.spawn(NodeBundle {
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
                        });
                        if game_params_res.get_max_players() > 3 {
                            parent.spawn(NodeBundle {
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
                            });
                        }
                    });
            }
        });
}

/// Handles players joining the game
pub(super) fn player_join_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut players_resource: ResMut<PlayersResource>,
    mut ui_queries: ParamSet<(
        Query<&mut Style, With<Player1JoinPrompt>>,
        Query<&mut Style, With<Player2JoinPrompt>>,
        Query<&mut Style, With<Player1CharacterSelection>>,
        Query<&mut Style, With<Player2CharacterSelection>>,
        Query<&mut Visibility, With<StartGamePrompt>>,
    )>,
    mut player_join_event: EventWriter<PlayerJoinEvent>,
) {
}

// handle the character selection for each player
pub(super) fn select_character_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut gamepad_events: EventReader<GamepadButtonChangedEvent>,
    mut players_resource: ResMut<PlayersResource>,
    player_1_selection: Query<&Children, With<Player1CharacterSelection>>,
    player_2_selection: Query<&Children, With<Player2CharacterSelection>>,

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
