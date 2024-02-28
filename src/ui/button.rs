use bevy::{
    asset::{self, AssetServer, Handle},
    ecs::{
        component::Component,
        event::EventWriter,
        query::{Changed, With},
        system::{Query, Res},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children},
    log::info,
    render::{color::Color, texture::Image},
    sprite::TextureAtlas,
    text::{Font, Text, TextStyle},
    time::Time,
    transform::components::Transform,
    ui::{
        node_bundles::{AtlasImageBundle, ButtonBundle, NodeBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, BorderColor, Interaction, JustifyContent, Style, UiImage,
        UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};

use crate::assets::UiAssets;

const BUTTON_WIDTH: Val = Val::Percent(25.0);
const BUTTON_MAX_WIDTH: Val = Val::Px(500.0);
const BUTTON_MIN_WIDTH: Val = Val::Px(200.0);
const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0));
const BUTTON_UP_PATH: &str = "texture/menu_button.png";
const BUTTON_DOWN_PATH: &str = "texture/menu_button_selected.png";

#[derive(Component)]
pub struct ThetawaveUiButtonComponent;

pub trait UiChildBuilderExt {
    fn spawn_menu_button(&mut self, ui_assets: &UiAssets, text: String, font: Handle<Font>);
}

impl UiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_menu_button(&mut self, ui_assets: &UiAssets, text: String, font: Handle<Font>) {
        self.spawn(ButtonBundle {
            style: Style {
                max_width: BUTTON_MAX_WIDTH,
                width: BUTTON_WIDTH,
                min_width: BUTTON_MIN_WIDTH,
                aspect_ratio: Some(160.0 / 34.0),

                margin: BUTTON_MARGIN,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
            //image: asset_server.load(BUTTON_UP_PATH).into(),
            ..default()
        })
        .insert(ThetawaveUiButtonComponent)
        .with_children(|parent| {
            parent
                .spawn(AtlasImageBundle {
                    image: ui_assets.thetawave_menu_button_image.clone().into(),
                    texture_atlas: ui_assets.thetawave_menu_button_layout.clone().into(),
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::top(Val::Percent(9.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        text,
                        TextStyle {
                            font: font.clone(),
                            font_size: 30.0,
                            color: Color::BLACK,
                        },
                    ));
                });
        });
    }
}

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut button_texture_query: Query<(&mut TextureAtlas, &mut Style)>,
    mut sound_effect: EventWriter<PlaySoundEffectEvent>,
    time: Res<Time>,
) {
    for (interaction, children) in &mut interaction_query {
        let (mut texture_atlas, mut style) = button_texture_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                texture_atlas.index = 1;
                style.padding = UiRect::top(Val::Percent(10.5));
                sound_effect.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::ButtonSelect,
                });
            }
            Interaction::None => {
                if time.elapsed_seconds() > 1.0 {
                    texture_atlas.index = 0;
                    style.padding = UiRect::top(Val::Percent(9.0));
                    sound_effect.send(PlaySoundEffectEvent {
                        sound_effect_type: SoundEffectType::ButtonRelease,
                    });
                }
            }
        }
    }
}
