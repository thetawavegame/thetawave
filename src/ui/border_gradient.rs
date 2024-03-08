use std::{f32::consts::PI, time::Duration};

use bevy::{
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    math::f32,
    render::color::Color,
    time::{Time, Timer, TimerMode},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        BackgroundColor, FlexDirection, Style, Val,
    },
    utils::default,
};
use thetawave_interface::objective::{DefenseInteraction, MobReachedBottomGateEvent};

use crate::assets::UiAssets;

const DURATION: f32 = 0.4;
const MAX_ALPHA: f32 = 0.5;
const HEIGHT: Val = Val::Percent(15.0);

#[derive(Event, PartialEq)]
pub(super) enum BorderGradientType {
    Warning,
    Defense,
}

pub(super) type BorderGradientEvent = BorderGradientType;

#[derive(Component)]
pub(super) struct BorderGradientComponent {
    pub bg_type: BorderGradientType,
    pub timer: Timer,
}

pub(super) trait UiCommandsExt {
    fn spawn_border_gradient(&mut self, ui_assets: &UiAssets, bg_type: BorderGradientType);
}

impl UiCommandsExt for Commands<'_, '_> {
    fn spawn_border_gradient(&mut self, ui_assets: &UiAssets, bg_type: BorderGradientType) {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: HEIGHT,
                        ..default()
                    },
                    image: match bg_type {
                        BorderGradientType::Warning => ui_assets.warning_gradient.clone(),
                        BorderGradientType::Defense => ui_assets.defense_gradient.clone(),
                    }
                    .into(),
                    background_color: Color::WHITE.with_a(0.0).into(),
                    ..default()
                })
                .insert(BorderGradientComponent {
                    bg_type,
                    timer: {
                        let mut timer = Timer::from_seconds(DURATION, TimerMode::Once);
                        timer.set_elapsed(Duration::from_secs_f32(DURATION));
                        timer
                    },
                });
        });
    }
}

pub(super) fn border_gradient_start_system(
    mut bg_query: Query<&mut BorderGradientComponent>,
    mut bg_event_reader: EventReader<BorderGradientEvent>,
) {
    for event in bg_event_reader.read() {
        for mut bg_component in bg_query.iter_mut() {
            if bg_component.bg_type == *event {
                bg_component.timer.reset();
            }
        }
    }
}

pub(super) fn border_gradient_update_system(
    mut bg_query: Query<(&mut BorderGradientComponent, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut bg_component, mut background_color) in bg_query.iter_mut() {
        bg_component.timer.tick(time.delta());
        background_color
            .0
            .set_a(MAX_ALPHA * f32::sin(PI * bg_component.timer.fraction()));
    }
}

/// Trigger a border gradient event mobs reach the bottom gate
pub(super) fn border_gradient_on_gate_interaction(
    mut gate_events: EventReader<MobReachedBottomGateEvent>,
    mut bg_event_writer: EventWriter<BorderGradientEvent>,
) {
    for event in gate_events.read() {
        match event.defense_interaction {
            DefenseInteraction::Heal(_) => bg_event_writer.send(BorderGradientEvent::Defense),

            DefenseInteraction::Damage(_) => bg_event_writer.send(BorderGradientEvent::Warning),
        };
    }
}
