use bevy::{
    hierarchy::{BuildChildren, ChildBuilder},
    prelude::Component,
    render::color::Color,
    ui::{node_bundles::ButtonBundle, BackgroundColor, Style, UiRect},
    utils::default,
};

pub trait UIChildBuilderExt {
    pub fn spawn_menu_button(&mut self, text: String);
}

impl UIChildBuilderExt for ChildBuilder {
    fn spawn_menu_button(&mut self, text: String) {
        self.spawn(ButtonBundle {
            style: Style {
                width: Val::Percent(20.0),
                max_width: Val::Px(300.0),
                min_height: Val::Percent(5.0),
                border: UiRect::all(Val::px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect {
                    bottom: Val::Percent(3.0),
                    top: Val::Auto,
                    right: Val::Auto,
                    left: Val::Auto,
                },
                ..default()
            },
            border_color: BorderColor(Color::RED),
            background_color: BackgroundColor(Color::GREEN),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font: font.clone(),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
    }
}
