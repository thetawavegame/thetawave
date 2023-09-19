use bevy::prelude::*;
use thetawave_interface::{
    health::HealthComponent,
    run::{LevelPhaseType, TutorialLesson},
};

use crate::{run::CurrentRunProgressResource, spawnable::BossComponent};

#[derive(Component)]
pub struct TopMiddleLeftUI;

#[derive(Component)]
pub struct TopMiddleRightUI;

//Phase UI
#[derive(Component)]
pub struct PhaseNameUI;

#[derive(Component)]
pub struct PhaseDataUI;

#[derive(Component)]
pub struct PhaseDataTextUI;

#[derive(Component)]
pub struct PhaseDataObjectivesListUI;

#[derive(Component)]
pub struct PhaseTextObjectiveUI;

#[derive(Component)]
pub struct BossHealthUI;

#[derive(Component)]
pub struct BossHealthValueUI;

// OLD phase ui - remove
#[derive(Component)]
pub struct PhaseUiComponent;

#[derive(Component)]
pub struct TutorialPhaseUI;

pub fn build_phase_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .insert(TopMiddleLeftUI)
        .with_children(|top_middle_left_ui| {
            top_middle_left_ui
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font,
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(PhaseNameUI);
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
        .insert(TopMiddleRightUI)
        .with_children(|top_middle_right_ui| {
            top_middle_right_ui
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
                .insert(PhaseDataUI);
        });
}

pub fn update_phase_ui_system(
    asset_server: ResMut<AssetServer>,
    mut commands: Commands,
    mut phase_name_ui_query: Query<&mut Text, With<PhaseNameUI>>,
    phase_data_ui_query: Query<Entity, With<PhaseDataUI>>,
    run_resource: Res<CurrentRunProgressResource>,
    boss_mobs_query: Query<&HealthComponent, With<BossComponent>>,
) {
    if let Some(current_level) = &run_resource.current_level {
        if let Some(current_phase) = &current_level.current_phase {
            if let Ok(mut text) = phase_name_ui_query.get_single_mut() {
                text.sections[0].value = current_phase.phase_type.get_name()
            }

            if let Ok(entity) = phase_data_ui_query.get_single() {
                commands.entity(entity).despawn_descendants();

                match &current_phase.phase_type {
                    LevelPhaseType::FormationSpawn { phase_timer, .. } => {
                        let font = asset_server.load("fonts/wibletown-regular.otf");

                        commands.entity(entity).with_children(|phase_data_ui| {
                            phase_data_ui
                                .spawn(TextBundle {
                                    style: Style {
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        format!("{:.0}", phase_timer.remaining_secs()),
                                        TextStyle {
                                            font,
                                            font_size: 48.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(PhaseDataTextUI);
                        });
                    }
                    LevelPhaseType::Break { phase_timer } => {
                        let font = asset_server.load("fonts/wibletown-regular.otf");

                        commands.entity(entity).with_children(|phase_data_ui| {
                            phase_data_ui
                                .spawn(TextBundle {
                                    style: Style {
                                        align_self: AlignSelf::Center,
                                        ..default()
                                    },
                                    text: Text::from_section(
                                        format!("{:.0}", phase_timer.remaining_secs()),
                                        TextStyle {
                                            font,
                                            font_size: 48.0,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..default()
                                })
                                .insert(PhaseDataTextUI);
                        });
                    }
                    LevelPhaseType::Boss { .. } => {
                        if let Ok(health) = boss_mobs_query.get_single() {
                            commands.entity(entity).with_children(|phase_data_ui| {
                                phase_data_ui
                                    .spawn(NodeBundle {
                                        style: Style {
                                            width: Val::Percent(80.0),
                                            height: Val::Percent(60.0),
                                            flex_direction: FlexDirection::Row,
                                            ..default()
                                        },
                                        background_color: Color::RED.with_a(0.05).into(),
                                        ..default()
                                    })
                                    .insert(BossHealthUI)
                                    .with_children(|boss_health_ui| {
                                        boss_health_ui
                                            .spawn(NodeBundle {
                                                style: Style {
                                                    width: Val::Percent(
                                                        100.0 * health.get_health_percentage(),
                                                    ),
                                                    height: Val::Percent(100.0),
                                                    ..default()
                                                },
                                                background_color: Color::RED.with_a(0.75).into(),
                                                ..default()
                                            })
                                            .insert(BossHealthValueUI);
                                    });
                            });
                        }
                    }
                    LevelPhaseType::Tutorial {
                        tutorial_lesson, ..
                    } => {
                        let font = asset_server.load("fonts/wibletown-regular.otf");

                        commands.entity(entity).with_children(|phase_data_ui| {
                            phase_data_ui
                                .spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        flex_direction: FlexDirection::Column,
                                        flex_wrap: FlexWrap::Wrap,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(PhaseDataObjectivesListUI)
                                .with_children(|phase_data_list_ui| {
                                    match tutorial_lesson {
                                        TutorialLesson::Movement { .. } => {
                                            for (progress_str, completed) in
                                                tutorial_lesson.get_movement_timer_strs().iter()
                                            {
                                                phase_data_list_ui
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            height: Val::Px(30.0), // Set a fixed height for each text section
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            progress_str,
                                                            TextStyle {
                                                                font: font.clone(),
                                                                font_size: 24.0,
                                                                color: if *completed {
                                                                    Color::GREEN
                                                                } else {
                                                                    Color::WHITE
                                                                },
                                                            },
                                                        )
                                                        .with_alignment(TextAlignment::Left),
                                                        ..default()
                                                    })
                                                    .insert(PhaseTextObjectiveUI);
                                            }
                                        }
                                        TutorialLesson::Attack { .. } => {
                                            for (progress_str, completed) in
                                                tutorial_lesson.get_attack_strs().iter()
                                            {
                                                phase_data_list_ui
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            height: Val::Px(30.0), // Set a fixed height for each text section
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            progress_str,
                                                            TextStyle {
                                                                font: font.clone(),
                                                                font_size: 24.0,
                                                                color: if *completed {
                                                                    Color::GREEN
                                                                } else {
                                                                    Color::WHITE
                                                                },
                                                            },
                                                        )
                                                        .with_alignment(TextAlignment::Left),
                                                        ..default()
                                                    })
                                                    .insert(PhaseTextObjectiveUI);
                                            }
                                        }
                                        TutorialLesson::CaptainAbility { .. } => {
                                            for (progress_str, completed) in
                                                tutorial_lesson.get_captain_ability_strs().iter()
                                            {
                                                phase_data_list_ui
                                                    .spawn(TextBundle {
                                                        style: Style {
                                                            height: Val::Px(30.0), // Set a fixed height for each text section
                                                            ..default()
                                                        },
                                                        text: Text::from_section(
                                                            progress_str,
                                                            TextStyle {
                                                                font: font.clone(),
                                                                font_size: 24.0,
                                                                color: if *completed {
                                                                    Color::GREEN
                                                                } else {
                                                                    Color::WHITE
                                                                },
                                                            },
                                                        )
                                                        .with_alignment(TextAlignment::Left),
                                                        ..default()
                                                    })
                                                    .insert(PhaseTextObjectiveUI);
                                            }
                                        }
                                    }
                                });
                        });
                    }
                }
            }
        }
    }
}
