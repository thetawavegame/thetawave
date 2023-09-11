use std::time::Duration;

use bevy::prelude::*;
use thetawave_interface::{
    health::HealthComponent,
    objective::{NewObjectiveEvent, Objective},
    player::{PlayerComponent, PlayersResource},
    states::GameCleanup,
};

use crate::run::CurrentRunProgressResource;

use super::BouncingPromptComponent;

/// Tag for level ui
#[derive(Component)]
pub struct ObjectiveUI;

/// Tag for level ui
#[derive(Component)]
pub struct ObjectiveLabelUI;

/// Tag for level ui
#[derive(Component)]
pub struct PowerGlowUI(Timer);

#[derive(Component)]
pub struct StatBarLabel;

#[derive(Component)]
pub struct AbilityChargingUI;

#[derive(Component)]
pub struct AbilityReadyUI;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct PhaseUiComponent;

#[derive(Component)]
pub struct TutorialPhaseUI;

// Player data Uis
#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]
pub struct HealthValueUI(usize);

#[derive(Component)]
pub struct ShieldsUI;

#[derive(Component)]
pub struct ShieldsValueUI(usize);

#[derive(Component)]
pub struct ArmorUI;

#[derive(Component)]
pub struct ArmorCounterUI;

#[derive(Component)]
pub struct BasicAttackUI;

#[derive(Component)]
pub struct BasicAttackValueUI(usize);

#[derive(Component)]
pub struct SpecialAbilityUI;

#[derive(Component)]
pub struct SpecialAbilityValueUI(usize);

// Player UIs on the sides
#[derive(Component)]
pub struct Player1UI;

#[derive(Component)]
pub struct Player2UI;

// Sides of each of the player uis
#[derive(Component)]
pub struct Player1RightUI;

#[derive(Component)]
pub struct Player1LeftUI;

#[derive(Component)]
pub struct Player2RightUI;

#[derive(Component)]
pub struct Player2LeftUI;

// Fundametal UIs for for dividing screen
#[derive(Component)]
pub struct TopUI;

#[derive(Component)]
pub struct MiddleUI;

#[derive(Component)]
pub struct BottomUI;

#[derive(Component)]
pub struct LeftUI;

#[derive(Component)]
pub struct RightUI;

#[derive(Component)]
pub struct CenterUI;

/// Initialize objective ui when objective changes
pub fn setup_level_objective_ui_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut new_objective_event_reader: EventReader<NewObjectiveEvent>,
    mut objective_ui_query: Query<Entity, With<ObjectiveUI>>,
    mut objective_label_ui_query: Query<Entity, With<ObjectiveLabelUI>>,
) {
    /*
    // read event for new objective set
    for event in new_objective_event_reader.iter() {
        //remove existing objective ui
        for entity in objective_ui_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in objective_label_ui_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        //create ui for new objective
        if let Some(objective) = &event.objective {
            match objective {
                Objective::Defense(_) => {
                    // level objective ui
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(800.0),
                                height: Val::Px(30.0),
                                left: Val::Percent(19.0),
                                bottom: Val::Percent(2.0),
                                position_type: PositionType::Absolute,
                                ..Style::default()
                            },
                            background_color: Color::BLUE.into(),
                            ..NodeBundle::default()
                        })
                        .insert(GameCleanup)
                        .insert(ObjectiveUI);

                    commands
                        .spawn(ImageBundle {
                            image: asset_server.load("texture/defense_bar_label.png").into(),
                            style: Style {
                                left: Val::Percent(42.5),
                                bottom: Val::Percent(1.7),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        .insert(GameCleanup)
                        .insert(ObjectiveLabelUI)
                        .insert(StatBarLabel);
                }
            }
        }
    }
    */
}

/// Initialize all ui
pub fn setup_game_ui_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    players_resource: Res<PlayersResource>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            //background_color: Color::BLACK.with_a(0.8).into(),
            ..default()
        })
        .insert(GameUI)
        .insert(GameCleanup)
        .with_children(|game_ui_node| {
            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(16.0),
                        ..default()
                    },
                    //background_color: Color::WHITE.with_a(0.25).into(),
                    ..default()
                })
                .insert(TopUI);

            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(68.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    //background_color: Color::ANTIQUE_WHITE.with_a(0.25).into(),
                    ..default()
                })
                .insert(MiddleUI)
                .with_children(|middle_ui_node| {
                    middle_ui_node
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            //background_color: Color::GREEN.with_a(0.25).into(),
                            ..default()
                        })
                        .insert(LeftUI)
                        .with_children(|left_ui_node| {
                            left_ui_node
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    //background_color: Color::RED.with_a(0.1).into(),
                                    ..default()
                                })
                                .insert(Player1UI)
                                .with_children(|player1_ui_node| {
                                    // if player 1 is registered spawn player 1 data ui
                                    if let Some(player_data) = &players_resource.player_data[0] {
                                        player1_ui_node
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(50.0),
                                                    height: Val::Percent(100.0),
                                                    flex_direction: FlexDirection::ColumnReverse,
                                                    padding: UiRect::all(Val::Percent(5.0)),
                                                    ..default()
                                                },
                                                //background_color: Color::PURPLE.with_a(0.25).into(),
                                                ..default()
                                            })
                                            .insert(Player1LeftUI)
                                            .with_children(|player1_left_node| {
                                                player1_left_node
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(100.0),
                                                            aspect_ratio: Some(1.0),
                                                            margin: UiRect::new(
                                                                Val::Px(0.0),
                                                                Val::Px(0.0),
                                                                Val::Px(10.0),
                                                                Val::Px(10.0),
                                                            ),
                                                            flex_direction:
                                                                FlexDirection::ColumnReverse,
                                                            ..default()
                                                        },
                                                        background_color: Color::ORANGE
                                                            .with_a(0.05)
                                                            .into(),
                                                        ..default()
                                                    })
                                                    .insert(BasicAttackUI)
                                                    .with_children(|basic_attack_node| {
                                                        basic_attack_node
                                                            .spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(100.0),
                                                                    height: Val::Percent(80.0),
                                                                    ..default()
                                                                },
                                                                background_color: Color::ORANGE
                                                                    .with_a(0.75)
                                                                    .into(),
                                                                ..default()
                                                            })
                                                            .insert(BasicAttackValueUI(0));
                                                    });

                                                player1_left_node
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(100.0),
                                                            aspect_ratio: Some(1.0),
                                                            margin: UiRect::new(
                                                                Val::Px(0.0),
                                                                Val::Px(0.0),
                                                                Val::Px(10.0),
                                                                Val::Px(10.0),
                                                            ),
                                                            flex_direction:
                                                                FlexDirection::ColumnReverse,
                                                            ..default()
                                                        },
                                                        background_color: Color::GREEN
                                                            .with_a(0.05)
                                                            .into(),
                                                        ..default()
                                                    })
                                                    .insert(SpecialAbilityUI)
                                                    .with_children(|special_ability_node| {
                                                        special_ability_node
                                                            .spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(100.0),
                                                                    height: Val::Percent(40.0),
                                                                    ..default()
                                                                },
                                                                background_color: Color::GREEN
                                                                    .with_a(0.75)
                                                                    .into(),
                                                                ..default()
                                                            })
                                                            .insert(SpecialAbilityValueUI(0));
                                                    });
                                            });

                                        player1_ui_node
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(50.0),
                                                    height: Val::Percent(100.0),
                                                    padding: UiRect::all(Val::Percent(5.0)),
                                                    flex_direction: FlexDirection::Row,
                                                    ..default()
                                                },
                                                //background_color: Color::RED.with_a(0.25).into(),
                                                ..default()
                                            })
                                            .insert(Player1RightUI)
                                            .with_children(|player1_right_node| {
                                                player1_right_node
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(45.0),
                                                            height: Val::Percent(100.0),
                                                            flex_direction:
                                                                FlexDirection::ColumnReverse,
                                                            ..default()
                                                        },
                                                        background_color: Color::RED
                                                            .with_a(0.05)
                                                            .into(),
                                                        ..default()
                                                    })
                                                    .insert(HealthUI)
                                                    .with_children(|health_ui_node| {
                                                        health_ui_node
                                                            .spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(100.0),
                                                                    height: Val::Percent(80.0),
                                                                    ..default()
                                                                },
                                                                background_color: Color::RED
                                                                    .with_a(0.75)
                                                                    .into(),
                                                                ..default()
                                                            })
                                                            .insert(HealthValueUI(0));
                                                    });

                                                player1_right_node
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(30.0),
                                                            height: Val::Percent(100.0),
                                                            flex_direction:
                                                                FlexDirection::ColumnReverse,
                                                            ..default()
                                                        },
                                                        background_color: Color::TEAL
                                                            .with_a(0.05)
                                                            .into(),
                                                        ..default()
                                                    })
                                                    .insert(ShieldsUI)
                                                    .with_children(|shields_ui_node| {
                                                        shields_ui_node
                                                            .spawn(NodeBundle {
                                                                style: Style {
                                                                    width: Val::Percent(100.0),
                                                                    height: Val::Percent(50.0),
                                                                    ..default()
                                                                },
                                                                background_color: Color::TEAL
                                                                    .with_a(0.75)
                                                                    .into(),
                                                                ..default()
                                                            })
                                                            .insert(ShieldsValueUI(0));
                                                    });

                                                player1_right_node
                                                    .spawn(NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(25.0),
                                                            height: Val::Percent(100.0),
                                                            padding: UiRect::new(
                                                                Val::Percent(8.0),
                                                                Val::Percent(8.0),
                                                                Val::Percent(0.0),
                                                                Val::Percent(0.0),
                                                            ),
                                                            flex_direction:
                                                                FlexDirection::ColumnReverse,
                                                            ..default()
                                                        },
                                                        ..default()
                                                    })
                                                    .insert(ArmorUI)
                                                    .with_children(|armor_ui_node| {
                                                        for _ in 0..20 {
                                                            armor_ui_node
                                                                .spawn(NodeBundle {
                                                                    style: Style {
                                                                        width: Val::Percent(100.0),
                                                                        aspect_ratio: Some(0.2),
                                                                        margin: UiRect::new(
                                                                            Val::Px(0.0),
                                                                            Val::Px(0.0),
                                                                            Val::Px(3.0),
                                                                            Val::Px(3.0),
                                                                        ),
                                                                        ..default()
                                                                    },
                                                                    background_color: Color::YELLOW
                                                                        .with_a(1.0)
                                                                        .into(),
                                                                    ..default()
                                                                })
                                                                .insert(ArmorCounterUI);
                                                        }
                                                    });
                                            });
                                    }
                                });
                        });

                    middle_ui_node
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            //background_color: Color::YELLOW.with_a(0.25).into(),
                            ..default()
                        })
                        .insert(CenterUI);

                    middle_ui_node
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            //background_color: Color::GREEN.with_a(0.25).into(),
                            ..default()
                        })
                        .insert(RightUI)
                        .with_children(|right_ui_node| {
                            right_ui_node
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        ..default()
                                    },
                                    //background_color: Color::BLUE.with_a(0.1).into(),
                                    ..default()
                                })
                                .insert(Player2UI);
                        });
                });

            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(16.0),
                        ..default()
                    },
                    //background_color: Color::WHITE.with_a(0.25).into(),
                    ..default()
                })
                .insert(BottomUI);

            /*
            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(13.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::RED.with_a(0.8).into(),
                    ..default()
                })
                .insert(Player1UI);

            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(74.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::YELLOW.with_a(0.8).into(),
                    ..default()
                })
                .insert(Player2UI);

            game_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(13.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::BLUE.with_a(0.8).into(),
                    ..default()
                })
                .insert(Player2UI);
            */
        });

    /*
    // player 1 ui
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(80.0),
                height: Val::Px(15.0),
                left: Val::Percent(1.5),
                bottom: Val::Percent(65.0),
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::PURPLE.into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(AbilityUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/ability_charging.png").into(),
            style: Style {
                left: Val::Percent(0.3),
                bottom: Val::Percent(63.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(AbilityChargingUI)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/ability_ready.png").into(),
            style: Style {
                left: Val::Percent(1.5),
                bottom: Val::Percent(65.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(AbilityReadyUI)
        .insert(BouncingPromptComponent {
            flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            is_active: true,
        })
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(15.0),
                height: Val::Px(200.0),
                left: Val::Percent(3.5),
                bottom: Val::Percent(67.0),
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::RED.into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(HealthUI)
        .insert(Player1UI);

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Px(15.0),
                height: Val::Px(200.0),
                left: Val::Percent(4.5),
                bottom: Val::Percent(67.0),
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::Rgba {
                red: 0.0,
                green: 0.74,
                blue: 1.0,
                alpha: 0.5,
            }
            .into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(ShieldsUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/health_bar_label.png").into(),
            style: Style {
                left: Val::Percent(3.5),
                bottom: Val::Percent(71.5),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/armor_spritesheet.png").into(),
            style: Style {
                width: Val::Px(10.0),
                height: Val::Px(10.0),
                left: Val::Percent(4.2),
                bottom: Val::Percent(90.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(6.0, 6.0, 1.0)),
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(ArmorUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_container.png").into(),
            style: Style {
                left: Val::Percent(4.5),
                bottom: Val::Percent(55.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_glow.png").into(),
            style: Style {
                left: Val::Percent(4.5),
                bottom: Val::Percent(55.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(PowerGlowUI(Timer::new(
            Duration::from_secs_f32(2.0),
            TimerMode::Repeating,
        )))
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_label.png").into(),
            style: Style {
                left: Val::Percent(3.0),
                bottom: Val::Percent(49.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.3, 1.3, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    // player 2 ui if there is a second player
    if players_resource.player_data[1].is_some() {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(80.0),
                    height: Val::Px(15.0),
                    left: Val::Percent(91.5),
                    bottom: Val::Percent(65.0),
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                background_color: Color::PURPLE.into(),
                ..NodeBundle::default()
            })
            .insert(GameCleanup)
            .insert(AbilityUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/ability_charging.png").into(),
                style: Style {
                    left: Val::Percent(90.5),
                    bottom: Val::Percent(63.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(AbilityChargingUI)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/ability_ready.png").into(),
                style: Style {
                    left: Val::Percent(91.5),
                    bottom: Val::Percent(65.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(AbilityReadyUI)
            .insert(BouncingPromptComponent {
                flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                is_active: true,
            })
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                    left: Val::Percent(94.5),
                    bottom: Val::Percent(67.0),
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                background_color: Color::RED.into(),
                ..NodeBundle::default()
            })
            .insert(GameCleanup)
            .insert(HealthUI)
            .insert(Player2UI);

        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                    left: Val::Percent(93.5),
                    bottom: Val::Percent(67.0),
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                background_color: Color::Rgba {
                    red: 0.0,
                    green: 0.74,
                    blue: 1.0,
                    alpha: 0.5,
                }
                .into(),
                ..NodeBundle::default()
            })
            .insert(GameCleanup)
            .insert(ShieldsUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/health_bar_label.png").into(),
                style: Style {
                    left: Val::Percent(94.5),
                    bottom: Val::Percent(71.5),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/armor_spritesheet.png").into(),
                style: Style {
                    width: Val::Px(10.0),
                    height: Val::Px(10.0),
                    left: Val::Percent(94.3),
                    bottom: Val::Percent(90.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(6.0, 6.0, 1.0)),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(ArmorUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_container.png").into(),
                style: Style {
                    left: Val::Percent(93.5),
                    bottom: Val::Percent(55.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_glow.png").into(),
                style: Style {
                    left: Val::Percent(93.5),
                    bottom: Val::Percent(55.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(PowerGlowUI(Timer::new(
                Duration::from_secs_f32(2.0),
                TimerMode::Repeating,
            )))
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_label.png").into(),
                style: Style {
                    left: Val::Percent(92.5),
                    bottom: Val::Percent(49.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(1.3, 1.3, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);
    }
    */
}
/*
#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_player1_ui(
    mut player1_ui_queries: ParamSet<(
        Query<&mut Style, (With<HealthUI>, With<Player1UI>)>,
        Query<&mut BackgroundColor, (With<ArmorUI>, With<Player1UI>)>,
        Query<(&mut BackgroundColor, &mut Transform, &mut PowerGlowUI), With<Player1UI>>,
        Query<&mut Style, (With<AbilityUI>, With<Player1UI>)>,
        Query<&mut Visibility, (With<AbilityChargingUI>, With<Player1UI>)>,
        Query<&mut Visibility, (With<AbilityReadyUI>, With<Player1UI>)>,
        Query<&mut Style, With<ObjectiveUI>>,
        Query<&mut Style, (With<ShieldsUI>, With<Player1UI>)>,
    )>,
    player_query: Query<(&HealthComponent, &PlayerComponent)>,
    run_resource: Res<CurrentRunProgressResource>,
    time: Res<Time>,
) {
    // update player health ui

    for mut style_component in player1_ui_queries.p0().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                style_component.height = Val::Px(200.0 * health_component.get_health_percentage())
            }
        }
    }

    for mut style_component in player1_ui_queries.p7().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                style_component.height = Val::Px(200.0 * health_component.get_shields_percentage())
            }
        }
    }

    for mut style_component in player1_ui_queries.p6().iter_mut() {
        if let Some(level) = &run_resource.current_level {
            if let Some(objective) = &level.objective {
                match objective {
                    Objective::Defense(data) => {
                        style_component.width = Val::Px(800.0 * data.get_percentage())
                    }
                }
            }
        }
    }

    for mut ui_color in player1_ui_queries.p1().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                if health_component.get_armor() > 0 {
                    ui_color.0.set_a(1.0);
                } else {
                    ui_color.0.set_a(0.2);
                }
            }
        }
    }

    for (mut ui_color, mut transform, mut power_glow) in player1_ui_queries.p2().iter_mut() {
        power_glow.0.tick(time.delta());
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                let new_scale = (3.0 * (player_component.money as f32 / 25.0).min(25.0))
                    + (0.2 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin())
                    + 0.2;
                transform.scale = Vec3::new(new_scale, new_scale, 1.0);
                ui_color.0.set_a(
                    (0.5 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin()) + 0.5,
                );
            }
        }
    }

    // update player ability ui
    for mut style_component in player1_ui_queries.p3().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                style_component.width = Val::Px(80.0 * cooldown_ratio);
            }
        }
    }

    for mut visibility_component in player1_ui_queries.p4().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Hidden;
                } else {
                    *visibility_component = Visibility::Visible;
                }
            }
        }
    }

    for mut visibility_component in player1_ui_queries.p5().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Visible;
                } else {
                    *visibility_component = Visibility::Hidden;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_player2_ui(
    mut player2_ui_queries: ParamSet<(
        Query<&mut Style, (With<HealthUI>, With<Player2UI>)>,
        Query<&mut BackgroundColor, (With<ArmorUI>, With<Player2UI>)>,
        Query<(&mut BackgroundColor, &mut Transform, &mut PowerGlowUI), With<Player2UI>>,
        Query<&mut Style, (With<AbilityUI>, With<Player2UI>)>,
        Query<&mut Visibility, (With<AbilityChargingUI>, With<Player2UI>)>,
        Query<&mut Visibility, (With<AbilityReadyUI>, With<Player2UI>)>,
        Query<&mut Style, (With<ShieldsUI>, With<Player2UI>)>,
    )>,
    player_query: Query<(&HealthComponent, &PlayerComponent)>,
    time: Res<Time>,
) {
    // update player health ui

    for mut style_component in player2_ui_queries.p0().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                style_component.height = Val::Px(200.0 * health_component.get_health_percentage())
            }
        }
    }

    for mut style_component in player2_ui_queries.p6().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                style_component.height = Val::Px(200.0 * health_component.get_shields_percentage())
            }
        }
    }

    for mut ui_color in player2_ui_queries.p1().iter_mut() {
        for (health_component, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                if health_component.get_armor() > 0 {
                    ui_color.0.set_a(1.0);
                } else {
                    ui_color.0.set_a(0.2);
                }
            }
        }
    }

    for (mut ui_color, mut transform, mut power_glow) in player2_ui_queries.p2().iter_mut() {
        power_glow.0.tick(time.delta());
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                let new_scale = (3.0 * (player_component.money as f32 / 25.0).min(25.0))
                    + (0.2 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin())
                    + 0.2;
                transform.scale = Vec3::new(new_scale, new_scale, 1.0);
                ui_color.0.set_a(
                    (0.5 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin()) + 0.5,
                );
            }
        }
    }

    // update player ability ui
    for mut style_component in player2_ui_queries.p3().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                style_component.width = Val::Px(80.0 * cooldown_ratio);
            }
        }
    }

    for mut visibility_component in player2_ui_queries.p4().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Hidden;
                } else {
                    *visibility_component = Visibility::Visible;
                }
            }
        }
    }

    for mut visibility_component in player2_ui_queries.p5().iter_mut() {
        for (_, player_component) in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Visible;
                } else {
                    *visibility_component = Visibility::Hidden;
                }
            }
        }
    }
}
*/
