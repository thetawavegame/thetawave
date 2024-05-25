use bevy::{
    app::AppExit,
    asset::Handle,
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        schedule::NextState,
        system::ResMut,
    },
    hierarchy::{BuildChildren, ChildBuilder},
    log::info,
    render::{color::Color, texture::Image},
    sprite::TextureAtlasLayout,
    text::{Font, JustifyText, TextStyle},
    ui::{
        node_bundles::{AtlasImageBundle, ButtonBundle, NodeBundle, TextBundle},
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{player::PlayerInput, states::AppStates};

use crate::assets::UiAssets;

/// Event and Component for giving and sending menu buttons actions to move the user from
/// `AppStates::MainMenu` to `AppStates::CharacterSelection`, plus possibly a few digressions and
/// sprinkles.
#[derive(Component, Event, Clone, PartialEq, Eq, Copy, Debug)]
pub enum ButtonActionComponent {
    CharacterSelectReady(u8),
    CharacterSelectJoin,
    CharacterSelectRight(u8),
    CharacterSelectLeft(u8),
    EnterCharacterSelection,
    EnterOptions,
    EnterCompendium,
    QuitGame,
}

pub type ButtonActionEvent = ButtonActionComponent;

impl ButtonActionComponent {
    /// The label that will show on the main menu screen for the button representing this
    /// option/action
    pub fn text(&self) -> Option<&'static str> {
        match self {
            Self::EnterCharacterSelection => Some("Start Game"),
            Self::EnterOptions => Some("Options"),
            Self::EnterCompendium => Some("Compendium"),
            Self::QuitGame => Some("Exit Game"),
            Self::CharacterSelectLeft(_) => None,
            Self::CharacterSelectRight(_) => None,
            Self::CharacterSelectJoin => Some("Join"),
            Self::CharacterSelectReady(_) => Some("Ready"),
        }
    }

    /// Get input button animations to show on the button
    pub fn inputs(
        &self,
        ui_assets: &UiAssets,
        player_input: Option<PlayerInput>, // optional input method (keyboard/gamepad) of the player that the button belongs to
    ) -> Option<Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>> {
        match self {
            Self::CharacterSelectJoin => Some(vec![
                (
                    ui_assets.gamepad_button_a_image.clone(),
                    ui_assets.gamepad_button_a_layout.clone(),
                ),
                (
                    ui_assets.keyboard_key_return_image.clone(),
                    ui_assets.keyboard_key_return_layout.clone(),
                ),
            ]),
            Self::CharacterSelectReady { .. } => player_input.map(|player_input| {
                vec![
                    (match player_input {
                        PlayerInput::Keyboard => (
                            ui_assets.keyboard_key_return_image.clone(),
                            ui_assets.keyboard_key_return_layout.clone(),
                        ),
                        PlayerInput::Gamepad(_) => (
                            ui_assets.gamepad_button_a_image.clone(),
                            ui_assets.gamepad_button_a_layout.clone(),
                        ),
                    }),
                ]
            }),
            _ => None,
        }
    }

    /// External style applied to buttons
    /// Returns different styles based the based on the button action
    fn external_style(&self) -> Style {
        match self {
            Self::EnterCharacterSelection
            | Self::EnterOptions
            | Self::EnterCompendium
            | Self::QuitGame => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(25.0),
                min_width: Val::Px(200.0),
                aspect_ratio: Some(160.0 / 34.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
            Self::CharacterSelectJoin => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(100.0),
                min_width: Val::Px(200.0),
                aspect_ratio: Some(2.077_922),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
            Self::CharacterSelectLeft(_) | Self::CharacterSelectRight(_) => Style {
                height: Val::Percent(10.0),
                ..default()
            },
            Self::CharacterSelectReady { .. } => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(85.0),
                min_width: Val::Px(300.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
        }
    }

    /// Internal style of the ui inside of the button
    /// such as the text and the input symbols
    fn internal_style(&self) -> Style {
        match self {
            Self::EnterCharacterSelection
            | Self::EnterOptions
            | Self::EnterCompendium
            | Self::QuitGame
            | Self::CharacterSelectJoin => Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO),
                ..default()
            },
            Self::CharacterSelectLeft(_) | Self::CharacterSelectRight(_) => Style {
                padding: UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO),
                ..default()
            },
            Self::CharacterSelectReady { .. } => Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO),
                ..default()
            },
        }
    }

    /// Get image and texture atlas assets based on the button action
    fn asset(&self, ui_assets: &UiAssets) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        match self {
            Self::EnterCharacterSelection
            | Self::EnterOptions
            | Self::EnterCompendium
            | Self::QuitGame => (
                ui_assets.thetawave_menu_button_image.clone(),
                ui_assets.thetawave_menu_button_layout.clone(),
            ),
            Self::CharacterSelectRight(_) => (
                ui_assets.arrow_right_image.clone(),
                ui_assets.arrow_right_layout.clone(),
            ),
            Self::CharacterSelectLeft(_) => (
                ui_assets.arrow_left_image.clone(),
                ui_assets.arrow_left_layout.clone(),
            ),
            Self::CharacterSelectJoin => (
                ui_assets.large_menu_button_image.clone(),
                ui_assets.large_menu_button_layout.clone(),
            ),
            Self::CharacterSelectReady { .. } => (
                ui_assets.thetawave_menu_button_image.clone(),
                ui_assets.thetawave_menu_button_layout.clone(),
            ),
        }
    }
}

// Handles state changes for buttons. This runs when a user actually
// clicks/whacks enter on a button in the main menu
pub fn button_action_change_state_system(
    mut button_event_reader: EventReader<ButtonActionEvent>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut exit: EventWriter<AppExit>,
) {
    for event in button_event_reader.read() {
        match event {
            ButtonActionComponent::EnterCharacterSelection => {
                next_app_state.set(AppStates::CharacterSelection);
            }
            ButtonActionComponent::EnterOptions => info!("Enter options menu."),
            ButtonActionComponent::EnterCompendium => info!("Enter compendium."),
            ButtonActionComponent::QuitGame => {
                exit.send(AppExit);
            }
            _ => {}
        }
    }
}

/// Extension trait for spawning customized UI elements for Thetawave
pub trait UiButtonChildBuilderExt {
    /// Spawn a Thetawave-stylized menu button
    fn spawn_button(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        action: ButtonActionComponent,
        player_input: Option<&PlayerInput>,
    );
}

impl UiButtonChildBuilderExt for ChildBuilder<'_> {
    fn spawn_button(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        action: ButtonActionComponent,
        player_input: Option<&PlayerInput>,
    ) {
        // Spawn button bundle entity, with a child entity containing the texture
        self.spawn(ButtonBundle {
            style: action.external_style(),
            background_color: BackgroundColor(Color::NONE),
            ..default()
        })
        .insert(action)
        .with_children(|parent| {
            let button_asset = action.asset(ui_assets);

            parent
                .spawn(AtlasImageBundle {
                    image: button_asset.0.into(),
                    texture_atlas: button_asset.1.into(),
                    style: action.internal_style(),
                    ..default()
                })
                .with_children(|parent| {
                    if let Some(text) = action.text() {
                        parent.spawn(
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                    }

                    if let Some(inputs) = action.inputs(ui_assets, player_input.copied()) {
                        // Row for all button inputs
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    flex_direction: FlexDirection::Row,
                                    height: Val::Percent(35.0),
                                    margin: UiRect::top(Val::Vh(1.0)),
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                for (image, layout) in inputs {
                                    parent.spawn(AtlasImageBundle {
                                        image: image.into(),
                                        texture_atlas: layout.into(),
                                        style: Style {
                                            margin: UiRect {
                                                left: Val::Vw(0.5),
                                                right: Val::Vw(0.5),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        ..default()
                                    });
                                }
                            });
                    }
                });
        });
    }
}
