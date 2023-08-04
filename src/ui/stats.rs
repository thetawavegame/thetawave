use bevy::prelude::*;
use thetawave_interface::game::historical_metrics::MobKillsByPlayerForCompletedGames;

use crate::states;

use super::BouncingPromptComponent;

#[derive(Component)]
pub struct StatsUI;

pub fn setup_stats_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    historical_games_enemy_mob_kill_counts: Res<MobKillsByPlayerForCompletedGames>,
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
        .insert(states::StatsCleanup)
        .insert(StatsUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server.load("texture/black_54.png").into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    let font = asset_server.load("fonts/wibletown-regular.otf");

                    parent.spawn(TextBundle {
                        style: Style {
                            left: Val::Percent(5.0),
                            bottom: Val::Percent(25.0),

                            position_type: PositionType::Absolute,
                            ..Style::default()
                        },
                        text: Text::from_section(
                            format!(
                                "Mobs destroyed:\n{}",
                                super::pprint_mob_kills_from_data(
                                    &(**historical_games_enemy_mob_kill_counts)
                                ),
                            ),
                            TextStyle {
                                font: font.clone(),
                                font_size: 18.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                    /*
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
                                    top: Val::Percent(20.0),
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
                        */
                });
        });
}
