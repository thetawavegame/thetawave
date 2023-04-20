use super::BouncingPromptComponent;
use crate::{
    player::{Character, CharacterType, PlayerInput, PlayersResource},
    states,
};
use bevy::{prelude::*, utils::hashbrown::HashMap};

#[derive(Component)]
pub struct CharacterSelectionUI;

#[derive(Component)]
pub struct Player1JoinPrompt;

#[derive(Component)]
pub struct Player1CharacterSelection;

#[derive(Component)]
pub struct Player2JoinPrompt;

#[derive(Component)]
pub struct Player2CharacterSelection;

#[derive(Component)]
pub struct CharacterSelectionChoice {
    pub character: CharacterType,
    pub is_active: bool,
}

#[derive(Component)]
pub struct StartGamePrompt;

pub fn setup_character_selection_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(states::CharacterSelectionCleanup)
        .insert(CharacterSelectionUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load("texture/character_selection_54.png")
                        .into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                flex_direction: FlexDirection::Column,

                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        margin: UiRect {
                                            top: Val::Percent(45.0),
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
                                                .load("texture/join_prompt_arcade.png")
                                                .into(),
                                            style: Style {
                                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    //top: Val::Percent(65.0),
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
                                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
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
                                                        size: Size::new(
                                                            Val::Px(18.0 * 5.0),
                                                            Val::Px(18.0 * 5.0),
                                                        ),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            //top: Val::Percent(65.0),
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
                                                        size: Size::new(
                                                            Val::Px(18.0 * 5.0),
                                                            Val::Px(18.0 * 5.0),
                                                        ),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            //top: Val::Percent(65.0),
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
                                                .load("texture/join_prompt_arcade.png")
                                                .into(),
                                            style: Style {
                                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                                margin: UiRect {
                                                    right: Val::Auto,
                                                    left: Val::Auto,
                                                    //top: Val::Percent(65.0),
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
                                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
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
                                                        size: Size::new(
                                                            Val::Px(18.0 * 5.0),
                                                            Val::Px(18.0 * 5.0),
                                                        ),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            //top: Val::Percent(65.0),
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
                                                        size: Size::new(
                                                            Val::Px(18.0 * 5.0),
                                                            Val::Px(18.0 * 5.0),
                                                        ),
                                                        margin: UiRect {
                                                            right: Val::Auto,
                                                            left: Val::Auto,
                                                            //top: Val::Percent(65.0),
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
                                .spawn(ImageBundle {
                                    image: asset_server
                                        .load("texture/start_game_prompt_arcade.png")
                                        .into(),
                                    style: Style {
                                        size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                        margin: UiRect {
                                            left: Val::Auto,
                                            right: Val::Auto,
                                            bottom: Val::Percent(2.0),
                                            //top: Val::Percent(65.0),
                                            ..Default::default()
                                        },
                                        //align_content: AlignContent::Center,
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

pub fn player_join_system(
    gamepads: Res<Gamepads>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut gamepad_input: ResMut<Input<GamepadButton>>,
    mut players_resource: ResMut<PlayersResource>,
    mut start_game_prompt: Query<&mut Visibility, With<StartGamePrompt>>,
    mut ui_queries: ParamSet<(
        Query<&mut Style, With<Player1JoinPrompt>>,
        Query<&mut Style, With<Player2JoinPrompt>>,
        Query<&mut Style, With<Player1CharacterSelection>>,
        Query<&mut Style, With<Player2CharacterSelection>>,
    )>,
) {
    // get all of the already used inputs
    let used_inputs: Vec<PlayerInput> = players_resource
        .player_inputs
        .iter()
        .filter(|input| input.is_some())
        .map(|input| input.clone().unwrap())
        .collect();

    // check for keyboard input
    let mut keyboard_join_input = keyboard_input.just_released(KeyCode::LShift);

    // join with keyboard
    if keyboard_join_input {
        // set the first available player input to keyboard
        for (idx, player_input) in players_resource.player_inputs.iter_mut().enumerate() {
            if player_input.is_none() && !used_inputs.contains(&PlayerInput::Keyboard) {
                *player_input = Some(PlayerInput::Keyboard);

                // remove the player join prompt
                if idx == 0 {
                    ui_queries.p0().single_mut().display = Display::None;
                    ui_queries.p2().single_mut().display = Display::Flex;
                } else if idx == 1 {
                    ui_queries.p1().single_mut().display = Display::None;
                    ui_queries.p3().single_mut().display = Display::Flex;
                } else {
                    todo!("implement more than 2 players")
                }
                break;
            }
        }
    }

    let gamepad_join_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.just_released(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::South,
                }),
            )
        })
        .collect();

    for (gamepad_id, input) in gamepad_join_inputs.iter() {
        if *input {
            //set the first available player input to gamepad
            for (idx, player_input) in players_resource.player_inputs.iter_mut().enumerate() {
                if player_input.is_none()
                    && !used_inputs.contains(&PlayerInput::Gamepad(*gamepad_id))
                {
                    *player_input = Some(PlayerInput::Gamepad(*gamepad_id));

                    // remove the player join prompt
                    if idx == 0 {
                        ui_queries.p0().single_mut().display = Display::None;
                        ui_queries.p2().single_mut().display = Display::Flex;
                    } else if idx == 1 {
                        ui_queries.p1().single_mut().display = Display::None;
                        ui_queries.p3().single_mut().display = Display::Flex;
                    } else {
                        todo!("implement more than 2 players")
                    }
                    break;
                }
            }
        }
    }

    // show the start game prompt if at least one player has joined
    if players_resource.player_inputs[0].is_some() {
        *start_game_prompt.single_mut() = Visibility::Inherited;
    } else {
        *start_game_prompt.single_mut() = Visibility::Hidden;
    }
    println!("players resource: {:?}", players_resource);
}

// handle the character selection for each player
pub fn select_character_system(
    gamepads: Res<Gamepads>,
    keyboard_input: Res<Input<KeyCode>>,
    gamepad_input: Res<Input<GamepadButton>>,
    mut players_resource: ResMut<PlayersResource>,
    player_1_selection: Query<&Children, With<Player1CharacterSelection>>,
    player_2_selection: Query<&Children, With<Player2CharacterSelection>>,
    mut selection_choice: Query<(
        &mut CharacterSelectionChoice,
        &mut BouncingPromptComponent,
        &mut BackgroundColor,
    )>,
) {
    let keyboard_input =
        keyboard_input.just_pressed(KeyCode::D) || keyboard_input.just_released(KeyCode::A);

    let gamepad_join_inputs: HashMap<usize, bool> = gamepads
        .iter()
        .map(|gamepad| {
            (
                gamepad.id,
                gamepad_input.just_pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadRight,
                }) || gamepad_input.just_pressed(GamepadButton {
                    gamepad,
                    button_type: GamepadButtonType::DPadLeft,
                }),
            )
        })
        .collect();

    // handle player 1 selection
    let children = player_1_selection.single();
    for child in children.iter() {
        let (mut choice, mut bounce, mut bg_color) = selection_choice.get_mut(*child).unwrap();
        if let Some(input_type) = &players_resource.player_inputs[0] {
            match input_type {
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
                            players_resource.player_characters[0] = Some(choice.character.clone());
                        }
                    }
                }
                PlayerInput::Gamepad(gamepad_id) => {
                    if gamepad_join_inputs[gamepad_id] {
                        if choice.is_active {
                            choice.is_active = false;
                            bounce.is_active = false;
                            *bg_color = BackgroundColor(Color::DARK_GRAY);
                        } else {
                            choice.is_active = true;
                            bounce.is_active = true;
                            *bg_color = BackgroundColor(Color::WHITE);
                            players_resource.player_characters[0] = Some(choice.character.clone());
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
        if let Some(input_type) = &players_resource.player_inputs[1] {
            match input_type {
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
                            players_resource.player_characters[1] = Some(choice.character.clone());
                        }
                    }
                }
                PlayerInput::Gamepad(gamepad_id) => {
                    if gamepad_join_inputs[gamepad_id] {
                        if choice.is_active {
                            choice.is_active = false;
                            bounce.is_active = false;
                            *bg_color = BackgroundColor(Color::DARK_GRAY);
                        } else {
                            choice.is_active = true;
                            bounce.is_active = true;
                            *bg_color = BackgroundColor(Color::WHITE);
                            players_resource.player_characters[1] = Some(choice.character.clone());
                        }
                    }
                }
            }
        }
    }

    // set default character to the captain
    if players_resource.player_inputs[0].is_some()
        && players_resource.player_characters[0].is_none()
    {
        players_resource.player_characters[0] = Some(CharacterType::Captain);
    }

    if players_resource.player_inputs[1].is_some()
        && players_resource.player_characters[1].is_none()
    {
        players_resource.player_characters[1] = Some(CharacterType::Captain);
    }
}
