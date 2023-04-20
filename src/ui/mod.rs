use std::time::Duration;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    player::{PlayerComponent, PlayersResource},
    run::RunResource,
    states::{self, AppStates, GameCleanup},
    GameEnterSet, GameUpdateSet,
};

mod character_selection;
mod debug;
mod game_over;
mod instructions;
mod main_menu;
mod pause_menu;
mod victory;

use self::character_selection::{
    player_join_system, select_character_system, setup_character_selection_system,
};
use self::instructions::setup_instructions_system;
pub use self::{
    debug::game_debug_ui,
    game_over::{
        fade_out_system, game_over_fade_in_system, setup_game_over_system,
        EndGameTransitionResource, GameFadeComponent,
    },
    main_menu::{bouncing_prompt_system, setup_main_menu_system, BouncingPromptComponent},
    pause_menu::setup_pause_system,
    victory::{setup_victory_system, victory_fade_in_system},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EndGameTransitionResource::new(
            2.0, 3.0, 2.5, 0.5, 0.5, 30.0,
        ));

        app.add_systems((bouncing_prompt_system,));

        app.add_systems(
            (setup_game_ui_system.after(GameEnterSet::BuildUi),)
                .in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (
                update_player1_ui.after(GameUpdateSet::UpdateUi),
                update_player2_ui.after(GameUpdateSet::UpdateUi),
                fade_out_system,
            )
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );

        app.add_systems(
            (setup_main_menu_system,).in_schedule(OnEnter(states::AppStates::MainMenu)),
        );

        app.add_systems(
            (setup_instructions_system,).in_schedule(OnEnter(states::AppStates::Instructions)),
        );

        app.add_systems(
            (setup_character_selection_system,)
                .in_schedule(OnEnter(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (player_join_system, select_character_system)
                .in_set(OnUpdate(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (setup_game_over_system,).in_schedule(OnEnter(states::AppStates::GameOver)),
        );

        app.add_systems((game_over_fade_in_system,).in_set(OnUpdate(states::AppStates::GameOver)));

        app.add_systems((setup_victory_system,).in_schedule(OnEnter(states::AppStates::Victory)));

        app.add_systems((victory_fade_in_system,).in_set(OnUpdate(states::AppStates::Victory)));

        app.add_systems((setup_pause_system,).in_schedule(OnEnter(states::GameStates::Paused)));
    }
}

/// Tag for player health ui
#[derive(Component)]
pub struct HealthUI;

#[derive(Component)]

pub struct AbilityUI;

/// Tag for armor ui
#[derive(Component)]
pub struct ArmorUI;

/// Tag for level ui
#[derive(Component)]
pub struct LevelUI;

/// Tag for level ui
#[derive(Component)]
pub struct PowerGlowUI(Timer);

#[derive(Component)]
pub struct FPSUI;

#[derive(Component)]
pub struct StatBarLabel;

#[derive(Component)]
pub struct AbilityChargingUI;

#[derive(Component)]
pub struct AbilityReadyUI;

#[derive(Component)]
pub struct Player1UI;

#[derive(Component)]
pub struct Player2UI;

/// Initialize all ui
pub fn setup_game_ui_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    players_resource: Res<PlayersResource>,
) {
    // level objective ui
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                },
                position: UiRect {
                    left: Val::Percent(6.5),
                    bottom: Val::Percent(70.0),
                    ..UiRect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::BLUE.into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(LevelUI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/defense_bar_label.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(6.5),
                    bottom: Val::Percent(73.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel);

    // player 1 ui
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(80.0),
                    height: Val::Px(15.0),
                },
                position: UiRect {
                    left: Val::Percent(1.5),
                    bottom: Val::Percent(65.0),
                    ..UiRect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::PURPLE.into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(AbilityUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/ability_charging.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(0.3),
                    bottom: Val::Percent(63.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(AbilityChargingUI)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/ability_ready.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(1.5),
                    bottom: Val::Percent(65.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(AbilityReadyUI)
        .insert(BouncingPromptComponent {
            flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            is_active: false,
        })
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                },
                position: UiRect {
                    left: Val::Percent(3.5),
                    bottom: Val::Percent(70.0),
                    ..UiRect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            background_color: Color::RED.into(),
            ..NodeBundle::default()
        })
        .insert(GameCleanup)
        .insert(HealthUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/health_bar_label.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(3.5),
                    bottom: Val::Percent(74.5),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/armor_spritesheet.png").into(),
            style: Style {
                size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                position: UiRect {
                    left: Val::Percent(3.5),
                    bottom: Val::Percent(69.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(2.5, 2.5, 1.0)),
            background_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(ArmorUI)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_container.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(4.5),
                    bottom: Val::Percent(55.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_glow.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(4.5),
                    bottom: Val::Percent(55.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(PowerGlowUI(Timer::new(
            Duration::from_secs_f32(2.0),
            TimerMode::Repeating,
        )))
        .insert(Player1UI);

    commands
        .spawn(ImageBundle {
            image: asset_server.load("texture/power_label.png").into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(3.0),
                    bottom: Val::Percent(49.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(1.3, 1.3, 1.0)),
            ..Default::default()
        })
        .insert(GameCleanup)
        .insert(StatBarLabel)
        .insert(Player1UI);

    // player 2 ui if there is a second player
    if players_resource.player_inputs[1].is_some() {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(80.0),
                        height: Val::Px(15.0),
                    },
                    position: UiRect {
                        left: Val::Percent(91.5),
                        bottom: Val::Percent(65.0),
                        ..UiRect::default()
                    },
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                background_color: Color::PURPLE.into(),
                ..NodeBundle::default()
            })
            .insert(GameCleanup)
            .insert(AbilityUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/ability_charging.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(90.5),
                        bottom: Val::Percent(63.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(AbilityChargingUI)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/ability_ready.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(91.5),
                        bottom: Val::Percent(65.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(AbilityReadyUI)
            .insert(BouncingPromptComponent {
                flash_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                is_active: false,
            })
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Px(15.0),
                        height: Val::Px(200.0),
                    },
                    position: UiRect {
                        left: Val::Percent(92.5),
                        bottom: Val::Percent(70.0),
                        ..UiRect::default()
                    },
                    position_type: PositionType::Absolute,
                    ..Style::default()
                },
                background_color: Color::RED.into(),
                ..NodeBundle::default()
            })
            .insert(GameCleanup)
            .insert(HealthUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/health_bar_label.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(92.5),
                        bottom: Val::Percent(74.5),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/armor_spritesheet.png").into(),
                style: Style {
                    size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                    position: UiRect {
                        left: Val::Percent(92.5),
                        bottom: Val::Percent(69.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(2.5, 2.5, 1.0)),
                background_color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(ArmorUI)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_container.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(93.5),
                        bottom: Val::Percent(55.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_glow.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(93.5),
                        bottom: Val::Percent(55.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(3.0, 3.0, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(PowerGlowUI(Timer::new(
                Duration::from_secs_f32(2.0),
                TimerMode::Repeating,
            )))
            .insert(Player2UI);

        commands
            .spawn(ImageBundle {
                image: asset_server.load("texture/power_label.png").into(),
                style: Style {
                    position: UiRect {
                        left: Val::Percent(92.5),
                        bottom: Val::Percent(49.0),
                        ..default()
                    },
                    position_type: PositionType::Absolute,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::new(1.3, 1.3, 1.0)),
                ..Default::default()
            })
            .insert(GameCleanup)
            .insert(StatBarLabel)
            .insert(Player2UI);
    }
}

pub fn setup_fps_ui_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // setup font
    let font = asset_server.load("fonts/SpaceMadness.ttf");

    commands
        .spawn(TextBundle {
            style: Style {
                size: Size::default(),
                position: UiRect {
                    left: Val::Percent(90.0),
                    bottom: Val::Percent(5.0),
                    ..UiRect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::from_section(
                "fps: ",
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ),
            ..Default::default()
        })
        .insert(Name::new("FPS UI"))
        .insert(FPSUI);
}

pub fn fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FPSUI>>) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("fps: {average:.2}");
        }
    };
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_player1_ui(
    mut player1_ui_queries: ParamSet<(
        Query<&mut Style, (With<HealthUI>, With<Player1UI>)>,
        Query<&mut BackgroundColor, (With<ArmorUI>, With<Player1UI>)>,
        Query<(&mut BackgroundColor, &mut Transform, &mut PowerGlowUI), With<Player1UI>>,
        Query<&mut Style, (With<AbilityUI>, With<Player1UI>)>,
        Query<&mut Visibility, (With<AbilityChargingUI>, With<Player1UI>)>,
        Query<&mut Visibility, (With<AbilityReadyUI>, With<Player1UI>)>,
        Query<&mut Style, With<LevelUI>>,
    )>,
    player_query: Query<&PlayerComponent>,
    run_resource: Res<RunResource>,
    time: Res<Time>,
) {
    // update player health ui

    for mut style_component in player1_ui_queries.p0().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                style_component.size.height = Val::Px(
                    200.0
                        * (player_component.health.get_health()
                            / player_component.health.get_max_health()),
                )
            }
        }
    }

    for mut style_component in player1_ui_queries.p6().iter_mut() {
        if let Some(level) = &run_resource.level {
            match &level.objective {
                crate::run::ObjectiveType::Defense(health) => {
                    style_component.size.height =
                        Val::Px(200.0 * (health.get_health() / health.get_max_health()));
                }
            }
        }
    }

    for mut ui_color in player1_ui_queries.p1().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                if player_component.health.get_armor() > 0 {
                    ui_color.0.set_a(1.0);
                } else {
                    ui_color.0.set_a(0.2);
                }
            }
        }
    }

    for (mut ui_color, mut transform, mut power_glow) in player1_ui_queries.p2().iter_mut() {
        power_glow.0.tick(time.delta());
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                let new_scale = (3.0 * (player_component.money as f32 / 25.0).min(25.0))
                    + (0.2 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin())
                    + 0.2;
                transform.scale = Vec3::new(new_scale, new_scale, 1.0);
                ui_color.0.set_a(
                    (0.5 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin()) + 0.5,
                );
            }
        }
    }

    // update player ability ui
    for mut style_component in player1_ui_queries.p3().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                style_component.size.width = Val::Px(80.0 * cooldown_ratio);
            }
        }
    }

    for mut visibility_component in player1_ui_queries.p4().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Hidden;
                } else {
                    *visibility_component = Visibility::Visible;
                }
            }
        }
    }

    for mut visibility_component in player1_ui_queries.p5().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 0 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Visible;
                } else {
                    *visibility_component = Visibility::Hidden;
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_player2_ui(
    mut player2_ui_queries: ParamSet<(
        Query<&mut Style, (With<HealthUI>, With<Player2UI>)>,
        Query<&mut BackgroundColor, (With<ArmorUI>, With<Player2UI>)>,
        Query<(&mut BackgroundColor, &mut Transform, &mut PowerGlowUI), With<Player2UI>>,
        Query<&mut Style, (With<AbilityUI>, With<Player2UI>)>,
        Query<&mut Visibility, (With<AbilityChargingUI>, With<Player2UI>)>,
        Query<&mut Visibility, (With<AbilityReadyUI>, With<Player2UI>)>,
    )>,
    player_query: Query<&PlayerComponent>,
    time: Res<Time>,
) {
    // update player health ui

    for mut style_component in player2_ui_queries.p0().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                style_component.size.height = Val::Px(
                    200.0
                        * (player_component.health.get_health()
                            / player_component.health.get_max_health()),
                )
            }
        }
    }

    for mut ui_color in player2_ui_queries.p1().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                if player_component.health.get_armor() > 0 {
                    ui_color.0.set_a(1.0);
                } else {
                    ui_color.0.set_a(0.2);
                }
            }
        }
    }

    for (mut ui_color, mut transform, mut power_glow) in player2_ui_queries.p2().iter_mut() {
        power_glow.0.tick(time.delta());
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                let new_scale = (3.0 * (player_component.money as f32 / 25.0).min(25.0))
                    + (0.2 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin())
                    + 0.2;
                transform.scale = Vec3::new(new_scale, new_scale, 1.0);
                ui_color.0.set_a(
                    (0.5 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin()) + 0.5,
                );
            }
        }
    }

    // update player ability ui
    for mut style_component in player2_ui_queries.p3().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                style_component.size.width = Val::Px(80.0 * cooldown_ratio);
            }
        }
    }

    for mut visibility_component in player2_ui_queries.p4().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Hidden;
                } else {
                    *visibility_component = Visibility::Visible;
                }
            }
        }
    }

    for mut visibility_component in player2_ui_queries.p5().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.player_index == 1 {
                let cooldown_ratio = player_component.ability_cooldown_timer.elapsed_secs()
                    / player_component
                        .ability_cooldown_timer
                        .duration()
                        .as_secs_f32();

                if cooldown_ratio as i8 == 1 {
                    *visibility_component = Visibility::Visible;
                } else {
                    *visibility_component = Visibility::Hidden;
                }
            }
        }
    }
}
