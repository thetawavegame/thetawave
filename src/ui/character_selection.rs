//! Systems to spawn and style the character selection screen, where each player picks a character
//! from one of a few options, and possibly enables/diables the tutorial.
use crate::options::PlayingOnArcadeResource;

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
};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::{
    character::CharacterType,
    character_selection::PlayerJoinEvent,
    player::{PlayerData, PlayerInput, PlayersResource},
    states::CharacterSelectionCleanup,
};

#[derive(Component)]
pub(super) struct CharacterSelectionUI;

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
    playing_on_arcade: Res<PlayingOnArcadeResource>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),              // Adjusted to 90% of window width
                height: Val::Percent(100.0),             // Adjusted to 90% of window height
                justify_content: JustifyContent::Center, // Center content horizontally
                align_items: AlignItems::Center,         // Center content vertically
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CharacterSelectionCleanup)
        .insert(CharacterSelectionUI)
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        margin: UiRect {
                                            top: Val::Percent(35.0),
                                            ..Default::default()
                                        },
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load(if **playing_on_arcade {
                                                    "texture/join_prompt_arcade.png"
                                                } else {
                                                    "texture/join_prompt_keyboard.png"
                                                })
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(100.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Player1JoinPrompt)
                                        .insert(BouncingPromptComponent {
                                            flash_timer: Timer::from_seconds(
                                                2.0,
                                                TimerMode::Repeating,
                                            ),
                                            is_active: true,
                                        });

                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                display: Display::None,
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                flex_direction: FlexDirection::Row,
                                                width: Val::Px(400.0),
                                                height: Val::Px(100.0),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Player1CharacterSelection)
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ImageBundle {
                                                    image: asset_server
                                                        .load("texture/captain_character.png")
                                                        .into(),
                                                    background_color: BackgroundColor(Color::WHITE),
                                                    style: Style {
                                                        width: Val::Px(18.0 * 5.0),
                                                        height: Val::Px(18.0 * 5.0),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(BouncingPromptComponent {
                                                    flash_timer: Timer::from_seconds(
                                                        2.0,
                                                        TimerMode::Repeating,
                                                    ),
                                                    is_active: true,
                                                })
                                                .insert(CharacterSelectionChoice {
                                                    character: CharacterType::Captain,
                                                    is_active: true,
                                                });

                                            parent
                                                .spawn(ImageBundle {
                                                    image: asset_server
                                                        .load("texture/juggernaut_character.png")
                                                        .into(),
                                                    background_color: BackgroundColor(
                                                        Color::DARK_GRAY,
                                                    ),
                                                    style: Style {
                                                        width: Val::Px(18.0 * 5.0),
                                                        height: Val::Px(18.0 * 5.0),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(BouncingPromptComponent {
                                                    flash_timer: Timer::from_seconds(
                                                        2.0,
                                                        TimerMode::Repeating,
                                                    ),
                                                    is_active: false,
                                                })
                                                .insert(CharacterSelectionChoice {
                                                    character: CharacterType::Juggernaut,
                                                    is_active: false,
                                                });
                                        });

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load(if **playing_on_arcade {
                                                    "texture/join_prompt_arcade.png"
                                                } else {
                                                    "texture/join_prompt_keyboard.png"
                                                })
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(100.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Player2JoinPrompt)
                                        .insert(BouncingPromptComponent {
                                            flash_timer: Timer::from_seconds(
                                                2.0,
                                                TimerMode::Repeating,
                                            ),
                                            is_active: true,
                                        });

                                    parent
                                        .spawn(NodeBundle {
                                            style: Style {
                                                display: Display::None,
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                flex_direction: FlexDirection::Row,
                                                width: Val::Px(400.0),
                                                height: Val::Px(100.0),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(Player2CharacterSelection)
                                        .with_children(|parent| {
                                            parent
                                                .spawn(ImageBundle {
                                                    image: asset_server
                                                        .load("texture/captain_character.png")
                                                        .into(),
                                                    background_color: BackgroundColor(Color::WHITE),
                                                    style: Style {
                                                        width: Val::Px(18.0 * 5.0),
                                                        height: Val::Px(18.0 * 5.0),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(BouncingPromptComponent {
                                                    flash_timer: Timer::from_seconds(
                                                        2.0,
                                                        TimerMode::Repeating,
                                                    ),
                                                    is_active: true,
                                                })
                                                .insert(CharacterSelectionChoice {
                                                    character: CharacterType::Captain,
                                                    is_active: true,
                                                });

                                            parent
                                                .spawn(ImageBundle {
                                                    image: asset_server
                                                        .load("texture/juggernaut_character.png")
                                                        .into(),
                                                    background_color: BackgroundColor(
                                                        Color::DARK_GRAY,
                                                    ),
                                                    style: Style {
                                                        width: Val::Px(18.0 * 5.0),
                                                        height: Val::Px(18.0 * 5.0),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            ..Default::default()
                                                        },
                                                        ..Default::default()
                                                    },
                                                    ..Default::default()
                                                })
                                                .insert(BouncingPromptComponent {
                                                    flash_timer: Timer::from_seconds(
                                                        2.0,
                                                        TimerMode::Repeating,
                                                    ),
                                                    is_active: false,
                                                })
                                                .insert(CharacterSelectionChoice {
                                                    character: CharacterType::Juggernaut,
                                                    is_active: false,
                                                });
                                        });
                                });

                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        margin: UiRect {
                                            ..Default::default()
                                        },
                                        flex_direction: FlexDirection::Row,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/captain_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription {
                                            character: Some(CharacterType::Captain),
                                        })
                                        .insert(Player1Description);

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/juggernaut_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription {
                                            character: Some(CharacterType::Juggernaut),
                                        })
                                        .insert(Player1Description);

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/blank_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription { character: None })
                                        .insert(Player1Description);

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/captain_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription {
                                            character: Some(CharacterType::Captain),
                                        })
                                        .insert(Player2Description);

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/juggernaut_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription {
                                            character: Some(CharacterType::Juggernaut),
                                        })
                                        .insert(Player2Description);

                                    parent
                                        .spawn(ImageBundle {
                                            image: asset_server
                                                .load("texture/blank_description.png")
                                                .into(),
                                            style: Style {
                                                width: Val::Px(400.0),
                                                height: Val::Px(300.0),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    ..Default::default()
                                                },
                                                display: Display::None,
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .insert(CharacterDescription { character: None })
                                        .insert(Player2Description);
                                });
                            parent
                                .spawn(ImageBundle {
                                    image: asset_server
                                        .load(if **playing_on_arcade {
                                            "texture/start_game_prompt_arcade.png"
                                        } else {
                                            "texture/start_game_prompt_keyboard.png"
                                        })
                                        .into(),
                                    style: Style {
                                        width: Val::Px(300.0),
                                        height: Val::Px(75.0),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            bottom: Val::Percent(1.0),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    visibility: Visibility::Hidden,
                                    ..Default::default()
                                })
                                .insert(BouncingPromptComponent {
                                    flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                    is_active: true,
                                })
                                .insert(StartGamePrompt);
                        });
                });
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
    // get all of the already used inputs
    let used_inputs = players_resource.get_used_inputs();

    // get menu action
    let action_state = menu_input_query.single();

    // join with keyboard
    if action_state.just_released(&MenuAction::JoinKeyboard) {
        // set the first available player input to keyboard
        for (i, player_data) in players_resource.player_data.iter_mut().enumerate() {
            if player_data.is_none() && !used_inputs.contains(&PlayerInput::Keyboard) {
                *player_data = Some(PlayerData {
                    character: CharacterType::Captain,
                    input: PlayerInput::Keyboard,
                });

                // send event that player joined
                player_join_event.send(PlayerJoinEvent(i));

                // remove the player join prompt
                if i == 0 {
                    ui_queries.p0().single_mut().display = Display::None;
                    ui_queries.p2().single_mut().display = Display::Flex;
                } else if i == 1 {
                    ui_queries.p1().single_mut().display = Display::None;
                    ui_queries.p3().single_mut().display = Display::Flex;
                } else {
                    todo!("implement more than 2 players")
                }
                break;
            }
        }
    }

    // join with gamepad
    if action_state.just_released(&MenuAction::JoinGamepad) {
        let gamepad_event = gamepad_events.read().next().unwrap();

        // set the first available player input the gamepad
        for (i, player_data) in players_resource.player_data.iter_mut().enumerate() {
            if player_data.is_none()
                && !used_inputs.contains(&PlayerInput::Gamepad(gamepad_event.gamepad.id))
            {
                *player_data = Some(PlayerData {
                    character: CharacterType::Captain,
                    input: PlayerInput::Gamepad(gamepad_event.gamepad.id),
                });

                // send event that player joined
                player_join_event.send(PlayerJoinEvent(i));

                // remove the player join prompt
                if i == 0 {
                    ui_queries.p0().single_mut().display = Display::None;
                    ui_queries.p2().single_mut().display = Display::Flex;
                } else if i == 1 {
                    ui_queries.p1().single_mut().display = Display::None;
                    ui_queries.p3().single_mut().display = Display::Flex;
                } else {
                    todo!("implement more than 2 players")
                }
                break;
            }
        }
    }

    // show the start game prompt if at least one player has joined
    if players_resource.player_data[0].is_some() {
        *ui_queries.p4().single_mut() = Visibility::Inherited;
    } else {
        *ui_queries.p4().single_mut() = Visibility::Hidden;
    }
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
    let action_state = menu_input_query.single();

    let keyboard_input = action_state.just_pressed(&MenuAction::ChangeCharacterKeyboard);

    let gamepad_input = action_state.just_pressed(&MenuAction::ChangeCharacterGamepad);

    let gamepad_event_id = if gamepad_input {
        gamepad_events
            .read()
            .next()
            .map(|gamepad_event| gamepad_event.gamepad.id)
    } else {
        None
    };

    // handle player 1 selection
    let children = player_1_selection.single();
    for child in children.iter() {
        let (mut choice, mut bounce, mut bg_color) = selection_choice.get_mut(*child).unwrap();
        if let Some(player_data) = &mut players_resource.player_data[0] {
            match player_data.input {
                PlayerInput::Keyboard => {
                    if keyboard_input {
                        if choice.is_active {
                            choice.is_active = false;
                            bounce.is_active = false;
                            *bg_color = BackgroundColor(Color::DARK_GRAY);
                        } else {
                            choice.is_active = true;
                            bounce.is_active = true;
                            *bg_color = BackgroundColor(Color::WHITE);
                            player_data.character = choice.character.clone();
                        }
                    }
                }
                PlayerInput::Gamepad(gamepad_id) => {
                    if let Some(id) = gamepad_event_id {
                        if gamepad_input && gamepad_id == id {
                            if choice.is_active {
                                choice.is_active = false;
                                bounce.is_active = false;
                                *bg_color = BackgroundColor(Color::DARK_GRAY);
                            } else {
                                choice.is_active = true;
                                bounce.is_active = true;
                                *bg_color = BackgroundColor(Color::WHITE);
                                player_data.character = choice.character.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    // handle player 2 selection
    let children = player_2_selection.single();
    for child in children.iter() {
        let (mut choice, mut bounce, mut bg_color) = selection_choice.get_mut(*child).unwrap();
        if let Some(player_data) = &mut players_resource.player_data[1] {
            match player_data.input {
                PlayerInput::Keyboard => {
                    if keyboard_input {
                        if choice.is_active {
                            choice.is_active = false;
                            bounce.is_active = false;
                            *bg_color = BackgroundColor(Color::DARK_GRAY);
                        } else {
                            choice.is_active = true;
                            bounce.is_active = true;
                            *bg_color = BackgroundColor(Color::WHITE);
                            player_data.character = choice.character.clone();
                        }
                    }
                }
                PlayerInput::Gamepad(gamepad_id) => {
                    if let Some(id) = gamepad_event_id {
                        if gamepad_input && gamepad_id == id {
                            if choice.is_active {
                                choice.is_active = false;
                                bounce.is_active = false;
                                *bg_color = BackgroundColor(Color::DARK_GRAY);
                            } else {
                                choice.is_active = true;
                                bounce.is_active = true;
                                *bg_color = BackgroundColor(Color::WHITE);
                                player_data.character = choice.character.clone();
                            }
                        }
                    }
                }
            }
        }
    }

    // set the charcater description for player 1
    for (mut style, description) in character_description_queries.p0().iter_mut() {
        if players_resource.player_data[0]
            .clone()
            .map(|player_data| player_data.character)
            == description.character
        {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }

    // set the charcater description for player 2
    for (mut style, description) in character_description_queries.p1().iter_mut() {
        if players_resource.player_data[1]
            .clone()
            .map(|player_data| player_data.character)
            == description.character
        {
            style.display = Display::Flex;
        } else {
            style.display = Display::None;
        }
    }
}
