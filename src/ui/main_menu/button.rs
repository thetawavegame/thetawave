//! Provides the layout (trait on `bevy::hierarchy::ChildBUilder`) and behavior (systems) to put 4
//! vertically layed out on the main menu, and change the state from
//! `thetawave_interface::states::AppStates::MainMenu` to
//! `thetawave_interface::states::AppStates::Instructions`
use crate::assets::UiAssets;
use bevy::{
    app::AppExit,
    asset::Handle,
    ecs::{
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::{Changed, With},
        schedule::NextState,
        system::{Local, Query, ResMut},
    },
    hierarchy::{BuildChildren, ChildBuilder, Children},
    log::{error, info},
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
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    input::{MenuAction, MenuExplorer},
    states::AppStates,
};

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
/// `AppStates::MainMenu` to `AppStates::Instructions`, plus possibly a few digressions and
/// sprinkles.
#[derive(Component, Event, Clone, PartialEq, Eq, Copy, Debug)]
pub(super) enum MainMenuButtonActionComponent {
    EnterInstructions,
    EnterOptions,
    EnterCompendium,
    QuitGame,
}

impl MainMenuButtonActionComponent {
    /// The label that will show on the main menu screen for the button representing this
    /// option/action
    fn in_game_text(&self) -> &'static str {
        match self {
            Self::EnterInstructions => "Start Game",
            Self::EnterOptions => "Options",
            Self::EnterCompendium => "Compendium",
            Self::QuitGame => "Exit Game",
        }
    }
}
/// This is the order (vertical, going down) of the buttons shown on the main menu UI.
const MAIN_MENU_BUTTON_ORDER: [MainMenuButtonActionComponent; 4] = [
    MainMenuButtonActionComponent::EnterInstructions,
    MainMenuButtonActionComponent::EnterOptions,
    MainMenuButtonActionComponent::EnterCompendium,
    MainMenuButtonActionComponent::QuitGame,
];

pub(super) type MainMenuButtonActionEvent = MainMenuButtonActionComponent;

/// Extension trait for spawning customized UI elements for Thetawave
pub(super) trait UiChildBuilderExt {
    /// Spawn a Thetawave-stylized menu button
    fn spawn_main_menu_button(
        &mut self,
        ui_assets: &UiAssets,
        text: String,
        font: Handle<Font>,
        action: MainMenuButtonActionComponent,
    );
    // Spawn 1 menu button for each element of `MainMenuButtonActionComponent`
    fn spawn_main_menu_buttons(&mut self, ui_assets: &UiAssets, font: Handle<Font>) -> &mut Self;
}

impl UiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_main_menu_buttons(&mut self, ui_assets: &UiAssets, font: Handle<Font>) -> &mut Self {
        for action in MAIN_MENU_BUTTON_ORDER.iter() {
            self.spawn_main_menu_button(
                ui_assets,
                action.in_game_text().into(),
                font.clone(),
                action.clone(),
            )
        }

        self
    }
    fn spawn_main_menu_button(
        &mut self,
        ui_assets: &UiAssets,
        text: String,
        font: Handle<Font>,
        action: MainMenuButtonActionComponent,
    ) {
        // Spawn button bundle entity, with a child entity containing the texture
        self.spawn(ButtonBundle {
            style: Style {
                max_width: BUTTON_MAX_WIDTH,
                width: BUTTON_WIDTH,
                min_width: BUTTON_MIN_WIDTH,
                aspect_ratio: BUTTON_ASPECT_RATIO,
                margin: BUTTON_MARGIN,
                ..default()
            },
            background_color: BackgroundColor(Color::NONE),
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
                        padding: BUTTON_TEXTURE_PADDING,
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

type TButtonIdx = i16;
// Roughly `(val1 + episilon) % modulo` except keeps everything positive and wraps to ensure that
// numbers stay small and that the returned value is `0<=returned_value < modulo`
fn wrapped_modulo_add(val1: TButtonIdx, epsilon: i8, modulo: usize) -> TButtonIdx {
    ((((val1 as isize) + (epsilon as isize)).rem_euclid(modulo as isize))
        .rem_euclid(TButtonIdx::MAX as isize)) as TButtonIdx
}
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ButtonSelectionCause {
    MouseOver,
    UpDownInputs,
}
#[derive(Debug, Default)]
pub(super) struct MainMenuUIState {
    current_selected_button_and_cause: Option<(TButtonIdx, ButtonSelectionCause)>,
}
#[inline]
fn bool_to_plus_minus_1(val: bool) -> i8 {
    match val {
        true => -1,
        false => 1,
    }
}
/// Selects a button on the main menu to click. Mainly sends a `MainMenuButtonActionEvent`.
/// Selection happens from the mouse, keyboard and gamepad. We deal with all kinds of inputs in 1
/// system to control the interactionsm between using, for example, arrows and hovers.
pub(super) fn main_menu_button_selection_and_click_system(
    main_menu_buttons: Query<(&MainMenuButtonActionComponent, &Children), With<Button>>,
    main_menu_button_mouse_movements: Query<
        (&MainMenuButtonActionComponent, &Interaction),
        With<Button>,
    >,
    main_menu_button_mouse_changed_movements: Query<
        (&MainMenuButtonActionComponent, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    menu_explorer_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut button_texture_query: Query<(&mut TextureAtlas, &mut Style)>,
    // Index into `MAIN_MENU_BUTTON_ORDER`, possibly mod its size
    mut ui_state: Local<MainMenuUIState>,
    // The main side effects of this system/UI component/widget
    mut sound_effect: EventWriter<PlaySoundEffectEvent>,
    mut button_event_writer: EventWriter<MainMenuButtonActionEvent>,
) {
    // 1. Compute some facts about the current ui state and compute the next frame's ui state
    // 2. Send out sound effect events.
    // 3. Send out any events for "button clicked" actions
    // 4. Set the styling so that only that one button looks "pressed" while all other are inactive
    let currently_hovered_on_button: Option<&MainMenuButtonActionComponent> =
        main_menu_button_mouse_movements
            .iter()
            .find_map(|(action, x)| match x {
                Interaction::Hovered => Some(action),
                _ => None,
            });
    // Apply d-pad/arrow keys. true = up, false = down
    let contribution_from_arrow_inputs: Option<bool> = match menu_explorer_query.get_single() {
        Err(_) => None,
        Ok(x) => x
            .get_just_pressed()
            .iter()
            .find_map(|action_| match action_ {
                MenuAction::NavigateUp => Some(true),
                MenuAction::NavigateDown => Some(false),
                _ => None,
            }),
    };
    let player_confirmed_button_selection: Option<MainMenuButtonActionComponent> =
        menu_explorer_query
            .get_single()
            .ok()
            .map(|x| match &ui_state.current_selected_button_and_cause {
                Some((idx, _)) if x.just_released(&MenuAction::Confirm) => Some(
                    MAIN_MENU_BUTTON_ORDER[(*idx as usize % MAIN_MENU_BUTTON_ORDER.len()) as usize]
                        .clone(),
                ),
                _ => None,
            })
            .flatten();
    // Note that this uses the Changed<_> query filter, which allows us to detect when the mouse
    // was clicked THEN RELEASED. Using the `main_menu_button_mouse_movements` query would make
    // this Some whenever the mouse is just pressed down on a button.
    let first_button_mouse_clicked: Option<MainMenuButtonActionComponent> =
        main_menu_button_mouse_changed_movements
            .iter()
            .find_map(|(res, interaction)| {
                if *interaction == Interaction::Pressed {
                    Some(res.clone())
                } else {
                    None
                }
            });
    // This is a bit subtle because some branches can be swapped, while some are order-dependent
    let next_frame_ui_state: MainMenuUIState = match (
        &ui_state.current_selected_button_and_cause,
        contribution_from_arrow_inputs,
        currently_hovered_on_button,
    ) {
        // Hovering overrides everything, so this is checked "first"
        (_, _, Some(currently_hovered_on_button)) => MainMenuUIState {
            current_selected_button_and_cause: MAIN_MENU_BUTTON_ORDER
                .iter()
                .position(|x| x == currently_hovered_on_button)
                .map(|idx| (idx as TButtonIdx, ButtonSelectionCause::MouseOver)),
        },
        // Initial movement using arrows (no mouse input/hover). Start at 1st (top) button.
        (None, Some(_), None) => MainMenuUIState {
            current_selected_button_and_cause: Some((0, ButtonSelectionCause::UpDownInputs)),
        },
        // button moused over -> no button moused over, without arrow inputs
        (Some((_, ButtonSelectionCause::MouseOver)), None, None) => MainMenuUIState {
            current_selected_button_and_cause: None,
        },

        // Arrow keys w/o mouse hovering
        (Some((idx, _)), Some(arrow_contrib), None) => MainMenuUIState {
            current_selected_button_and_cause: Some((
                wrapped_modulo_add(
                    *idx,
                    bool_to_plus_minus_1(arrow_contrib),
                    MAIN_MENU_BUTTON_ORDER.len(),
                ),
                ButtonSelectionCause::UpDownInputs,
            )),
        },
        // First few frames of the main menu. Nothing is selected and the user hasn't done any
        // input
        (None, None, None) => MainMenuUIState::default(),
        // catch all...keep the ui state as is. Keep this last in the match branches
        (x, _, _) => MainMenuUIState {
            current_selected_button_and_cause: *x,
        },
    };

    let next_frame_button_state: Option<MainMenuButtonActionComponent> = next_frame_ui_state
        .current_selected_button_and_cause
        .map(|(idx, _)| {
            MAIN_MENU_BUTTON_ORDER
                [(idx.rem_euclid(MAIN_MENU_BUTTON_ORDER.len() as TButtonIdx)) as usize]
        });

    // Side effects/fire off events
    match (
        &ui_state.current_selected_button_and_cause,
        &next_frame_ui_state.current_selected_button_and_cause,
    ) {
        (Some((_, _)), None) => {
            sound_effect.send(PlaySoundEffectEvent {
                sound_effect_type: SoundEffectType::ButtonRelease,
            });
        }
        (Some((old, _)), Some((new, _))) if *old != *new => {
            sound_effect.send(PlaySoundEffectEvent {
                sound_effect_type: SoundEffectType::ButtonSelect,
            });
        }
        (None, Some(_)) => {
            sound_effect.send(PlaySoundEffectEvent {
                sound_effect_type: SoundEffectType::ButtonSelect,
            });
        }
        _ => {}
    };
    if let Some(action) = first_button_mouse_clicked.or(player_confirmed_button_selection) {
        button_event_writer.send(action.clone());
        sound_effect.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::ButtonConfirm,
        });
    }
    if contribution_from_arrow_inputs.is_some() {
        info!(
            "Arrows: {:?}, current_idx: {:?}      next_idx: {:?}, next_buttonState: {:?}",
            contribution_from_arrow_inputs,
            &ui_state.current_selected_button_and_cause.map(|x| x.0),
            &next_frame_ui_state,
            &next_frame_button_state
        );
    }

    // Update the sprite sheets of each buttons to animate/"select" exactly 0 or 1.
    for (action, children) in main_menu_buttons.iter() {
        if let Some(button_child_entity) = children.first() {
            if let Ok((mut texture_atlas, mut style)) =
                button_texture_query.get_mut(*button_child_entity)
            {
                if next_frame_button_state == Some(*action) {
                    texture_atlas.index = 1;
                    style.padding = BUTTON_TEXTURE_PADDING_HOVERED;
                } else {
                    texture_atlas.index = 0;
                    style.padding = BUTTON_TEXTURE_PADDING;
                }
            } else {
                error!("Button sprite sheet not found");
            }
        } else {
            error!("Childless main menu button");
        }
    }
    // This MUST come last
    *ui_state = next_frame_ui_state;
}

// Handles actions for menu buttons, changeing states, quitting. This runs when a user actually
// clicks/whacks enter on a button in the main menu
pub(super) fn main_menu_button_on_click_system(
    mut button_event_reader: EventReader<MainMenuButtonActionEvent>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut exit: EventWriter<AppExit>,
) {
    for event in button_event_reader.read() {
        match event {
            MainMenuButtonActionComponent::EnterInstructions => {
                next_app_state.set(AppStates::Instructions);
            }
            MainMenuButtonActionComponent::EnterOptions => info!("Enter options menu."),
            MainMenuButtonActionComponent::EnterCompendium => info!("Enter compendium."),
            MainMenuButtonActionComponent::QuitGame => {
                exit.send(AppExit);
            }
        }
    }
}
