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
        node_bundles::{AtlasImageBundle, ButtonBundle, TextBundle},
        AlignItems, BackgroundColor, JustifyContent, Style, UiRect, Val,
    },
    utils::default,
};
use thetawave_interface::states::AppStates;

use crate::assets::UiAssets;

const BUTTON_WIDTH: Val = Val::Percent(25.0);
const BUTTON_MAX_WIDTH: Val = Val::Px(500.0);
const BUTTON_MIN_WIDTH: Val = Val::Px(200.0);
const BUTTON_MARGIN: UiRect =
    UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0));
const BUTTON_ASPECT_RATIO: Option<f32> = Some(160.0 / 34.0);
const BUTTON_TEXTURE_PADDING: UiRect =
    UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(9.0), Val::ZERO);
const BUTTON_TEXTURE_PADDING_HOVERED: UiRect =
    UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(10.5), Val::ZERO);

/// Event and Component for giving and sending menu buttons actions to move the user from
/// `AppStates::MainMenu` to `AppStates::CharacterSelection`, plus possibly a few digressions and
/// sprinkles.
#[derive(Component, Event, Clone, PartialEq, Eq, Copy, Debug)]
pub enum ButtonActionComponent {
    CharacterSelectJoin,
    CharacterSelectRight,
    CharacterSelectLeft,
    EnterCharacterSelection,
    EnterOptions,
    EnterCompendium,
    QuitGame,
}

pub type ButtonActionEvent = ButtonActionComponent;

impl ButtonActionComponent {
    /// The label that will show on the main menu screen for the button representing this
    /// option/action
    pub fn in_game_text(&self) -> Option<&'static str> {
        match self {
            Self::EnterCharacterSelection => Some("Start Game"),
            Self::EnterOptions => Some("Options"),
            Self::EnterCompendium => Some("Compendium"),
            Self::QuitGame => Some("Exit Game"),
            Self::CharacterSelectLeft => None,
            Self::CharacterSelectRight => None,
            Self::CharacterSelectJoin => Some("Press to\njoin"),
        }
    }

    fn external_style(&self) -> Style {
        match self {
            Self::EnterCharacterSelection
            | Self::EnterOptions
            | Self::EnterCompendium
            | Self::QuitGame => Style {
                max_width: BUTTON_MAX_WIDTH,
                width: BUTTON_WIDTH,
                min_width: BUTTON_MIN_WIDTH,
                aspect_ratio: BUTTON_ASPECT_RATIO,
                margin: BUTTON_MARGIN,
                ..default()
            },
            Self::CharacterSelectJoin => Style {
                max_width: Val::Px(500.0),
                width: Val::Percent(100.0),
                min_width: Val::Px(200.0),
                aspect_ratio: Some(2.0779221),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(1.0), Val::Percent(1.0)),
                ..default()
            },
            Self::CharacterSelectLeft | Self::CharacterSelectRight => Style { ..default() },
        }
    }

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
                align_items: AlignItems::FlexStart,
                padding: BUTTON_TEXTURE_PADDING,
                ..default()
            },
            Self::CharacterSelectLeft | Self::CharacterSelectRight => Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: BUTTON_TEXTURE_PADDING,
                ..default()
            },
        }
    }

    fn asset(&self, ui_assets: &UiAssets) -> (Handle<Image>, Handle<TextureAtlasLayout>) {
        match self {
            Self::EnterCharacterSelection
            | Self::EnterOptions
            | Self::EnterCompendium
            | Self::QuitGame => (
                ui_assets.thetawave_menu_button_image.clone(),
                ui_assets.thetawave_menu_button_layout.clone(),
            ),
            Self::CharacterSelectLeft | Self::CharacterSelectRight => (
                ui_assets.arrow_image.clone(),
                ui_assets.arrow_layout.clone(),
            ),
            Self::CharacterSelectJoin => (
                ui_assets.large_menu_button_image.clone(),
                ui_assets.large_menu_button_layout.clone(),
            ),
        }
    }
}

// Handles actions for menu buttons, changeing states, quitting. This runs when a user actually
// clicks/whacks enter on a button in the main menu
pub fn button_on_click_system(
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
            ButtonActionComponent::CharacterSelectRight => info!("Character selection right."),
            ButtonActionComponent::CharacterSelectLeft => info!("Character selection left."),
            ButtonActionComponent::CharacterSelectJoin => info!("Character selection join."),
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
    );
}

impl UiButtonChildBuilderExt for ChildBuilder<'_> {
    fn spawn_button(
        &mut self,
        ui_assets: &UiAssets,
        font: Handle<Font>,
        action: ButtonActionComponent,
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
                    if let Some(text) = action.in_game_text() {
                        parent.spawn(
                            TextBundle::from_section(
                                text,
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 30.0,
                                    color: Color::BLACK,
                                },
                            )
                            .with_text_justify(JustifyText::Center)
                            .with_background_color(Color::BLUE.with_a(0.3)), // TODO: remove after testing
                        );
                    }
                });
        });
    }
}
