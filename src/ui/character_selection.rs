use super::BouncingPromptComponent;
use crate::states;
use bevy::prelude::*;

#[derive(Component)]
pub struct CharacterSelectionUI;

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
                                        .insert(BouncingPromptComponent {
                                            flash_timer: Timer::from_seconds(
                                                2.0,
                                                TimerMode::Repeating,
                                            ),
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
                                        .insert(BouncingPromptComponent {
                                            flash_timer: Timer::from_seconds(
                                                2.0,
                                                TimerMode::Repeating,
                                            ),
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
                                    ..Default::default()
                                })
                                .insert(BouncingPromptComponent {
                                    flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                                });
                        });
                    /*

                    */
                });
        });
}

pub fn player_join_system() {}
