use std::time::Duration;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    player::PlayerComponent,
    run::RunResource,
    states::{self, AppStates, GameCleanup},
    GameEnterSet, GameUpdateSet,
};

mod debug;
mod game_over;
mod main_menu;
mod pause_menu;
mod victory;

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
            (update_ui.after(GameUpdateSet::UpdateUi), fade_out_system)
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );

        app.add_systems(
            (setup_main_menu_system,).in_schedule(OnEnter(states::AppStates::MainMenu)),
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

/// Initialize all ui
pub fn setup_game_ui_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
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
        .insert(HealthUI);

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
        .insert(StatBarLabel);

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
        .insert(ArmorUI);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                },
                position: UiRect {
                    left: Val::Percent(95.5),
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
                    left: Val::Percent(95.5),
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
        .insert(StatBarLabel);

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
        )));

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
        .insert(StatBarLabel);
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
pub fn update_ui(
    mut ui_queries: ParamSet<(
        Query<&mut Style, With<HealthUI>>,
        Query<&mut Style, With<LevelUI>>,
        Query<&mut BackgroundColor, With<ArmorUI>>,
        Query<(&mut BackgroundColor, &mut Transform, &mut PowerGlowUI)>,
    )>,
    player_query: Query<&PlayerComponent>,
    run_resource: Res<RunResource>,
    time: Res<Time>,
) {
    // update player health ui

    for mut style_component in ui_queries.p0().iter_mut() {
        for player_component in player_query.iter() {
            style_component.size.height = Val::Px(
                200.0
                    * (player_component.health.get_health()
                        / player_component.health.get_max_health()),
            )
        }
    }

    for mut style_component in ui_queries.p1().iter_mut() {
        if let Some(level) = &run_resource.level {
            match &level.objective {
                crate::run::ObjectiveType::Defense(health) => {
                    style_component.size.height =
                        Val::Px(200.0 * (health.get_health() / health.get_max_health()));
                }
            }
        }
    }

    for mut ui_color in ui_queries.p2().iter_mut() {
        for player_component in player_query.iter() {
            if player_component.health.get_armor() > 0 {
                ui_color.0.set_a(1.0);
            } else {
                ui_color.0.set_a(0.2);
            }
        }
    }

    for (mut ui_color, mut transform, mut power_glow) in ui_queries.p3().iter_mut() {
        power_glow.0.tick(time.delta());
        for player_component in player_query.iter() {
            let new_scale = (3.0 * (player_component.money as f32 / 25.0).min(25.0))
                + (0.2 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin())
                + 0.2;
            transform.scale = Vec3::new(new_scale, new_scale, 1.0);
            ui_color
                .0
                .set_a((0.5 * (power_glow.0.elapsed_secs() * std::f32::consts::PI).sin()) + 0.5);
        }
    }
}
