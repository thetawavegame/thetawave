use std::{f32::consts::PI, time::Duration};

use bevy::{
    color::Color,
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query, Res},
    },
    hierarchy::BuildChildren,
    math::f32,
    time::{Time, Timer, TimerMode},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        FlexDirection, Style, UiImage, Val,
    },
    utils::default,
};
use thetawave_interface::{
    objective::{DefenseInteraction, MobReachedBottomGateEvent},
    states::GameCleanup,
};

use crate::assets::UiAssets;

use super::parent::BorderGradientCommandsExt;

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

impl BorderGradientCommandsExt for Commands<'_, '_> {
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
        .insert(GameCleanup)
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: HEIGHT,
                        ..default()
                    },
                    image: UiImage::new(match bg_type {
                        BorderGradientType::Warning => ui_assets.warning_gradient.clone(),
                        BorderGradientType::Defense => ui_assets.defense_gradient.clone(),
                    })
                    .with_color(Color::srgba(1.0, 1.0, 1.0, 0.0)),
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

/// Starts a border gradient effect by reseting its timer when an event is read
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

/// Sets the alpha of a border gradient's background color based on the time reamining
/// in the border gradient component's timer
pub(super) fn border_gradient_update_system(
    mut bg_query: Query<(&mut BorderGradientComponent, &mut UiImage)>,
    time: Res<Time>,
) {
    for (mut bg_component, mut ui_image) in bg_query.iter_mut() {
        bg_component.timer.tick(time.delta());
        ui_image.color = Color::srgba(
            1.0,
            1.0,
            1.0,
            MAX_ALPHA * f32::sin(PI * bg_component.timer.fraction()),
        );
    }
}

/// Trigger a border gradient event mobs reach the bottom gate
pub(super) fn border_gradient_on_gate_interaction_system(
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
