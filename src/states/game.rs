use bevy::ecs::event::EventWriter;
use bevy::ecs::query::With;
use bevy::ecs::schedule::NextState;
use bevy::ecs::system::{Query, Res, ResMut};
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::{MenuAction, MenuExplorer};
use thetawave_interface::{
    audio::{PlaySoundEffectEvent, SoundEffectType},
    player::PlayersResource,
    states::AppStates,
};

// Start the game by entering the Game state
pub(super) fn start_game_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    players_resource: Res<PlayersResource>,
    mut sound_effect_pub: EventWriter<PlaySoundEffectEvent>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if input read enter the game state
    if action_state.just_released(&MenuAction::Confirm) && players_resource.player_data[0].is_some()
    {
        // set the state to game
        next_app_state.set(AppStates::InitializeRun);

        // play sound effect
        sound_effect_pub.send(PlaySoundEffectEvent {
            sound_effect_type: SoundEffectType::MenuInputSuccess,
        });
    }
}
