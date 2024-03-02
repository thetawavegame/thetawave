use std::{f32::consts::PI, time::Duration};

use bevy::{
    asset::AssetServer,
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
        BackgroundColor, FlexDirection, Style, Val, ZIndex,
    },
    utils::default,
};
use thetawave_interface::objective::{DefenseInteraction, MobReachedBottomGateEvent};

const BG_DURATION: f32 = 0.4;
const BG_MAX_ALPHA: f32 = 0.5;

#[derive(Event, PartialEq)]
pub enum BorderGradientType {
    Warning,
    Defense,
}

pub type BorderGradientEvent = BorderGradientType;

#[derive(Component)]
pub struct BorderGradientComponent {
    pub bg_type: BorderGradientType,
    pub timer: Timer,
}

pub trait UiCommandsExt {
    fn spawn_border_gradient(&mut self, asset_server: &AssetServer, bg_type: BorderGradientType);
}

impl UiCommandsExt for Commands<'_, '_> {
    fn spawn_border_gradient(&mut self, asset_server: &AssetServer, bg_type: BorderGradientType) {
        self.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            z_index: ZIndex::Local(1),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        ..default()
                    },
                    image: asset_server
                        .load(match bg_type {
                            BorderGradientType::Warning => "texture/warning_gradient.png",
                            BorderGradientType::Defense => "texture/defense_gradient.png",
                        })
                        .into(),
                    background_color: Color::WHITE.with_a(0.0).into(),
                    ..default()
                })
                .insert(BorderGradientComponent {
                    bg_type,
                    timer: {
                        let mut timer = Timer::from_seconds(BG_DURATION, TimerMode::Once);
                        timer.set_elapsed(Duration::from_secs_f32(BG_DURATION));
                        timer
                    },
                });
        });
    }
}

pub fn border_gradient_start_system(
    mut bg_query: Query<(&mut BorderGradientComponent, &mut BackgroundColor)>,
    mut bg_event_reader: EventReader<BorderGradientEvent>,
) {
    for event in bg_event_reader.read() {
        for (mut bg_component, mut background_color) in bg_query.iter_mut() {
            if bg_component.bg_type == *event {
                background_color.0.set_a(1.0);
                bg_component.timer.reset();
            }
        }
    }
}

pub fn border_gradient_update_system(
    mut bg_query: Query<(&mut BorderGradientComponent, &mut BackgroundColor)>,
    time: Res<Time>,
) {
    for (mut bg_component, mut background_color) in bg_query.iter_mut() {
        bg_component.timer.tick(time.delta());
        background_color
            .0
            .set_a(BG_MAX_ALPHA * f32::sin(PI * bg_component.timer.fraction()));
    }
}

/// Trigger a border gradient event mobs reach the bottom gate
pub fn border_gradient_on_gate_interaction(
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
