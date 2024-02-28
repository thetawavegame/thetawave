use bevy::{
    asset::{self, AssetServer, Handle},
    ecs::{
        component::Component,
        query::{Changed, With},
        system::{Query, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children},
    log::info,
    render::{color::Color, texture::Image},
    text::{Font, Text, TextStyle},
    transform::components::Transform,
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, Style, UiImage,
        UiRect, Val,
    },
    utils::default,
};

const BUTTON_WIDTH: Val = Val::Percent(25.0);
const BUTTON_MAX_WIDTH: Val = Val::Px(500.0);
const BUTTON_MIN_WIDTH: Val = Val::Px(200.0);
const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0));
const BUTTON_UP_PATH: &str = "texture/menu_button_up.png";
const BUTTON_DOWN_PATH: &str = "texture/menu_button_down.png";

#[derive(Component)]
pub struct ThetawaveUiButtonComponent;

pub trait UiChildBuilderExt {
    fn spawn_menu_button(&mut self, asset_server: &AssetServer, text: String, font: Handle<Font>);
}

impl UiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_menu_button(&mut self, asset_server: &AssetServer, text: String, font: Handle<Font>) {
        self.spawn(ButtonBundle {
            style: Style {
                max_width: BUTTON_MAX_WIDTH,
                width: BUTTON_WIDTH,
                min_width: BUTTON_MIN_WIDTH,
                aspect_ratio: Some(160.0 / 34.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexStart,
                margin: BUTTON_MARGIN,
                padding: UiRect::top(Val::Percent(2.2)),
                ..default()
            },
            image: asset_server.load(BUTTON_UP_PATH).into(),
            ..default()
        })
        .insert(ThetawaveUiButtonComponent)
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    text,
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::BLACK,
                    },
                )
                .with_style(Style {
                    //margin: UiRect::top(Val::Percent(44.0)),
                    ..default()
                }),
            );
        });
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (&mut UiImage, &Interaction, &mut Style, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (mut uiimage, interaction, mut style, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                *uiimage = asset_server.load(BUTTON_DOWN_PATH).into();
                style.padding = UiRect::top(Val::Percent(2.4));
            }
            Interaction::None => {
                *uiimage = asset_server.load(BUTTON_UP_PATH).into();
                style.padding = UiRect::top(Val::Percent(2.2));
            }
        }
    }
}
