use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl, AudioEasing, AudioTween};
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
};

use crate::states::MainMenuCleanup;
use crate::{assets::GameAudioAssets, audio, states};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct BouncingPromptComponent {
    pub flash_timer: Timer,
    pub is_active: bool,
}

pub fn setup_main_menu_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<audio::BackgroundMusicAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
    historical_games_shot_counts: Res<UserStatsByPlayerForCompletedGamesCache>,
    historical_games_enemy_mob_kill_counts: Res<MobKillsByPlayerForCompletedGames>,
) {
    let maybe_user_stats = (**historical_games_shot_counts).get(&DEFAULT_USER_ID);

    let (accuracy_rate, total_shots_fired): (f32, usize) = match maybe_user_stats {
        None => (100.0, 0),
        Some(current_game_shot_counts) => {
            let accuracy = (current_game_shot_counts.total_shots_hit as f32
                / current_game_shot_counts.total_shots_fired as f32)
                * 100.0;
            (accuracy, current_game_shot_counts.total_shots_fired)
        }
    };

    audio_channel
        .play(audio_assets.get_bg_music_asset(&crate::assets::BGMusicType::Main))
        .looped()
        .fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::Linear));

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
        .insert(MainMenuCleanup)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load("texture/main_menu_background_54.png")
                        .into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexEnd,
                        ..Default::default()
                    },
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
                                    top: Val::Percent(40.0),
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
                                        super::pprint_mob_kills_from_data(&(**historical_games_enemy_mob_kill_counts)),
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
                                    "texture/start_game_prompt_arcade.png"
                                } else {
                                    "texture/start_game_prompt_keyboard.png"
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
    /*
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            ..Default::default()
        })
        //.insert(AppStateComponent(AppStates::MainMenu))
        .insert(states::MainMenuCleanup)
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    image: asset_server
                        .load("texture/main_menu_background_54.png")
                        .into(),
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
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
                                width: Val::Px(400.),
                                height: Val::Px(100.),
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
        */
}

pub fn bouncing_prompt_system(
    mut flashing_prompt_query: Query<(&mut Transform, &mut BouncingPromptComponent)>,
    time: Res<Time>,
) {
    for (mut transform, mut prompt) in flashing_prompt_query.iter_mut() {
        if !prompt.is_active {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
            prompt.flash_timer.reset();
            continue;
        }
        prompt.flash_timer.tick(time.delta());

        let scale: f32 = -0.2 * (prompt.flash_timer.elapsed_secs() - 1.0).powf(2.0) + 1.2;

        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
