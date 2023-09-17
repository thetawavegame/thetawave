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
pub struct AbilitySlotUI {
    pub player_index: usize,
    pub ability_index: usize,
}

#[derive(Component)]
pub struct AbilityIconUI;

#[derive(Component)]
pub struct AbilityValueUI {
    pub player_index: usize,
    pub ability_index: usize,
}

pub fn build_player_1_ui(
    parent: &mut ChildBuilder,
    players_resource: &PlayersResource,
    asset_server: &AssetServer,
) {
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
            .with_children(|player_ui| {
                player_ui
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(65.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::ColumnReverse,
                            padding: UiRect::all(Val::Percent(5.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player1LeftUI)
                    .with_children(|player_left_ui| {
                        let ability_square = asset_server.load("texture/ability_square_left.png");

                        player_left_ui
                            .spawn(ImageBundle {
                                image: ability_square.clone().into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(AbilitySlotUI {
                                player_index: 0,
                                ability_index: 0,
                            })
                            .with_children(|ability_slot_ui| {
                                ability_slot_ui.spawn(ImageBundle {
                                    image: asset_server.load(match player_data.character {
                                        thetawave_interface::character::CharacterType::Captain => "texture/blast_ability.png",
                                        thetawave_interface::character::CharacterType::Juggernaut => "texture/bullet_ability.png",
                                    }).into(),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                }).insert(AbilityIconUI).with_children(|ability_icon_ui| {
                                    
                                    ability_icon_ui
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: Color::BLACK.with_a(0.85).into(),
                                            ..default()
                                        })
                                        .insert(AbilityValueUI {
                                            player_index: 0,
                                            ability_index: 0,
                                        });
                                    
                                    });
                                
                            });

                            player_left_ui
                            .spawn(ImageBundle {
                                image: ability_square.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(AbilitySlotUI {
                                player_index: 0,
                                ability_index: 1,
                            })
                            .with_children(|ability_slot_ui| {
                                ability_slot_ui.spawn(ImageBundle {
                                    image: asset_server.load(match player_data.character {
                                        thetawave_interface::character::CharacterType::Captain => "texture/megablast_ability.png",
                                        thetawave_interface::character::CharacterType::Juggernaut => "texture/charge_ability.png",
                                    }).into(),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                }).insert(AbilityIconUI).with_children(|ability_icon_ui| {
                                    
                                    ability_icon_ui
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: Color::BLACK.with_a(0.85).into(),
                                            ..default()
                                        })
                                        .insert(AbilityValueUI {
                                            player_index: 0,
                                            ability_index: 1,
                                        });
                                
                                });
                                
                            });

                        
                    });

                player_ui
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(35.0),
                            height: Val::Percent(100.0),
                            padding: UiRect::all(Val::Percent(5.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player1RightUI)
                    .with_children(|player_right_ui| {
                        player_right_ui
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(55.0),
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

                        player_right_ui
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(25.0),
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

                        player_right_ui
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(20.0),
                                    padding: UiRect::new(
                                        Val::Percent(0.0),
                                        Val::Percent(0.0),
                                        Val::Vh(0.1),
                                        Val::Vh(0.1),
                                    ),
                                    flex_direction: FlexDirection::ColumnReverse,
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(ArmorUI(0));
                    });
            });
    }
}

pub fn build_player_2_ui(
    parent: &mut ChildBuilder,
    players_resource: &PlayersResource,
    asset_server: &AssetServer,
) {
    // if player 2 is registered spawn player 2 data ui
    if let Some(player_data) = &players_resource.player_data[1] {
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
            .insert(Player2UI)
            .with_children(|player_ui| {
                player_ui
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(35.0),
                            height: Val::Percent(100.0),
                            padding: UiRect::all(Val::Percent(5.0)),
                            flex_direction: FlexDirection::ColumnReverse,
                            ..default()  
                        },
                        ..default()
                    })
                    .insert(Player2LeftUI)
                    .with_children(|player_left_ui| {
                        
                        player_left_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(55.0),
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

                    player_left_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(25.0),
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

                    player_left_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(20.0),
                                padding: UiRect::new(
                                    Val::Percent(0.0),
                                    Val::Percent(0.0),
                                    Val::Vh(0.1),
                                    Val::Vh(0.1),
                                ),
                                flex_direction: FlexDirection::ColumnReverse,
                                ..default()
                            },
                            ..default()
                        })
                        .insert(ArmorUI(1));
                        
                    });

                player_ui
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(65.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::ColumnReverse,
                            padding: UiRect::all(Val::Percent(5.0)),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(Player2RightUI)
                    .with_children(|player_right_ui| {
                        let ability_square = asset_server.load("texture/ability_square_right.png");

                        player_right_ui
                            .spawn(ImageBundle {
                                image: ability_square.clone().into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(AbilitySlotUI {
                                player_index: 1,
                                ability_index: 0,
                            })
                            .with_children(|ability_slot_ui| {
                                ability_slot_ui.spawn(ImageBundle {
                                    image: asset_server.load(match player_data.character {
                                        thetawave_interface::character::CharacterType::Captain => "texture/blast_ability.png",
                                        thetawave_interface::character::CharacterType::Juggernaut => "texture/bullet_ability.png",
                                    }).into(),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                }).insert(AbilityIconUI).with_children(|ability_icon_ui| {
                                    
                                    ability_icon_ui
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: Color::BLACK.with_a(0.85).into(),
                                            ..default()
                                        })
                                        .insert(AbilityValueUI {
                                            player_index: 1,
                                            ability_index: 0,
                                        });
                                    
                                    });
                                
                            });

                            player_right_ui
                            .spawn(ImageBundle {
                                image: ability_square.into(),
                                style: Style {
                                    width: Val::Percent(100.0),
                                    aspect_ratio: Some(1.0),
                                    ..default()
                                },
                                ..default()
                            })
                            .insert(AbilitySlotUI {
                                player_index: 1,
                                ability_index: 1,
                            })
                            .with_children(|ability_slot_ui| {
                                ability_slot_ui.spawn(ImageBundle {
                                    image: asset_server.load(match player_data.character {
                                        thetawave_interface::character::CharacterType::Captain => "texture/megablast_ability.png",
                                        thetawave_interface::character::CharacterType::Juggernaut => "texture/charge_ability.png",
                                    }).into(),
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        aspect_ratio: Some(1.0),
                                        flex_direction: FlexDirection::Column,
                                        ..default()
                                    },
                                    ..default()
                                }).insert(AbilityIconUI).with_children(|ability_icon_ui| {
                                    
                                    ability_icon_ui
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Percent(100.0),
                                                ..default()
                                            },
                                            background_color: Color::BLACK.with_a(0.85).into(),
                                            ..default()
                                        })
                                        .insert(AbilityValueUI {
                                            player_index: 1,
                                            ability_index: 1,
                                        });
                                
                                });
                                
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
        Query<(&mut Style, &AbilityValueUI)>,
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
                                    aspect_ratio: Some(10.0),
                                    margin: UiRect::new(
                                        Val::Px(0.0),
                                        Val::Px(0.0),
                                        Val::Vh(0.1),
                                        Val::Vh(0.1),
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

        for (mut style, ability_value_ui) in player_ui.p3().iter_mut() {
            if player_index == ability_value_ui.player_index && ability_value_ui.ability_index == 0 {
                style.height = Val::Percent(100.0 * (1.0 - player_component.fire_timer.percent()));
            }
        }

        for (mut style, ability_value_ui) in player_ui.p3().iter_mut() {
            if player_index == ability_value_ui.player_index && ability_value_ui.ability_index == 1 {
                style.height =
                    Val::Percent(100.0 * (1.0 - player_component.ability_cooldown_timer.percent()));
            }
        }
    }
}
