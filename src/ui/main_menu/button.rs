//! Provides the layout (trait on `bevy::hierarchy::ChildBUilder`) and behavior (systems) to put 4
//! vertically layed out on the main menu, and change the state from
//! `thetawave_interface::states::AppStates::MainMenu` to
//! `thetawave_interface::states::AppStates::CharacterSelection`
use crate::ui::button::{
    ButtonActionComponent, ButtonActionEvent, ButtonActionType, UiButtonChildBuilderExt,
};
use bevy::{
    asset::Handle,
    ecs::{
        event::EventWriter,
        query::{Changed, With},
        system::{Local, Query},
    },
    hierarchy::{ChildBuilder, Children},
    log::{error, info},
    sprite::TextureAtlas,
    text::Font,
    ui::{widget::Button, Interaction, Style, UiRect, Val},
};
use leafwing_input_manager::prelude::ActionState;
use thetawave_assets::UiAssets;
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    input::{MainMenuExplorer, MenuAction},
};

const BUTTON_TEXTURE_PADDING: UiRect =
    UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(5.0), Val::ZERO);
const BUTTON_TEXTURE_PADDING_HOVERED: UiRect =
    UiRect::new(Val::ZERO, Val::ZERO, Val::Percent(8.5), Val::ZERO);

/// This is the order (vertical, going down) of the buttons shown on the main menu UI.
const MAIN_MENU_BUTTON_ORDER: [ButtonActionType; 4] = [
    ButtonActionType::EnterCharacterSelection,
    ButtonActionType::EnterOptions,
    ButtonActionType::EnterCompendium,
    ButtonActionType::QuitGame,
];

/// Extension trait for spawning customized UI elements for Thetawave
pub(super) trait UiChildBuilderExt {
    // Spawn 1 menu button for each element of `MainMenuButtonActionComponent`
    fn spawn_main_menu_buttons(&mut self, ui_assets: &UiAssets, font: Handle<Font>) -> &mut Self;
}

impl UiChildBuilderExt for ChildBuilder<'_> {
    fn spawn_main_menu_buttons(&mut self, ui_assets: &UiAssets, font: Handle<Font>) -> &mut Self {
        for action in MAIN_MENU_BUTTON_ORDER.iter() {
            self.spawn_button(
                ui_assets,
                font.clone(),
                ButtonActionComponent::from(*action),
                None,
            );
        }

        self
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
    main_menu_buttons: Query<(&ButtonActionComponent, &Children), With<Button>>,
    main_menu_button_mouse_movements: Query<(&ButtonActionComponent, &Interaction), With<Button>>,
    main_menu_button_mouse_changed_movements: Query<
        (&ButtonActionComponent, &Interaction),
        (Changed<Interaction>, With<Button>),
    >,
    menu_explorer_query: Query<&ActionState<MenuAction>, With<MainMenuExplorer>>,
    mut button_texture_query: Query<(&mut TextureAtlas, &mut Style)>,
    // Index into `MAIN_MENU_BUTTON_ORDER`, possibly mod its size
    mut ui_state: Local<MainMenuUIState>,
    // The main side effects of this system/UI component/widget
    mut sound_effect: EventWriter<PlaySoundEffectEvent>,
    mut button_event_writer: EventWriter<ButtonActionEvent>,
) {
    // We do a fair number of linear traversals, but there should only be < 10 buttons, children,
    // etc. So all of those linear time operations should actually be fast.
    // 1. Compute some facts about the current ui state and compute the next frame's ui state
    // 2. Send out sound effect events.
    // 3. Send out any events for "button clicked" actions
    // 4. Set the styling so that only that one button looks "pressed" while all other are inactive
    // 5. Update the `ui_state` for the next frame.
    let currently_hovered_on_button: Option<&ButtonActionType> = main_menu_button_mouse_movements
        .iter()
        .find_map(|(action, x)| match x {
            Interaction::Hovered => Some(&action.action),
            _ => None,
        });
    // Apply d-pad/arrow keys. true = up, false = down
    let contribution_from_arrow_inputs: Option<bool> = match menu_explorer_query.get_single() {
        Err(_) => None,
        Ok(x) => x
            .get_just_pressed()
            .iter()
            .find_map(|action_| match action_ {
                MenuAction::NavigateUpKeyboard | MenuAction::NavigateUpGamepad => Some(true),
                MenuAction::NavigateDownKeyboard | MenuAction::NavigateDownGamepad => Some(false),
                _ => None,
            }),
    };
    let player_confirmed_button_selection: Option<ButtonActionType> = menu_explorer_query
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
    let first_button_mouse_clicked: Option<ButtonActionType> =
        main_menu_button_mouse_changed_movements
            .iter()
            .find_map(|(res, interaction)| {
                if *interaction == Interaction::Pressed {
                    Some(res.action)
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

    let next_frame_button_state: Option<ButtonActionType> = next_frame_ui_state
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
        button_event_writer.send(ButtonActionEvent::from(action));
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
                if next_frame_button_state == Some(action.action) {
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
