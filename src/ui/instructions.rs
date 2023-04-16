use bevy::prelude::*;

use crate::states;

use super::BouncingPromptComponent;

#[derive(Component)]
pub struct InstructionsUI;

pub fn setup_instructions_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(AppStateComponent(AppStates::MainMenu))
        .insert(states::InstructionsCleanup)
        .insert(InstructionsUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server.load("texture/instructions_54.png").into(),
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            image: asset_server
                                .load("texture/start_game_prompt_arcade.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(350.0), Val::Px(87.5)),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Percent(20.0),
                                    top: Val::Percent(65.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(BouncingPromptComponent {
                            flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                        });
                    /*
                    parent
                        .spawn(ImageBundle {
                            image: asset_server
                                .load("texture/exit_game_prompt_controller.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Auto,
                                    top: Val::Percent(20.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(BouncingPromptComponent {
                            flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                        });
                        */
                });
        });
}
