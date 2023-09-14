use bevy::prelude::*;
use thetawave_interface::player::PlayersResource;

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

pub fn build_player_1_ui(parent: &mut ChildBuilder, players_resource: &PlayersResource) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            background_color: Color::BLACK.with_a(0.75).into(),
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
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                background_color: Color::ORANGE.with_a(0.05).into(),
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
                                        background_color: Color::ORANGE.with_a(0.75).into(),
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
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                background_color: Color::GREEN.with_a(0.05).into(),
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
                                        background_color: Color::GREEN.with_a(0.75).into(),
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
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                background_color: Color::RED.with_a(0.05).into(),
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
                                        background_color: Color::RED.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(HealthValueUI(0));
                            });

                        player1_right_node
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(30.0),
                                    height: Val::Percent(100.0),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                background_color: Color::TEAL.with_a(0.05).into(),
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
                                        background_color: Color::TEAL.with_a(0.75).into(),
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
                                    flex_direction: FlexDirection::ColumnReverse,
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
                                            background_color: Color::YELLOW.with_a(1.0).into(),
                                            ..default()
                                        })
                                        .insert(ArmorCounterUI);
                                }
                            });
                    });
            }
        });
}

pub fn build_player_2_ui(parent: &mut ChildBuilder, players_resource: &PlayersResource) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            //background_color: Color::BLUE.with_a(0.1).into(),
            ..default()
        })
        .insert(Player2UI)
        .with_children(|player2_ui_node| {
            // spawn 2 player ui if registered
            //if let Some(player_data) = &players_resource.player_data[0] {
            player2_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::all(Val::Percent(5.0)),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    //background_color: Color::PURPLE.with_a(0.25).into(),
                    ..default()
                })
                .insert(Player2LeftUI)
                .with_children(|player2_left_node| {
                    player2_left_node
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
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ArmorUI)
                        .with_children(|armor_ui_node| {
                            for _ in 0..5 {
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
                                        background_color: Color::YELLOW.with_a(1.0).into(),
                                        ..default()
                                    })
                                    .insert(ArmorCounterUI);
                            }
                        });

                    player2_left_node
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            background_color: Color::TEAL.with_a(0.05).into(),
                            ..default()
                        })
                        .insert(ShieldsUI)
                        .with_children(|shields_ui_node| {
                            shields_ui_node
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(80.0),
                                        ..default()
                                    },
                                    background_color: Color::TEAL.with_a(0.75).into(),
                                    ..default()
                                })
                                .insert(ShieldsValueUI(1));
                        });

                    player2_left_node
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(45.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            background_color: Color::RED.with_a(0.05).into(),
                            ..default()
                        })
                        .insert(HealthUI)
                        .with_children(|health_ui_node| {
                            health_ui_node
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(15.0),
                                        ..default()
                                    },
                                    background_color: Color::RED.with_a(0.75).into(),
                                    ..default()
                                })
                                .insert(HealthValueUI(1));
                        });
                });

            player2_ui_node
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: UiRect::all(Val::Percent(5.0)),
                        ..default()
                    },
                    //background_color: Color::RED.with_a(0.25).into(),
                    ..default()
                })
                .insert(Player2RightUI)
                .with_children(|player2_right_node| {
                    player2_right_node
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
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            background_color: Color::ORANGE.with_a(0.05).into(),
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
                                    background_color: Color::ORANGE.with_a(0.75).into(),
                                    ..default()
                                })
                                .insert(BasicAttackValueUI(1));
                        });

                    player2_right_node
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
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            background_color: Color::GREEN.with_a(0.05).into(),
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
                                    background_color: Color::GREEN.with_a(0.75).into(),
                                    ..default()
                                })
                                .insert(SpecialAbilityValueUI(1));
                        });
                });
        });
}
