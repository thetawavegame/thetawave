use bevy::prelude::*;
use thetawave_interface::objective::NewObjectiveEvent;

#[derive(Component)]
pub struct BottomMiddleLeftUI;

#[derive(Component)]
pub struct BottomMiddleRightUI;

//Level UI
#[derive(Component)]
pub struct LevelNameUI;

#[derive(Component)]
pub struct DefenseUI;

#[derive(Component)]
pub struct DefenseValueUI;

// USED FOR OLD UI LAYOUT
/// Tag for level ui
#[derive(Component)]
pub struct ObjectiveUI;

/// Tag for level ui
#[derive(Component)]
pub struct ObjectiveLabelUI;

pub fn build_level_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(50.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                ..default()
            },
            //background_color: Color::GREEN.with_a(0.25).into(),
            ..default()
        })
        .insert(BottomMiddleLeftUI)
        .with_children(|bottom_middle_left_ui| {
            bottom_middle_left_ui
                .spawn(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..default()
                    },
                    text: Text::from_section(
                        "Defense",
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
                padding: UiRect::new(Val::Vw(1.0), Val::Vw(1.0), Val::Vh(2.0), Val::Vh(2.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            //background_color: Color::YELLOW.with_a(0.1).into(),
            ..default()
        })
        .insert(BottomMiddleRightUI)
        .with_children(|bottom_middle_right_ui| {
            // Uncomment for text phase objective

            bottom_middle_right_ui
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
                .insert(DefenseUI)
                .with_children(|defense_ui| {
                    defense_ui
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            background_color: Color::BLUE.with_a(0.75).into(),
                            ..default()
                        })
                        .insert(DefenseValueUI);
                });
        });
}

/// Initialize objective ui when objective changes
pub fn setup_level_objective_ui_system(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut new_objective_event_reader: EventReader<NewObjectiveEvent>,
    mut objective_ui_query: Query<Entity, With<ObjectiveUI>>,
    mut objective_label_ui_query: Query<Entity, With<ObjectiveLabelUI>>,
) {
    /*
    // read event for new objective set
    for event in new_objective_event_reader.iter() {
        //remove existing objective ui
        for entity in objective_ui_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }
        for entity in objective_label_ui_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        //create ui for new objective
        if let Some(objective) = &event.objective {
            match objective {
                Objective::Defense(_) => {
                    // level objective ui
                    commands
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Px(800.0),
                                height: Val::Px(30.0),
                                left: Val::Percent(19.0),
                                bottom: Val::Percent(2.0),
                                position_type: PositionType::Absolute,
                                ..Style::default()
                            },
                            background_color: Color::BLUE.into(),
                            ..NodeBundle::default()
                        })
                        .insert(GameCleanup)
                        .insert(ObjectiveUI);

                    commands
                        .spawn(ImageBundle {
                            image: asset_server.load("texture/defense_bar_label.png").into(),
                            style: Style {
                                left: Val::Percent(42.5),
                                bottom: Val::Percent(1.7),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            transform: Transform::from_xyz(0.0, 0.0, 1.0),
                            ..Default::default()
                        })
                        .insert(GameCleanup)
                        .insert(ObjectiveLabelUI)
                        .insert(StatBarLabel);
                }
            }
        }
    }
    */
}
