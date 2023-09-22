use bevy::prelude::*;

use crate::run::CurrentRunProgressResource;

/// Used for querying UI for displaying name
#[derive(Component)]
pub struct LevelNameUI;

/// Used for querying UI for displaying level information
#[derive(Component)]
pub struct LevelDataUI;

pub fn build_level_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|bottom_middle_left_ui| {
            bottom_middle_left_ui
                .spawn(TextBundle {
                    style: Style::default(),
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font.clone(),
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(LevelNameUI);
        });

    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        })
        .with_children(|bottom_middle_right_ui| {
            bottom_middle_right_ui
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        padding: UiRect::new(
                            Val::Vw(1.0),
                            Val::Vw(1.0),
                            Val::Vh(2.0),
                            Val::Vh(2.0),
                        ),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .insert(LevelDataUI);
        });
}

pub fn update_level_ui_system(
    mut commands: Commands,
    level_data_ui_query: Query<Entity, With<LevelDataUI>>,
    mut level_name_ui_query: Query<&mut Text, With<LevelNameUI>>,
    run_resource: Res<CurrentRunProgressResource>,
) {
    if let Some(current_level) = &run_resource.current_level {
        if let Ok(mut text) = level_name_ui_query.get_single_mut() {
            text.sections[0].value = current_level.get_name();
        }

        if let Ok(entity) = level_data_ui_query.get_single() {
            commands.entity(entity).despawn_descendants();

            if let Some(objective) = &current_level.objective {
                match objective {
                    thetawave_interface::objective::Objective::Defense(defense_data) => {
                        commands.entity(entity).with_children(|level_data_ui| {
                            level_data_ui
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(80.0),
                                        height: Val::Percent(60.0),
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    background_color: Color::BLUE.with_a(0.05).into(),
                                    ..default()
                                })
                                .with_children(|defense_ui| {
                                    defense_ui.spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(
                                                100.0 * defense_data.get_percentage(),
                                            ),
                                            height: Val::Percent(100.0),
                                            ..default()
                                        },
                                        background_color: Color::BLUE.with_a(0.75).into(),
                                        ..default()
                                    });
                                });
                        });
                    }
                }
            }
        }
    }
}
