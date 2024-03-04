use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    render::color::Color,
    ui::{node_bundles::NodeBundle, AlignItems, FlexDirection, JustifyContent, Style, Val},
    utils::default,
};
use thetawave_interface::{
    player::{PlayerIDComponent, PlayersResource},
    states::GameCleanup,
};

use super::{
    game_center::build_center_text_ui, level::build_level_ui, phase::build_phase_ui,
    player::build_player_ui,
};

/// initializes the game ui hierarchy
pub fn setup_game_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    players_resource: Res<PlayersResource>,
) {
    let font = asset_server.load("fonts/wibletown-regular.otf");

    // top level node of all game UI
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(GameCleanup)
        .with_children(|game_ui| {
            // node for the top row of ui in the window
            game_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(13.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|top_ui| {
                    // node for the ui at the top left corner of the window
                    top_ui.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::BLACK.with_a(0.75).into(),
                        ..default()
                    });

                    // node for the ui at the center of the top row
                    top_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .with_children(|top_middle_ui| {
                            // build the phase ui inside the top center node
                            build_phase_ui(top_middle_ui, font.clone());
                        });

                    // node for the ui at the top right corner of the window
                    top_ui.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::BLACK.with_a(0.75).into(),
                        ..default()
                    });
                });

            // node for the middle row of ui in the center of the window
            game_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(74.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|middle_ui| {
                    // left column of ui at very left of the window (excluding the corners)
                    middle_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .with_children(|left_ui| {
                            // build player 1 ui
                            build_player_ui(
                                PlayerIDComponent::One,
                                left_ui,
                                &players_resource,
                                &asset_server,
                            );
                        });

                    // middle column of ui at the center of the window (over the top of the arena)
                    middle_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(100.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|center_ui| build_center_text_ui(center_ui, font.clone()));

                    // right column of ui at very right of the window (excluding the corners)
                    middle_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .with_children(|right_ui| {
                            // build player 2 ui
                            build_player_ui(
                                PlayerIDComponent::Two,
                                right_ui,
                                &players_resource,
                                &asset_server,
                            );
                        });
                });

            // node for the bottom row of ui in the window
            game_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(13.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|bottom_ui| {
                    // node for the ui at the bottom left corner of the window
                    bottom_ui.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::BLACK.with_a(0.75).into(),
                        ..default()
                    });

                    // node for the ui at the center of the bottom row
                    bottom_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(100.0),
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .with_children(|bottom_middle_ui| {
                            // build the level ui inside the bottom center node
                            build_level_ui(bottom_middle_ui, font.clone());
                        });

                    // node for the ui at the bottom right corner of the window
                    bottom_ui.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(10.0),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::BLACK.with_a(0.75).into(),
                        ..default()
                    });
                });
        });
}
