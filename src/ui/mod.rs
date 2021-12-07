use bevy::prelude::*;

use crate::player::PlayerComponent;

pub struct HealthUI;

/// Initialize ui
pub fn setup_ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/SpaceMadness.ttf");
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                size: Size::default(),
                position: Rect {
                    left: Val::Percent(87.0),
                    bottom: Val::Percent(95.0),
                    ..Rect::default()
                },
                position_type: PositionType::Absolute,
                ..Style::default()
            },
            text: Text::with_section(
                "Health: 0/0",
                TextStyle {
                    font,
                    font_size: 16.0,
                    color: Color::WHITE,
                },
                TextAlignment::default(),
            ),
            ..TextBundle::default()
        })
        .insert(HealthUI);
}

/// Update ui to current data from game
pub fn update_ui(
    mut ui_query: Query<&mut Text, With<HealthUI>>,
    player_query: Query<&PlayerComponent>,
) {
    for mut text_component in ui_query.iter_mut() {
        for player_component in player_query.iter() {
            text_component.sections[0].value = format!(
                "Health: {}/{}",
                player_component.health.get_health(),
                player_component.health.get_max_health()
            );
        }
    }
}
