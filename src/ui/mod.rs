//! Exposes a plugin that handles layout, rendering, and styling for each of the major game states.
use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, OnEnter},
    prelude::{Component, IntoSystemConfigs, Query, Res, Time, Timer, Transform},
};
use thetawave_interface::character_selection::PlayerJoinEvent;
use thetawave_interface::game::historical_metrics::{MobsKilledByPlayerCacheT, DEFAULT_USER_ID};

use thetawave_interface::states;

mod button;
mod character_selection;
mod game;
mod game_over;
mod main_menu;
mod pause_menu;
mod victory;

use self::{
    button::ButtonActionEvent,
    character_selection::{
        player_join_system, select_character_system, setup_character_selection_system,
        CharacterSelectionPlugin,
    },
    game::GameUiPlugin,
    game_over::setup_game_over_system,
    main_menu::MainMenuUIPlugin,
    pause_menu::setup_pause_system,
    victory::setup_victory_system,
};

/// Handles layout, styling, and updating the UI state on each frame update. Without this plugin,
/// we mostly just have a black screen with some images moving across the screen.
pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonActionEvent>();
        app.add_plugins(GameUiPlugin);
        app.add_plugins(MainMenuUIPlugin);
        app.add_plugins(CharacterSelectionPlugin);
        app.add_systems(Update, bouncing_prompt_system);

        app.add_systems(OnEnter(states::AppStates::GameOver), setup_game_over_system);

        app.add_systems(OnEnter(states::AppStates::Victory), setup_victory_system);

        app.add_systems(OnEnter(states::GameStates::Paused), setup_pause_system);
    }
}

// Consistently format mob+kill-count pairs.
fn pprint_mob_kills_from_data(data: &MobsKilledByPlayerCacheT) -> String {
    match (*data).get(&DEFAULT_USER_ID) {
        None => String::from("No mobs killed"),
        Some(mob_kill_counts) => mob_kill_counts
            .iter()
            .map(|(mobtype, n)| format!("{mobtype}: {n}"))
            .collect::<Vec<String>>()
            .join("\n"),
    }
}

/// A component that will cause the assocaited entity to bounce up and down at a rate determined by
/// `Self::flash_timer.duration` while `Self::is_active`.
#[derive(Component)]
pub(super) struct BouncingPromptComponent {
    pub flash_timer: Timer,
    /// Set this to `false` to pause the animation
    pub is_active: bool,
}
/// Manipulate the `Transform` to make it look like the component is bouncing. This needs to be run
/// as frequently as possible for the animation to be smooth.
fn bouncing_prompt_system(
    mut flashing_prompt_query: Query<(&mut Transform, &mut BouncingPromptComponent)>,
    time: Res<Time>,
) {
    for (mut transform, mut prompt) in flashing_prompt_query.iter_mut() {
        if !prompt.is_active {
            transform.scale.x = 1.0;
            transform.scale.y = 1.0;
            prompt.flash_timer.reset();
            continue;
        }
        prompt.flash_timer.tick(time.delta());

        let scale: f32 = -0.2 * (prompt.flash_timer.elapsed_secs() - 1.0).powf(2.0) + 1.2;

        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
