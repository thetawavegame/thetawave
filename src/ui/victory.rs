use std::time::Duration;

use bevy::prelude::*;

use crate::ui::BouncingPromptComponent;
use thetawave_interface::{
    audio::ChangeBackgroundMusicEvent,
    game::historical_metrics::{
        MobKillsByPlayerForCurrentGame, UserStatsByPlayerForCurrentGameCache, DEFAULT_USER_ID,
    },
    states::VictoryCleanup,
};

#[derive(Component)]
pub struct VictoryUI;

pub fn setup_victory_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
    current_game_shot_counts: Res<UserStatsByPlayerForCurrentGameCache>,
    current_game_enemy_mob_kill_counts: Res<MobKillsByPlayerForCurrentGame>,
) {
    // fade music out
    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        fade_out: Some(Duration::from_secs(5)),
        ..default()
    });

    let maybe_current_game_stats = (**current_game_shot_counts).get(&DEFAULT_USER_ID);
    let (accuracy_rate, total_shots_fired): (f32, usize) = match maybe_current_game_stats {
        None => (100.0, 0),
        Some(current_game_shot_counts) => {
            let accuracy = (current_game_shot_counts.total_shots_hit as f32
                / current_game_shot_counts.total_shots_fired as f32)
                * 100.0;
            (accuracy, current_game_shot_counts.total_shots_fired)
        }
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(VictoryCleanup)
        .insert(VictoryUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server.load("texture/victory_background.png").into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
                    background_color: Color::rgba(1.0, 1.0, 1.0, 1.0).into(),
                    ..default()
                })
                .with_children(|parent| {
                    let font = asset_server.load("fonts/wibletown-regular.otf");

                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Auto,
                                height: Val::Auto,
                                margin: UiRect {
                                    bottom: Val::Auto,
                                    top: Val::Percent(25.0),
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                padding: UiRect::all(Val::Px(10.0)),

                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            background_color: BackgroundColor::from(Color::BLACK.with_a(0.9)),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                style: Style {
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    margin: UiRect::all(Val::Auto),
                                    ..Default::default()
                                },

                                text: Text::from_section(
                                    format!(
                                        "Projectiles fired: {}\nAccuracy: {:.2}%\n\nEnemies destroyed:\n{}",
                                        total_shots_fired,
                                        accuracy_rate,
                                        super::pprint_mob_kills_from_data(
                                            &current_game_enemy_mob_kill_counts
                                        ),
                                    ),
                                    TextStyle {
                                        font,
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_alignment(TextAlignment::Center),

                                ..default()
                            });
                        });

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
                                    bottom: Val::Percent(10.0),
                                    top: Val::Auto,
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                justify_content: JustifyContent::Center,
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
