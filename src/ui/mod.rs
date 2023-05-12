use bevy::prelude::*;

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
    player_join_system, select_character_system, setup_character_selection_system, PlayerJoinEvent,
};
use self::instructions::setup_instructions_system;
pub use self::{
    debug::game_debug_ui,
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

        app.add_systems((bouncing_prompt_system,));

        app.add_systems(
            (game::setup_game_ui_system.after(GameEnterSet::BuildUi),)
                .in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (
                game::update_player1_ui.after(GameUpdateSet::UpdateUi),
                game::update_player2_ui.after(GameUpdateSet::UpdateUi),
                fade_out_system,
            )
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );

        app.add_systems(
            (setup_main_menu_system,).in_schedule(OnEnter(states::AppStates::MainMenu)),
        );

        app.add_systems(
            (setup_instructions_system,).in_schedule(OnEnter(states::AppStates::Instructions)),
        );

        app.add_systems(
            (setup_character_selection_system,)
                .in_schedule(OnEnter(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (player_join_system, select_character_system)
                .in_set(OnUpdate(states::AppStates::CharacterSelection)),
        );

        app.add_systems(
            (setup_game_over_system,).in_schedule(OnEnter(states::AppStates::GameOver)),
        );

        app.add_systems((game_over_fade_in_system,).in_set(OnUpdate(states::AppStates::GameOver)));

        app.add_systems((setup_victory_system,).in_schedule(OnEnter(states::AppStates::Victory)));

        app.add_systems((victory_fade_in_system,).in_set(OnUpdate(states::AppStates::Victory)));

        app.add_systems((setup_pause_system,).in_schedule(OnEnter(states::GameStates::Paused)));
    }
}
