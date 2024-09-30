use bevy::{
    app::AppExit,
    asset::Handle,
    color::{Color, Srgba},
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::ResMut,
    },
    hierarchy::{BuildChildren, ChildBuilder},
    log::info,
    prelude::{ImageBundle, NextState},
    render::texture::Image,
    sprite::{TextureAtlas, TextureAtlasLayout},
    text::{Font, JustifyText, TextStyle},
    ui::{
        node_bundles::{ButtonBundle, NodeBundle, TextBundle},
        AlignItems, BackgroundColor, FlexDirection, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::{player::PlayerInput, states::AppStates};

use crate::assets::UiAssets;

/// Event and Component for giving and sending menu buttons actions to move the user from
/// `AppStates::MainMenu` to `AppStates::CharacterSelection`, plus possibly a few digressions and
/// sprinkles.
#[derive(Clone, PartialEq, Eq, Copy, Debug)]
pub(super) enum ButtonActionType {
    CharacterSelectReady(u8),
    CharacterSelectJoin,
    CharacterSelectRight(u8),
    CharacterSelectLeft(u8),
    EnterCharacterSelection,
    EnterOptions,
    EnterCompendium,
    QuitGame,
}

#[derive(Component, Clone, PartialEq, Eq, Copy, Debug)]
pub(super) struct ButtonActionComponent {
    pub action: ButtonActionType,
}

impl From<ButtonActionType> for ButtonActionComponent {
    fn from(value: ButtonActionType) -> Self {
        ButtonActionComponent { action: value }
    }
}

#[derive(Event, Clone, PartialEq, Eq, Copy, Debug)]
pub(super) struct ButtonActionEvent {
    pub action: ButtonActionType,
}

impl From<ButtonActionType> for ButtonActionEvent {
    fn from(value: ButtonActionType) -> Self {
        ButtonActionEvent { action: value }
    }
}

impl ButtonActionComponent {
    /// The label that will show on the main menu screen for the button representing this
    /// option/action
    fn text(&self) -> Option<&'static str> {
        match self.action {
            ButtonActionType::EnterCharacterSelection => Some("Start Game"),
            ButtonActionType::EnterOptions => Some("Options"),
            ButtonActionType::EnterCompendium => Some("Compendium"),
            ButtonActionType::QuitGame => Some("Exit Game"),
            ButtonActionType::CharacterSelectLeft(_) => None,
            ButtonActionType::CharacterSelectRight(_) => None,
            ButtonActionType::CharacterSelectJoin => Some("Join"),
            ButtonActionType::CharacterSelectReady(_) => Some("Ready"),
        }
    }

    /// Get input button animations to show on the button
    fn get_input_images(
        &self,
        ui_assets: &UiAssets,
        player_input: Option<PlayerInput>, // optional input method (keyboard/gamepad) of the player that the button belongs to
    ) -> Option<Vec<(Handle<Image>, Handle<TextureAtlasLayout>)>> {
        match self.action {
            ButtonActionType::CharacterSelectJoin => Some(vec![
                (
                    ui_assets.gamepad_button_a_image.clone(),
                    ui_assets.gamepad_button_a_layout.clone(),
                ),
                (
                    ui_assets.keyboard_key_return_image.clone(),
                    ui_assets.keyboard_key_return_layout.clone(),
                ),
            ]),
            ButtonActionType::CharacterSelectReady { .. } => player_input.map(|player_input| {
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
    fn get_external_style(&self) -> Style {
        match self.action {
            ButtonActionType::EnterCharacterSelection
            | ButtonActionType::EnterOptions
            | ButtonActionType::EnterCompendium
            | ButtonActionType::QuitGame => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(25.0),
                min_width: Val::Px(200.0),
                aspect_ratio: Some(160.0 / 34.0),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
            ButtonActionType::CharacterSelectJoin => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(100.0),
                min_width: Val::Px(200.0),
                aspect_ratio: Some(2.077_922),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
            ButtonActionType::CharacterSelectLeft(_)
            | ButtonActionType::CharacterSelectRight(_) => Style {
                height: Val::Percent(10.0),
                ..default()
            },
            ButtonActionType::CharacterSelectReady { .. } => Style {
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
    fn get_internal_style(&self) -> Style {
        match self.action {
            ButtonActionType::EnterCharacterSelection
            | ButtonActionType::EnterOptions
            | ButtonActionType::EnterCompendium
            | ButtonActionType::QuitGame
            | ButtonActionType::CharacterSelectJoin => Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO),
                ..default()
            },
            ButtonActionType::CharacterSelectLeft(_)
            | ButtonActionType::CharacterSelectRight(_) => Style {
                padding: UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO),
                ..default()
            },
            ButtonActionType::CharacterSelectReady { .. } => Style {
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
        match self.action {
            ButtonActionType::EnterCharacterSelection
            | ButtonActionType::EnterOptions
            | ButtonActionType::EnterCompendium
            | ButtonActionType::QuitGame => (
                ui_assets.thetawave_menu_button_image.clone(),
                ui_assets.thetawave_menu_button_layout.clone(),
            ),
            ButtonActionType::CharacterSelectRight(_) => (
                ui_assets.arrow_right_image.clone(),
                ui_assets.arrow_right_layout.clone(),
            ),
            ButtonActionType::CharacterSelectLeft(_) => (
                ui_assets.arrow_left_image.clone(),
                ui_assets.arrow_left_layout.clone(),
            ),
            ButtonActionType::CharacterSelectJoin => (
                ui_assets.large_menu_button_image.clone(),
                ui_assets.large_menu_button_layout.clone(),
            ),
            ButtonActionType::CharacterSelectReady { .. } => (
                ui_assets.thetawave_menu_button_image.clone(),
                ui_assets.thetawave_menu_button_layout.clone(),
            ),
        }
    }
}

// Handles state changes for buttons. This runs when a user actually
// clicks/whacks enter on a button in the main menu
pub(super) fn button_action_change_state_system(
    mut button_event_reader: EventReader<ButtonActionEvent>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut exit: EventWriter<AppExit>,
) {
    for event in button_event_reader.read() {
        match event.action {
            ButtonActionType::EnterCharacterSelection => {
                next_app_state.set(AppStates::CharacterSelection);
            }
            ButtonActionType::EnterOptions => info!("Enter options menu."),
            ButtonActionType::EnterCompendium => info!("Enter compendium."),
            ButtonActionType::QuitGame => {
                exit.send(AppExit::Success);
            }
            _ => {}
        }
    }
}

/// Extension trait for spawning customized UI elements for Thetawave
pub(super) trait UiButtonChildBuilderExt {
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
            style: action.get_external_style(),
            background_color: BackgroundColor(Color::Srgba(Srgba::NONE)),
            ..default()
        })
        .insert(action)
        .with_children(|parent| {
            let button_asset = action.asset(ui_assets);

            parent
                .spawn(ImageBundle {
                    image: button_asset.0.into(),
                    style: action.get_internal_style(),
                    ..default()
                })
                .insert(TextureAtlas {
                    layout: button_asset.1.into(),
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

                    if let Some(inputs) = action.get_input_images(ui_assets, player_input.copied())
                    {
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
                                    parent
                                        .spawn(ImageBundle {
                                            image: image.into(),
                                            style: Style {
                                                margin: UiRect {
                                                    left: Val::Vw(0.5),
                                                    right: Val::Vw(0.5),
                                                    ..default()
                                                },
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .insert(TextureAtlas {
                                            layout: layout.into(),
                                            ..default()
                                        });
                                }
                            });
                    }
                });
        });
    }
}
