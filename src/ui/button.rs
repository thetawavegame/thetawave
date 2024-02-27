use bevy::{
    asset::Handle,
    hierarchy::ChildBuilder,
    render::texture::Image,
    ui::{node_bundles::ButtonBundle, AlignItems, JustifyContent, Style, UiRect, Val},
    utils::default,
};

const BUTTON_WIDTH: Val = Val::Percent(20.0);
const BUTTON_MAX_WIDTH: Val = Val::Px(500.0);
const BUTTON_MIN_WIDTH: Val = Val::Px(125.0);
const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0));

pub trait UIChildBuilderExt {
    fn spawn_menu_button(&mut self, image: Handle<Image>);
}

impl UIChildBuilderExt for ChildBuilder<'_> {
    fn spawn_menu_button(&mut self, image: Handle<Image>) {
        self.spawn(ButtonBundle {
            style: Style {
                max_width: BUTTON_MAX_WIDTH,
                width: BUTTON_WIDTH,
                min_width: BUTTON_MIN_WIDTH,
                aspect_ratio: Some(132.0 / 32.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: BUTTON_MARGIN,
                ..default()
            },
            image: image.into(),
            ..default()
        });
    }
}
