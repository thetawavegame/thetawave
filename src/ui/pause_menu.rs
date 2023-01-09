use bevy::prelude::*;

use crate::{
    states::{AppStateComponent, AppStates},
    ui::BouncingPromptComponent,
};

#[derive(Component)]
pub struct PauseUI;

pub fn setup_pause_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            background_color: Color::rgba(0.5, 0.5, 0.5, 0.1).into(),
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::PauseMenu))
        .insert(PauseUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load("texture/restart_game_prompt_keyboard.png")
                        .into(),
                    style: Style {
                        size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                        margin: UiRect {
                            left: Val::Auto,
                            right: Val::Auto,
                            top: Val::Auto,
                            bottom: Val::Auto,
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(BouncingPromptComponent {
                    flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                });
        });
}
