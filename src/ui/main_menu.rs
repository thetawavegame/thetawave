use crate::{
    animation::{AnimationComponent, AnimationDirection},
    assets::UiAssets,
};
use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        component::Component,
        event::EventWriter,
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    render::color::Color,
    text::Font,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
    ui::{
        node_bundles::{AtlasImageBundle, NodeBundle},
        AlignItems, FlexDirection, JustifyContent, Style, Val,
    },
    utils::default,
};
use std::time::Duration;
use thetawave_interface::audio::{BGMusicType, ChangeBackgroundMusicEvent};
use thetawave_interface::states::MainMenuCleanup;

use super::button::{MenuButtonActionComponent, UiChildBuilderExt};

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
    ui_assets: Res<UiAssets>,
) {
    let font: Handle<Font> = asset_server.load("fonts/Lunchds.ttf");

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
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(AtlasImageBundle {
                                    style: Style {
                                        max_width: Val::Px(900.0),
                                        width: Val::Percent(70.0),
                                        min_width: Val::Px(300.0),
                                        aspect_ratio: Some(1920.0 / 1080.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    image: ui_assets.thetawave_logo_image.clone().into(),
                                    texture_atlas: ui_assets.thetawave_logo_layout.clone().into(),
                                    ..default()
                                })
                                .insert(AnimationComponent {
                                    timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                                    direction: AnimationDirection::Forward,
                                });
                        });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Percent(50.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn_menu_button(
                                &ui_assets,
                                "Start Game".to_string(),
                                font.clone(),
                                MenuButtonActionComponent::EnterInstructions,
                            );
                            parent.spawn_menu_button(
                                &ui_assets,
                                "Compendium".to_string(),
                                font.clone(),
                                MenuButtonActionComponent::EnterCompendium,
                            );
                            parent.spawn_menu_button(
                                &ui_assets,
                                "Options".to_string(),
                                font.clone(),
                                MenuButtonActionComponent::EnterOptions,
                            );
                            parent.spawn_menu_button(
                                &ui_assets,
                                "Quit".to_string(),
                                font.clone(),
                                MenuButtonActionComponent::QuitGame,
                            );
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
