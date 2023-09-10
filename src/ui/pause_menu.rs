use bevy::prelude::*;
use thetawave_interface::states::PauseCleanup;

use crate::ui::BouncingPromptComponent;

#[derive(Component)]
pub struct PauseUI;

pub fn setup_pause_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            background_color: Color::rgba(0.5, 0.5, 0.5, 0.1).into(),
            ..Default::default()
        })
        .insert(PauseCleanup)
        .insert(PauseUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load(if cfg!(feature = "arcade") {
                            "texture/restart_game_prompt_arcade.png"
                        } else {
                            "texture/restart_game_prompt_keyboard.png"
                        })
                        .into(),
                    style: Style {
                        width: Val::Px(400.0),
                        height: Val::Px(100.0),
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
                    is_active: true,
                });
        });
}
