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

/// Tag for level ui
#[derive(Component)]
pub struct LevelUI;

#[derive(Component)]
pub struct FPSUI;

pub fn setup_ui_camera_system(mut commands: Commands) {
    // spawn camera for viewing ui
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Initialize all ui
pub fn setup_game_ui_system(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    // setup font
    let font = asset_server.load("fonts/SpaceMadness.ttf");

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
        .insert(AppStateComponent(AppStates::Game))
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(LevelUI);
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
        Query<&mut Text, With<HealthUI>>,
        Query<&mut Text, With<LevelUI>>,
    )>,
    player_query: Query<&PlayerComponent>,
    run_resource: Res<RunResource>,
    end_game_trans_resource: Res<EndGameTransitionResource>,
) {
    // update player health ui
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
    // update level ui
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
}
