use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::{color::Color, view::Visibility},
    time::{Timer, TimerMode},
    ui::{
        node_bundles::{ImageBundle, NodeBundle},
        FlexDirection, Style, Val, ZIndex,
    },
    utils::default,
};

const BG_DURATION: f32 = 1.0;

pub enum BorderGradientType {
    Warning,
    Defense,
}

#[derive(Component)]
pub struct BorderGradientComponent {
    bg_type: BorderGradientType,
    timer: Timer,
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
                    background_color: Color::WHITE.with_a(0.50).into(),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(BorderGradientComponent {
                    bg_type,
                    timer: Timer::from_seconds(BG_DURATION, TimerMode::Once),
                });
        });
    }
}
