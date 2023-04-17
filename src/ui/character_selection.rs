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
                });
        });
}
