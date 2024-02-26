use crate::options::PlayingOnArcadeResource;
use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        event::EventWriter,
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::{JustifyText, Text, TextStyle},
    time::{Time, Timer},
    transform::components::Transform,
    ui::{
        node_bundles::{ButtonBundle, ImageBundle, NodeBundle, TextBundle},
        AlignItems, BackgroundColor, BorderColor, FlexDirection, JustifyContent, Style, UiRect,
        Val,
    },
    utils::default,
};
use std::time::Duration;
use thetawave_interface::audio::{BGMusicType, ChangeBackgroundMusicEvent};
use thetawave_interface::game::historical_metrics::{
    MobKillsByPlayerForCompletedGames, UserStatsByPlayerForCompletedGamesCache, DEFAULT_USER_ID,
};
use thetawave_interface::states::MainMenuCleanup;

use super::button::UIChildBuilderExt;

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
    mut change_bg_music_event_writer: EventWriter<ChangeBackgroundMusicEvent>,
    historical_games_shot_counts: Res<UserStatsByPlayerForCompletedGamesCache>,
    historical_games_enemy_mob_kill_counts: Res<MobKillsByPlayerForCompletedGames>,
    playing_on_arcade: Res<PlayingOnArcadeResource>,
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

    change_bg_music_event_writer.send(ChangeBackgroundMusicEvent {
        bg_music_type: Some(BGMusicType::Main),
        loop_from: Some(0.0),
        fade_in: Some(Duration::from_secs(2)),
        fade_out: Some(Duration::from_secs(2)),
    });

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

                    /*
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
                                        super::pprint_mob_kills_from_data(&historical_games_enemy_mob_kill_counts),
                                    ),
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 32.0,
                                        color: Color::WHITE,
                                    },
                                )
                                    .with_justify(JustifyText::Center),

                                ..default()
                            });
                        });
                        */
                    /*
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(20.0),
                                max_width: Val::Px(300.0),
                                min_height: Val::Percent(5.0),
                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                /* */
                                margin: UiRect {
                                    bottom: Val::Percent(3.0),
                                    top: Val::Percent(50.0),
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                ..default()
                            },
                            border_color: BorderColor(Color::RED),
                            background_color: BackgroundColor(Color::GREEN),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                        */

                    parent.spawn_menu_button(String::from("Test"));

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(20.0),
                                max_width: Val::Px(300.0),
                                min_height: Val::Percent(5.0),
                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                /* */
                                margin: UiRect {
                                    bottom: Val::Percent(3.0),
                                    top: Val::Auto,
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                ..default()
                            },
                            border_color: BorderColor(Color::RED),
                            background_color: BackgroundColor(Color::GREEN),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Compendium",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(20.0),
                                max_width: Val::Px(300.0),
                                min_height: Val::Percent(5.0),
                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                /* */
                                margin: UiRect {
                                    bottom: Val::Percent(3.0),
                                    top: Val::Auto,
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                ..default()
                            },
                            border_color: BorderColor(Color::RED),
                            background_color: BackgroundColor(Color::GREEN),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Options",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });

                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Percent(20.0),
                                max_width: Val::Px(300.0),
                                min_height: Val::Percent(5.0),
                                border: UiRect::all(Val::Px(5.0)),
                                // horizontally center child text
                                justify_content: JustifyContent::Center,
                                // vertically center child text
                                align_items: AlignItems::Center,
                                /* */
                                margin: UiRect {
                                    bottom: Val::Percent(5.0),
                                    top: Val::Auto,
                                    right: Val::Auto,
                                    left: Val::Auto,
                                },
                                ..default()
                            },
                            border_color: BorderColor(Color::RED),
                            background_color: BackgroundColor(Color::GREEN),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Quit",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ));
                        });
                });
        });
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
