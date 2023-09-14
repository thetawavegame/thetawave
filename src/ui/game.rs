use bevy::prelude::*;
use thetawave_interface::{player::PlayersResource, states::GameCleanup};

use super::{
    level::build_level_ui,
    phase::build_phase_ui,
    player::{build_player_1_ui, build_player_2_ui},
};

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

// Fundametal UIs for for dividing screen
#[derive(Component)]
pub struct TopUI;

#[derive(Component)]
pub struct MiddleUI;

#[derive(Component)]
pub struct BottomUI;

#[derive(Component)]
pub struct BottomLeftCornerUI;

#[derive(Component)]
pub struct BottomMiddleUI;

#[derive(Component)]
pub struct BottomRightCornerUI;

#[derive(Component)]
pub struct LeftUI;

#[derive(Component)]
pub struct RightUI;

#[derive(Component)]
pub struct CenterUI;

#[derive(Component)]
pub struct TopLeftCornerUI;

#[derive(Component)]
pub struct TopMiddleUI;

#[derive(Component)]
pub struct TopRightCornerUI;

/// Initialize all ui
pub fn setup_game_ui_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
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
        .insert(GameUI)
        .insert(GameCleanup)
        .with_children(|game_ui| {
            // node for the top row of ui at the top of the window
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
                .insert(TopUI)
                .with_children(|top_ui| {
                    // node for the ui at the top left corner of the window
                    top_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(TopLeftCornerUI);

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
                        .insert(TopMiddleUI)
                        .with_children(|top_middle_ui| {
                            // build the phase ui inside the top center node
                            build_phase_ui(top_middle_ui, font.clone());
                        });

                    // node for the ui at the top right corner of the window
                    top_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(TopRightCornerUI);
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
                .insert(MiddleUI)
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
                        .insert(LeftUI)
                        .with_children(|left_ui| {
                            build_player_1_ui(left_ui, &players_resource);
                        });

                    // middle column of ui at the center of the window (over the top of the arena)
                    middle_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        })
                        .insert(CenterUI);

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
                        .insert(RightUI)
                        .with_children(|right_ui_node| {
                            build_player_2_ui(right_ui_node, &players_resource)
                        });
                });

            // node for the bottom row of ui at the bottom of the window
            game_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(13.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(BottomUI)
                .with_children(|bottom_ui| {
                    // node for the ui at the bottom left corner of the window
                    bottom_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(BottomLeftCornerUI);

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
                        .insert(BottomMiddleUI)
                        .with_children(|bottom_middle_ui| {
                            // build the level ui inside the bottom center node
                            build_level_ui(bottom_middle_ui, font.clone());
                        });

                    // node for the ui at the bottom right corner of the window
                    bottom_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(10.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLACK.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(BottomRightCornerUI);
                });
        });
}
