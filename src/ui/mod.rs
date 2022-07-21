use std::time::Duration;

use bevy::prelude::*;

use crate::{
    game_over::EndGameTransitionResource,
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

pub fn setup_ui_camera_system(mut commands: Commands) {
    // spawn camera for viewing ui
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Initialize all ui
pub fn setup_game_ui_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // setup font
    //let font = asset_server.load("fonts/SpaceMadness.ttf");

    /*
    // spawn player health ui
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(86.0),
                    bottom: Val::Percent(90.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                "Health: 0/0\nArmor: 0\nPower: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(HealthUI);
        */

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                },
                position: Rect {
                    left: Val::Percent(92.5),
                    bottom: Val::Percent(70.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            color: Color::RED.into(),
            ..NodeBundle::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(HealthUI);

    commands
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/health_bar_label.png").into(),
            style: Style {
                position: Rect {
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
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/armor_spritesheet.png").into(),
            style: Style {
                size: Size::new(Val::Px(12.0), Val::Px(12.0)),
                position: Rect {
                    left: Val::Percent(92.5),
                    bottom: Val::Percent(69.0),
                    ..default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(2.5, 2.5, 1.0)),
            color: Color::rgba(1.0, 1.0, 1.0, 0.2).into(),
            ..Default::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(StatBarLabel)
        .insert(ArmorUI);

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Px(15.0),
                    height: Val::Px(200.0),
                },
                position: Rect {
                    left: Val::Percent(95.5),
                    bottom: Val::Percent(70.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            color: Color::BLUE.into(),
            ..NodeBundle::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(LevelUI);

    commands
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/defense_bar_label.png").into(),
            style: Style {
                position: Rect {
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
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/power_container.png").into(),
            style: Style {
                position: Rect {
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
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/power_glow.png").into(),
            style: Style {
                position: Rect {
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
        .insert(PowerGlowUI(Timer::new(Duration::from_secs_f32(2.0), true)));

    commands
        .spawn_bundle(ImageBundle {
            image: asset_server.load("texture/power_label.png").into(),
            style: Style {
                position: Rect {
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

    // spawn level ui
    /*
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(86.0),
                    bottom: Val::Percent(80.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                "Phase Type: None\nPhase Number: None\nObjective: None",
                TextStyle {
                    font: font.clone(),
                    font_size: 12.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(AppStateComponent(AppStates::Game))
        .insert(LevelUI);
        */
}

pub fn setup_fps_ui_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // setup font
    let font = asset_server.load("fonts/SpaceMadness.ttf");

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(90.0),
                    bottom: Val::Percent(5.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                "fps: ",
                TextStyle {
                    font,
                    font_size: 18.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..Default::default()
        })
        .insert(Name::new("FPS UI"))
        .insert(FPSUI);
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_ui(
    mut ui_queries: ParamSet<(
        Query<&mut Style, With<HealthUI>>,
        Query<&mut Style, With<LevelUI>>,
        Query<&mut UiColor, With<ArmorUI>>,
        Query<(&mut UiColor, &mut Transform, &mut PowerGlowUI)>,
    )>,
    player_query: Query<&PlayerComponent>,
    run_resource: Res<RunResource>,
    time: Res<Time>,
    end_game_trans_resource: Res<EndGameTransitionResource>,
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
            /*
            style_component.size.height = Val::Px(
                200.0
                    * (player_component.health.get_health()
                        / player_component.health.get_max_health()),
            )
            */
        }
    }

    /*
    for mut text_component in ui_queries.p0().iter_mut() {
        for player_component in player_query.iter() {
            text_component.sections[0].value = format!(
                "Health: {}/{}\nArmor: {}\nPower: {}",
                player_component.health.get_health(),
                player_component.health.get_max_health(),
                player_component.health.get_armor(),
                player_component.money,
            );
        }
        continue;
    }
    */

    // update level ui
    /*
    if let Some(level) = &run_resource.level {
        for mut text_component in ui_queries.p1().iter_mut() {
            text_component.sections[0].value = format!(
                "Phase Type: {}\nPhase Number: {}\nObjective:{}",
                level.get_phase_name(),
                level.get_phase_number(),
                match &level.objective {
                    crate::run::ObjectiveType::Defense(health) => {
                        format!(
                            "\n    Defense: {}/{}",
                            health.get_health(),
                            health.get_max_health()
                        )
                    }
                }
            );
            continue;
        }
    }
    */
}

pub fn position_stat_bar_label_system(
    mut stat_bar_query: Query<&mut GlobalTransform, With<StatBarLabel>>,
) {
    for mut stat_bar_transform in stat_bar_query.iter_mut() {
        stat_bar_transform.translation.z = 1.0;
    }
}
