use bevy::{
    app::AppExit,
    asset::Handle,
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        schedule::NextState,
        system::{Query, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children},
    log::info,
    render::color::Color,
    sprite::TextureAtlas,
    text::{Font, TextStyle},
    ui::{
        node_bundles::{AtlasImageBundle, ButtonBundle, TextBundle},
        widget::Button,
        AlignItems, BackgroundColor, Interaction, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    states::AppStates,
};

use crate::assets::UiAssets;

const BUTTON_WIDTH: Val = Val::Percent(25.0);
const BUTTON_MAX_WIDTH: Val = Val::Px(500.0);
const BUTTON_MIN_WIDTH: Val = Val::Px(200.0);
const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0));

#[derive(Component, Event, Clone)]
pub enum ThetawaveUiButtonActionComponent {
    EnterInstructions,
    EnterOptions,
    EnterGame,
    EnterCompendium,
    QuitGame,
}

pub type ThetawaveUiButtonActionEvent = ThetawaveUiButtonActionComponent;

pub trait UiChildBuilderExt {
    fn spawn_menu_button(
        &mut self,
        ui_assets: &UiAssets,
        text: String,
        font: Handle<Font>,
        action: ThetawaveUiButtonActionComponent,
    );
}

impl UiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_menu_button(
        &mut self,
        ui_assets: &UiAssets,
        text: String,
        font: Handle<Font>,
        action: ThetawaveUiButtonActionComponent,
    ) {
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
        .insert(action)
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

pub fn button_interaction_system(
    mut interaction_query: Query<
        (&ThetawaveUiButtonActionComponent, &Interaction, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_texture_query: Query<(&mut TextureAtlas, &mut Style)>,
    mut sound_effect: EventWriter<PlaySoundEffectEvent>,
    mut button_event_writer: EventWriter<ThetawaveUiButtonActionEvent>,
) {
    for (action, interaction, children) in &mut interaction_query {
        let (mut texture_atlas, mut style) = button_texture_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                button_event_writer.send(action.clone());
            }
            Interaction::Hovered => {
                texture_atlas.index = 1;
                style.padding = UiRect::top(Val::Percent(10.5));
                sound_effect.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::ButtonSelect,
                });
            }
            Interaction::None => {
                texture_atlas.index = 0;
                style.padding = UiRect::top(Val::Percent(9.0));
                sound_effect.send(PlaySoundEffectEvent {
                    sound_effect_type: SoundEffectType::ButtonRelease,
                });
            }
        }
    }
}

pub fn button_action_system(
    mut button_event_reader: EventReader<ThetawaveUiButtonActionEvent>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut exit: EventWriter<AppExit>,
) {
    for event in button_event_reader.read() {
        match event {
            ThetawaveUiButtonActionComponent::EnterInstructions => {
                next_app_state.set(AppStates::Instructions);
            }
            ThetawaveUiButtonActionComponent::EnterOptions => info!("Enter options menu."),
            ThetawaveUiButtonActionComponent::EnterGame => info!("Enter game."),
            ThetawaveUiButtonActionComponent::EnterCompendium => info!("Enter compendium."),
            ThetawaveUiButtonActionComponent::QuitGame => {
                exit.send(AppExit);
            }
        }
    }
}
