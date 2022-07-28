use bevy::prelude::*;

use crate::states::{AppStateComponent, AppStates};

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct BouncingPromptComponent {
    pub flash_timer: Timer,
}

pub fn setup_main_menu_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            visibility: Visibility { is_visible: false },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::MainMenu))
        .insert(MainMenuUI)
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    image: asset_server.load("texture/main_menu_background.png").into(), // not using assetsmanager as we don't load everything on the main menu
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(ImageBundle {
                            image: asset_server
                                .load("texture/start_game_prompt_keyboard.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                margin: Rect {
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
                            flash_timer: Timer::from_seconds(2.0, true),
                        });

                    parent
                        .spawn_bundle(ImageBundle {
                            image: asset_server
                                .load("texture/exit_game_prompt_keyboard.png")
                                .into(),
                            style: Style {
                                size: Size::new(Val::Px(400.0), Val::Px(100.0)),
                                margin: Rect {
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
                            flash_timer: Timer::from_seconds(2.0, true),
                        });
                });
        });
}

pub fn bouncing_prompt_system(
    mut flashing_prompt_query: Query<(&mut Transform, &mut BouncingPromptComponent)>,
    time: Res<Time>,
) {
    for (mut transform, mut prompt) in flashing_prompt_query.iter_mut() {
        prompt.flash_timer.tick(time.delta());

        let scale: f32 = -0.2 * (prompt.flash_timer.elapsed_secs() - 1.0).powf(2.0) + 1.2;

        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
