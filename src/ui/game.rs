use std::time::Duration;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use crate::{
    assets::GameUIAssets,
    player::PlayerComponent,
    run::RunResource,
    states::{AppStateComponent, AppStates},
};

/// Tag for player health ui

#[derive(Component)]
pub struct HealthUI;

/// Tag for armor ui
#[derive(Component)]
pub struct ArmorUI;

#[derive(Component)]
pub struct StatBarLabel;

/// Tag for level ui
#[derive(Component)]
pub struct LevelUI;

/// Tag for level ui
#[derive(Component)]
pub struct PowerGlowUI(Timer);

/// Initialize all ui
pub fn setup_game_ui_system(mut commands: Commands, ui_assets: Res<GameUIAssets>) {
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(HealthUI);

    commands
        .spawn(ImageBundle {
            image: ui_assets.health_label.clone().into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(92.5),
                    bottom: Val::Percent(74.5),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(StatBarLabel);

    commands
        .spawn(ImageBundle {
            image: ui_assets.armor_icon.clone().into(),
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
        .insert(AppStateComponent(AppStates::Game))
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(LevelUI);

    commands
        .spawn(ImageBundle {
            image: ui_assets.defense_label.clone().into(),
            style: Style {
                position: UiRect {
                    left: Val::Percent(95.5),
                    bottom: Val::Percent(73.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(StatBarLabel);

    commands
        .spawn(ImageBundle {
            image: ui_assets.power_container.clone().into(),
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(StatBarLabel);

    commands
        .spawn(ImageBundle {
            image: ui_assets.power_glow.clone().into(),
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(PowerGlowUI(Timer::new(
            Duration::from_secs_f32(2.0),
            TimerMode::Repeating,
        )));

    commands
        .spawn(ImageBundle {
            image: ui_assets.power_label.clone().into(),
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(StatBarLabel);
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_game_ui(
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

pub fn position_stat_bar_label_system(
    mut stat_bar_query: Query<&mut GlobalTransform, With<StatBarLabel>>,
) {
    for mut stat_bar_transform in stat_bar_query.iter_mut() {
        stat_bar_transform.translation_mut().z = 1.0;
    }
}
