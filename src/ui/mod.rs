use bevy::prelude::*;
pub use thetawave_interface::character_selection::PlayerJoinEvent;

use crate::{states, GameEnterSet, GameUpdateSet};

mod character_selection;
mod debug;
mod game;
mod game_over;
mod instructions;
mod main_menu;
mod pause_menu;
mod victory;

pub use self::character_selection::{
    player_join_system, select_character_system, setup_character_selection_system,
};
use self::instructions::setup_instructions_system;
pub use self::{
    game_over::{
        fade_out_system, game_over_fade_in_system, setup_game_over_system,
        EndGameTransitionResource, GameFadeComponent,
    },
    main_menu::{bouncing_prompt_system, setup_main_menu_system, BouncingPromptComponent},
    pause_menu::setup_pause_system,
    victory::{setup_victory_system, victory_fade_in_system},
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerJoinEvent>();

        app.insert_resource(EndGameTransitionResource::new(
            2.0, 3.0, 2.5, 0.5, 0.5, 30.0,
        ));

        app.add_systems(Update, bouncing_prompt_system);

        app.add_systems(
            OnEnter(states::AppStates::Game),
            game::setup_game_ui_system.after(GameEnterSet::BuildUi),
        );

        app.add_systems(
            Update,
            (
                game::update_player1_ui.after(GameUpdateSet::UpdateUi),
                game::update_player2_ui.after(GameUpdateSet::UpdateUi),
                fade_out_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );

        app.add_systems(OnEnter(states::AppStates::MainMenu), setup_main_menu_system);

        app.add_systems(
            OnEnter(states::AppStates::Instructions),
            setup_instructions_system,
        );

        app.add_systems(
            OnEnter(states::AppStates::CharacterSelection),
            setup_character_selection_system,
        );

        app.add_systems(
            Update,
            (player_join_system, select_character_system)
                .run_if(in_state(states::AppStates::CharacterSelection)),
        );

        app.add_systems(OnEnter(states::AppStates::GameOver), setup_game_over_system);

        app.add_systems(
            Update,
            game_over_fade_in_system.run_if(in_state(states::AppStates::GameOver)),
        );

        app.add_systems(OnEnter(states::AppStates::Victory), setup_victory_system);

        app.add_systems(
            Update,
            victory_fade_in_system.run_if(in_state(states::AppStates::Victory)),
        );

        app.add_systems(OnEnter(states::GameStates::Paused), setup_pause_system);
    }
}
