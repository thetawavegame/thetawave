use bevy::prelude::*;
use thetawave_interface::{
    health::HealthComponent,
    player::{PlayerComponent, PlayersResource},
};

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
pub struct ArmorUI(usize);

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
    // if player 1 is registered spawn player 1 data ui
    if let Some(player_data) = &players_resource.player_data[0] {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .insert(Player1UI)
            .with_children(|player1_ui| {
                player1_ui
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
                    .with_children(|player1_left_ui| {
                        player1_left_ui
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
                            .with_children(|basic_attack_ui| {
                                basic_attack_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::ORANGE.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(BasicAttackValueUI(0));
                            });

                        player1_left_ui
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
                            .with_children(|special_ability_ui| {
                                special_ability_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::GREEN.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(SpecialAbilityValueUI(0));
                            });
                    });

                player1_ui
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
                    .with_children(|player1_right_ui| {
                        player1_right_ui
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
                            .with_children(|health_ui| {
                                health_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::RED.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(HealthValueUI(0));
                            });

                        player1_right_ui
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
                            .with_children(|shields_ui| {
                                shields_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::TEAL.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(ShieldsValueUI(0));
                            });

                        player1_right_ui
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
                            .insert(ArmorUI(0))
                            .with_children(|armor_ui| {
                                /*
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
                                */
                            });
                    });
            });
    }
}

pub fn build_player_2_ui(parent: &mut ChildBuilder, players_resource: &PlayersResource) {
    // if player 1 is registered spawn player 1 data ui
    if let Some(player_data) = &players_resource.player_data[1] {
        parent
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            })
            .insert(Player2UI)
            .with_children(|player2_ui| {
                // spawn 2 player ui if registered
                //if let Some(player_data) = &players_resource.player_data[0] {
                player2_ui
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
                    .with_children(|player2_left_ui| {
                        player2_left_ui
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
                            .insert(ArmorUI(1))
                            .with_children(|armor_ui| {});

                        player2_left_ui
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
                            .with_children(|shields_ui| {
                                shields_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::TEAL.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(ShieldsValueUI(1));
                            });

                        player2_left_ui
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
                            .with_children(|health_ui| {
                                health_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::RED.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(HealthValueUI(1));
                            });
                    });

                player2_ui
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::ColumnReverse,
                            padding: UiRect::all(Val::Percent(5.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player2RightUI)
                    .with_children(|player2_right_ui| {
                        player2_right_ui
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
                            .with_children(|basic_attack_ui| {
                                basic_attack_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::ORANGE.with_a(0.75).into(),
                                        ..default()
                                    })
                                    .insert(BasicAttackValueUI(1));
                            });

                        player2_right_ui
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
                            .with_children(|special_ability_ui| {
                                special_ability_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(100.0),
                                            height: Val::Percent(100.0),
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
}

pub fn update_player_ui_system(
    mut commands: Commands,
    player_query: Query<(&HealthComponent, &PlayerComponent)>,
    mut player_ui: ParamSet<(
        Query<(&mut Style, &HealthValueUI)>,
        Query<(&mut Style, &ShieldsValueUI)>,
        Query<(Entity, &ArmorUI)>,
        Query<(&mut Style, &BasicAttackValueUI)>,
        Query<(&mut Style, &SpecialAbilityValueUI)>,
    )>,
) {
    for (player_health, player_component) in player_query.iter() {
        let player_index = player_component.player_index;

        // health ui
        for (mut style, health_value_ui) in player_ui.p0().iter_mut() {
            if player_index == health_value_ui.0 {
                style.height = Val::Percent(100.0 * player_health.get_health_percentage());
            }
        }

        // shields ui
        for (mut style, shields_value_ui) in player_ui.p1().iter_mut() {
            if player_index == shields_value_ui.0 {
                style.height = Val::Percent(100.0 * player_health.get_shields_percentage());
            }
        }

        // armor ui
        for (entity, armor_value_ui) in player_ui.p2().iter() {
            if player_index == armor_value_ui.0 {
                // spawn all of the existing child armor ticks
                commands.entity(entity).despawn_descendants();

                // spawn armor ticks
                commands.entity(entity).with_children(|armor_ui| {
                    for _ in 0..player_health.get_armor() {
                        armor_ui
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    aspect_ratio: Some(0.05),
                                    margin: UiRect::new(
                                        Val::Px(0.0),
                                        Val::Px(0.0),
                                        Val::Px(3.0),
                                        Val::Px(3.0),
                                    ),
                                    ..default()
                                },
                                background_color: Color::YELLOW.with_a(0.75).into(),
                                ..default()
                            })
                            .insert(ArmorCounterUI);
                    }
                });
            }
        }

        // basic attack ui
        for (mut style, basic_attack_ui) in player_ui.p3().iter_mut() {
            if player_index == basic_attack_ui.0 {
                style.height = Val::Percent(100.0 * player_component.fire_timer.percent());
            }
        }

        // special ability ui
        for (mut style, special_ability_ui) in player_ui.p4().iter_mut() {
            if player_index == special_ability_ui.0 {
                style.height =
                    Val::Percent(100.0 * player_component.ability_cooldown_timer.percent());
            }
        }
    }
}
