use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    player::PlayersResource,
    states::AppStates,
};

// Start the game by entering the Game state
pub fn start_game_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    players_resource: Res<PlayersResource>,
    mut sound_effect_pub: EventWriter<PlaySoundEffectEvent>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(MenuAction::Confirm) && players_resource.player_data[0].is_some()
    {
        // set the state to game
        next_app_state.set(AppStates::InitializeRun);

        // play sound effect
        sound_effect_pub.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::MenuInputSuccess,
        });
    }
}

// Start the game by entering the Game state
pub fn start_instructions_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut sound_effect_pub: EventWriter<PlaySoundEffectEvent>,
) {
    // read menu input action
    if let Ok(action_state) = menu_input_query.get_single() {
        // if input read enter the game state
        if action_state.just_released(MenuAction::Confirm) {
            // set the state to game
            next_app_state.set(AppStates::Instructions);

            // play sound effect
            sound_effect_pub.send(PlaySoundEffectEvent {
                sound_effect_type: SoundEffectType::MenuInputSuccess,
            });
        }
    }
}

pub fn start_character_selection_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut sound_effect_pub: EventWriter<PlaySoundEffectEvent>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(MenuAction::Confirm) {
        // set the state to game
        next_app_state.set(AppStates::CharacterSelection);

        // play sound effect
        sound_effect_pub.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::MenuInputSuccess,
        });
    }
}
