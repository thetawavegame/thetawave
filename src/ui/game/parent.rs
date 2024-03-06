use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    hierarchy::BuildChildren,
    render::{color::Color, texture::Image},
    text::Font,
    ui::{
        node_bundles::NodeBundle, AlignItems, BackgroundColor, FlexDirection, JustifyContent,
        Style, Val,
    },
    utils::default,
};
use thetawave_interface::{
    abilities::AbilitySlotIDComponent,
    character::Character,
    player::{PlayerIDComponent, PlayersResource},
    states::GameCleanup,
};

use crate::{assets::UiAssets, player::CharactersResource};

const TOP_ROW_HEIGHT: Val = Val::Percent(13.0);
const TOP_CORNER_WIDTH: Val = Val::Percent(10.0);
const TOP_CENTER_WIDTH: Val = Val::Percent(80.0);
const BOTTOM_ROW_HEIGHT: Val = Val::Percent(13.0);
const BOTTOM_CORNER_WIDTH: Val = Val::Percent(10.0);
const BOTTOM_CENTER_WIDTH: Val = Val::Percent(80.0);
const OUTSIDE_BORDER_BG_COLOR: BackgroundColor = BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.75));
const MIDDLE_ROW_HEIGHT: Val = Val::Percent(74.0);
const MIDDLE_SIDE_WIDTH: Val = Val::Percent(10.0);
const MIDDLE_CENTER_WIDTH: Val = Val::Percent(80.0);

pub trait PhaseUiChildBuilderExt {
    fn spawn_phase_ui(&mut self, font: Handle<Font>);
}

pub trait LevelUiChildBuilderExt {
    fn spawn_level_ui(&mut self, font: Handle<Font>);
}

pub trait GameCenterUiChildBuilderExt {
    fn spawn_game_center_ui(&mut self, font: Handle<Font>);
}

pub trait PlayerUiChildBuilderExt {
    fn spawn_player_ui(
        &mut self,
        characters_res: &CharactersResource,
        id: PlayerIDComponent,
        players_res: &PlayersResource,
        ui_assets: &UiAssets,
    );
    fn spawn_inner_player_ui(&mut self, id: PlayerIDComponent);
    fn spawn_outer_player_ui(
        &mut self,
        character: &Character,
        id: PlayerIDComponent,
        ui_assets: &UiAssets,
    );
    fn spawn_player_ability_slot_ui(
        &mut self,
        character: &Character,
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
        is_flipped: bool,
        ui_assets: &UiAssets,
    );
    fn spawn_player_armor_counter_ui(&mut self);
    fn spawn_player_ability_icon_ui(
        &mut self,
        player_id: PlayerIDComponent,
        ability_slot_id: AbilitySlotIDComponent,
        icon: Handle<Image>,
    );
}

/// initializes the game ui hierarchy
pub fn setup_game_ui_system(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    players_resource: Res<PlayersResource>,
    characters_resource: Res<CharactersResource>,
) {
    let font: Handle<Font> = ui_assets.wibletown_font.clone();

    // Spawn the top level Node for all game ui and all of its child entities
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
        .with_children(|game| {
            // Parent node for top row containing the phase ui
            game.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: TOP_ROW_HEIGHT,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|top_row| {
                // Top left corner
                top_row.spawn(NodeBundle {
                    style: Style {
                        width: TOP_CORNER_WIDTH,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: OUTSIDE_BORDER_BG_COLOR,
                    ..default()
                });

                // Top middle
                top_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: TOP_CENTER_WIDTH,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: OUTSIDE_BORDER_BG_COLOR,
                        ..default()
                    })
                    .with_children(|top_middle| {
                        // spawn the phase ui
                        top_middle.spawn_phase_ui(font.clone());
                    });

                // Top right corner
                top_row.spawn(NodeBundle {
                    style: Style {
                        width: TOP_CORNER_WIDTH,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: OUTSIDE_BORDER_BG_COLOR,
                    ..default()
                });
            });

            // Middle Row
            game.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: MIDDLE_ROW_HEIGHT,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|middle_row| {
                // Left column on the left side of window excluding the corners
                middle_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: MIDDLE_SIDE_WIDTH,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: OUTSIDE_BORDER_BG_COLOR,
                        ..default()
                    })
                    .with_children(|middle_left| {
                        // Player 1 Ui on the left
                        middle_left.spawn_player_ui(
                            &characters_resource,
                            PlayerIDComponent::One,
                            &players_resource,
                            &ui_assets,
                        );
                    });

                // Middle column over the top of the arena
                middle_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: MIDDLE_CENTER_WIDTH,
                            height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|middle_center| {
                        // spawn the ui for displaying messages in the center of the game
                        middle_center.spawn_game_center_ui(font.clone());
                    });

                // Left column on the left side of window excluding the corners
                middle_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: MIDDLE_SIDE_WIDTH,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        background_color: OUTSIDE_BORDER_BG_COLOR,
                        ..default()
                    })
                    .with_children(|middle_right| {
                        // Player 1 Ui onm the left
                        middle_right.spawn_player_ui(
                            &characters_resource,
                            PlayerIDComponent::Two,
                            &players_resource,
                            &ui_assets,
                        );
                    });
            });

            game.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: BOTTOM_ROW_HEIGHT,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            })
            .with_children(|bottom_row| {
                // Top left corner
                bottom_row.spawn(NodeBundle {
                    style: Style {
                        width: BOTTOM_CORNER_WIDTH,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: OUTSIDE_BORDER_BG_COLOR,
                    ..default()
                });

                // Bottom middle
                bottom_row
                    .spawn(NodeBundle {
                        style: Style {
                            width: BOTTOM_CENTER_WIDTH,
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        background_color: OUTSIDE_BORDER_BG_COLOR,
                        ..default()
                    })
                    .with_children(|bottom_middle| {
                        // spawn the level ui
                        bottom_middle.spawn_level_ui(font);
                    });

                // Bottom right corner
                bottom_row.spawn(NodeBundle {
                    style: Style {
                        width: BOTTOM_CORNER_WIDTH,
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: OUTSIDE_BORDER_BG_COLOR,
                    ..default()
                });
            });
        });
}
