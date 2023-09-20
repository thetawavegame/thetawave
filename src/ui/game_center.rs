use bevy::prelude::*;
use thetawave_interface::run::CyclePhaseEvent;

use crate::run::CurrentRunProgressResource;

#[derive(Component)]
pub struct CenterTextUI;

pub fn build_center_text_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(TextBundle {
            style: Style::default(),
            text: Text::from_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 120.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::Center),
            //background_color: Color::BLACK.with_a(0.02).into(),
            ..default()
        })
        .insert(CenterTextUI);
}

pub fn update_center_text_ui_system(
    mut cycle_phase_event_reader: EventReader<CyclePhaseEvent>,
    run_resource: Res<CurrentRunProgressResource>,
    mut center_text_query: Query<(&mut Text, &mut BackgroundColor), With<CenterTextUI>>,
) {
    // if phase has been cycled update the text
    if cycle_phase_event_reader.iter().next().is_some() {
        if let Some(level) = &run_resource.current_level {
            if let Some(phase) = &level.current_phase {
                if let Ok((mut text, mut bg_color)) = center_text_query.get_single_mut() {
                    if let Some(intro_text) = phase.intro_text.clone() {
                        text.sections[0].value = intro_text;
                        *bg_color = Color::BLACK.with_a(0.4).into();
                    }
                }
            }
        }
    }
}
