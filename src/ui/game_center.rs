use crate::run::CurrentRunProgressResource;
use bevy::{
    asset::Handle,
    ecs::{
        component::Component,
        event::EventReader,
        query::With,
        system::{Query, Res},
    },
    hierarchy::ChildBuilder,
    render::color::Color,
    text::{Font, JustifyText, Text, TextStyle},
    time::{Time, Timer, TimerMode},
    ui::{node_bundles::TextBundle, BackgroundColor, Style},
    utils::default,
};
use std::time::Duration;
use thetawave_interface::run::CyclePhaseEvent;

const BASE_TEXT_ALPHA: f32 = 1.0;
const BASE_BACKGROUND_ALPHA: f32 = 0.4;
const FONT_SIZE: f32 = 90.0;
const TEXT_COLOR: Color = Color::WHITE;
const BACKGROUND_COLOR: Color = Color::BLACK;
const DEFAULT_FADE_TIME: f32 = 5.0;

#[derive(Component)]
pub struct CenterTextUI;

#[derive(Component)]
pub struct FadeOutUIComponent {
    pub timer: Timer,
}

impl Default for FadeOutUIComponent {
    fn default() -> Self {
        let mut timer = Timer::from_seconds(DEFAULT_FADE_TIME, TimerMode::Once);
        timer.set_elapsed(Duration::from_secs_f32(DEFAULT_FADE_TIME));

        Self { timer }
    }
}

pub fn build_center_text_ui(parent: &mut ChildBuilder, font: Handle<Font>) {
    parent
        .spawn(TextBundle {
            style: Style::default(),
            text: Text::from_section(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                },
            )
            .with_justify(JustifyText::Center),
            background_color: Color::BLACK.with_a(0.0).into(),
            ..default()
        })
        .insert(CenterTextUI)
        .insert(FadeOutUIComponent::default());
}

pub fn update_center_text_ui_system(
    mut cycle_phase_event_reader: EventReader<CyclePhaseEvent>,
    run_resource: Res<CurrentRunProgressResource>,
    mut center_text_query: Query<
        (&mut Text, &mut BackgroundColor, &mut FadeOutUIComponent),
        With<CenterTextUI>,
    >,
) {
    // if phase has been cycled update the text
    if cycle_phase_event_reader.read().next().is_some() {
        if let Some(level) = &run_resource.current_level {
            if let Some(phase) = &level.current_phase {
                if let Ok((mut text, mut bg_color, mut fade_out)) =
                    center_text_query.get_single_mut()
                {
                    if let Some(intro_text) = phase.intro_text.clone() {
                        text.sections[0].value = intro_text;
                        *bg_color = Color::BLACK.with_a(BASE_BACKGROUND_ALPHA).into();
                        fade_out.timer.reset();
                    }
                }
            }
        }
    }
}

pub fn text_fade_out_system(
    mut background_color_query: Query<(&mut Text, &mut BackgroundColor, &mut FadeOutUIComponent)>,
    time: Res<Time>,
) {
    for (mut text, mut bg_color, mut fade_out) in background_color_query.iter_mut() {
        fade_out.timer.tick(time.delta());

        *bg_color = BACKGROUND_COLOR
            .with_a(BASE_BACKGROUND_ALPHA * fade_out.timer.fraction_remaining())
            .into();

        text.sections[0].style.color =
            TEXT_COLOR.with_a(BASE_TEXT_ALPHA * fade_out.timer.fraction_remaining());
    }
}
