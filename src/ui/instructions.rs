use crate::options::PlayingOnArcadeResource;

use super::BouncingPromptComponent;
use bevy::prelude::*;
use thetawave_interface::states::InstructionsCleanup;

#[derive(Component)]
pub struct InstructionsUI;

pub fn setup_instructions_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playing_on_arcade: Res<PlayingOnArcadeResource>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(InstructionsCleanup)
        .insert(InstructionsUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load(if **playing_on_arcade {
                            "texture/instructions_54_arcade.png"
                        } else {
                            "texture/instructions_54.png"
                        })
                        .into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            image: asset_server
                                .load(if **playing_on_arcade {
                                    "texture/start_game_prompt_arcade.png"
                                } else {
                                    "texture/start_game_prompt_keyboard.png"
                                })
                                .into(),
                            style: Style {
                                width: Val::Px(350.0),
                                height: Val::Px(87.5),
                                margin: UiRect {
                                    left: Val::Auto,
                                    right: Val::Percent(18.0),
                                    top: Val::Percent(70.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(BouncingPromptComponent {
                            flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                            is_active: true,
                        });
                });
        });
}
