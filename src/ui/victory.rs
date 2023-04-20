use std::time::Duration;

use bevy::prelude::*;

use crate::audio::BackgroundMusicAudioChannel;
use crate::states::AppStates;
use crate::states::VictoryCleanup;
use crate::ui::BouncingPromptComponent;
use crate::ui::EndGameTransitionResource;
use bevy_kira_audio::prelude::*;

#[derive(Component)]
pub struct VictoryFadeComponent;

#[derive(Component)]
pub struct VictoryUI;

#[cfg(target_arch = "wasm32")]
pub fn victory_fade_out_system(
    mut app_state: ResMut<State<AppStates>>,
    mut rapier_config: ResMut<RapierConfiguration>,
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    mut game_fade_query: Query<&mut Sprite, With<GameFadeComponent>>,
) {
    if end_game_trans_resource.start {
        end_game_trans_resource.fade_out_timer.tick(time.delta());

        for mut fade_sprite in game_fade_query.iter_mut() {
            let alpha = (end_game_trans_resource.fade_out_speed
                * end_game_trans_resource.fade_out_timer.elapsed_secs())
            .min(1.0);

            fade_sprite.color.set_a(alpha);
        }

        if end_game_trans_resource.fade_out_timer.just_finished() {
            rapier_config.physics_pipeline_active = false;
            rapier_config.query_pipeline_active = false;
            app_state.set(AppStates::Victory).unwrap();
        }
    }
}

pub fn victory_fade_in_system(
    time: Res<Time>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    mut victory_fade_query: Query<&mut BackgroundColor, With<VictoryFadeComponent>>,
) {
    end_game_trans_resource.fade_in_timer.tick(time.delta());

    let timer_finished = end_game_trans_resource.fade_in_timer.finished();

    for mut color in victory_fade_query.iter_mut() {
        if !timer_finished {
            let alpha = (end_game_trans_resource.fade_in_speed
                * end_game_trans_resource.fade_in_timer.elapsed_secs())
            .min(1.0);

            color.0.set_a(alpha);
        } else {
            color.0.set_a(1.0);
        }
    }
}

pub fn setup_victory_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
) {
    audio_channel
        .stop()
        .fade_out(AudioTween::linear(Duration::from_secs_f32(5.0)));

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
                    image: asset_server
                        .load("texture/victory_background_54.png")
                        .into(), // not using assetsmanager as we don't load everything on the main menu
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: Color::rgba(1.0, 1.0, 1.0, 0.0).into(),
                    ..default()
                })
                .insert(VictoryFadeComponent)
                .with_children(|parent| {
                    parent
                        .spawn(ImageBundle {
                            image: asset_server
                                .load("texture/restart_game_prompt_arcade.png")
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
                            is_active: true,
                        });

                    /*
                    parent
                        .spawn(ImageBundle {
                            image: asset_server
                                .load("texture/exit_game_prompt_keyboard.png")
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
