use bevy::prelude::*;

use crate::{player::PlayerComponent, run::RunResource};

/// Tag for player health ui
#[derive(Component)]
pub struct HealthUI;

/// Tag for level ui
#[derive(Component)]
pub struct LevelUI;

#[derive(Component)]
pub struct FPSUI;

/// Initialize all ui
pub fn setup_ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // setup font
    let font = asset_server.load("fonts/SpaceMadness.ttf");

    // spawn camera for viewing ui
    commands.spawn_bundle(UiCameraBundle::default());

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
                "Health: 0/0\nArmor: 0\nMoney: 0",
                TextStyle {
                    font: font.clone(),
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(HealthUI);

    // spawn level ui
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
        .insert(LevelUI);

    // debug ui
    if cfg!(debug_assertions) {
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
            .insert(FPSUI);
    }
}

#[allow(clippy::type_complexity)]
/// Update ui to current data from game
pub fn update_ui(
    mut ui_queries: ParamSet<(
        Query<&mut Text, With<HealthUI>>,
        Query<&mut Text, With<LevelUI>>,
    )>,
    player_query: Query<&PlayerComponent>,
    run_resource: Res<RunResource>,
) {
    // update player health ui
    for mut text_component in ui_queries.p0().iter_mut() {
        for player_component in player_query.iter() {
            text_component.sections[0].value = format!(
                "Health: {}/{}\nArmor: {}\nMoney: {}",
                player_component.health.get_health(),
                player_component.health.get_max_health(),
                player_component.health.get_armor(),
                player_component.money,
            );
        }
        continue;
    }

    // update level ui
    for mut text_component in ui_queries.p1().iter_mut() {
        text_component.sections[0].value = format!(
            "Phase Type: {}\nPhase Number: {}\nObjective:{}",
            run_resource.levels[run_resource.level_idx].get_phase_name(),
            run_resource.levels[run_resource.level_idx].get_phase_number(),
            match &run_resource.levels[run_resource.level_idx].objective {
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
