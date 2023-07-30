use bevy::prelude::*;

use crate::states;

use super::BouncingPromptComponent;

#[derive(Component)]
pub struct InstructionsUI;

pub fn setup_instructions_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                //size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(states::InstructionsCleanup)
        .insert(InstructionsUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load(if cfg!(feature = "arcade") {
                            "texture/instructions_54_arcade.png"
                        } else {
                            "texture/instructions_54.png"
                        })
                        .into(),
                    style: Style {
                        //size: Size::new(Val::Percent(100.), Val::Percent(100.)),
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
                                .load(if cfg!(feature = "arcade") {
                                    "texture/start_game_prompt_arcade.png"
                                } else {
                                    "texture/start_game_prompt_keyboard.png"
                                })
                                .into(),
                            style: Style {
                                //size: Size::new(Val::Px(350.0), Val::Px(87.5)),
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
