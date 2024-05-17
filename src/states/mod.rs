//! Exposes a plugin that manages state transitions and the core behavior that deals with
//! `thetawave_interface::states::AppStates`.
use bevy::prelude::{
    in_state, App, Commands, Component, DespawnRecursiveExt, Entity, IntoSystemConfigs,
    IntoSystemSetConfigs, NextState, OnEnter, OnExit, Plugin, Query, ResMut, Update, With,
};
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::standard_dynamic_asset::StandardDynamicAssetCollection;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::MenuAction;
use thetawave_interface::input::MenuExplorer;
use thetawave_interface::states::CharacterSelectionCleanup;
use thetawave_interface::states::GameCleanup;
use thetawave_interface::states::GameOverCleanup;
use thetawave_interface::states::MainMenuCleanup;
use thetawave_interface::states::PauseCleanup;
use thetawave_interface::states::VictoryCleanup;
use thetawave_interface::states::{AppStates, GameStates};

mod game;
mod pause_menu;

use crate::assets::ConsumableAssets;
use crate::assets::EffectAssets;
use crate::assets::GameAudioAssets;
use crate::assets::ItemAssets;
use crate::assets::MobAssets;
use crate::assets::PlayerAssets;
use crate::assets::ProjectileAssets;
use crate::assets::UiAssets;
use crate::GameEnterSet;
use crate::GameUpdateSet;

use self::game::start_game_system;
use self::pause_menu::{close_pause_menu_system, open_pause_menu_system};
/// Includes systems that handle state transitions for `AppStates` and `GameStates`. Also includes
/// an asset loading state.
pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppStates::LoadingAssets)
                .continue_to_state(AppStates::MainMenu)
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "player_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "projectile_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("mob_assets.assets.ron")
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "consumable_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "item_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "effect_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>(
                    "game_audio_assets.assets.ron",
                )
                .with_dynamic_assets_file::<StandardDynamicAssetCollection>("ui_assets.assets.ron")
                .load_collection::<PlayerAssets>()
                .load_collection::<ProjectileAssets>()
                .load_collection::<MobAssets>()
                .load_collection::<ItemAssets>()
                .load_collection::<ConsumableAssets>()
                .load_collection::<EffectAssets>()
                .load_collection::<GameAudioAssets>()
                .load_collection::<UiAssets>(),
        );

        app.edit_schedule(OnEnter(AppStates::Game), |schedule| {
            schedule.configure_sets(
                (
                    GameEnterSet::Initialize,
                    GameEnterSet::BuildLevel,
                    GameEnterSet::SpawnPlayer,
                    GameEnterSet::BuildUi,
                )
                    .chain(),
            );
        });

        app.configure_sets(
            Update,
            (
                //GameUpdateSet::Enter,
                GameUpdateSet::Level,
                GameUpdateSet::Spawn,
                GameUpdateSet::NextLevel,
                GameUpdateSet::UpdateUi,
                GameUpdateSet::SetTargetBehavior,
                GameUpdateSet::ContactCollision,
                GameUpdateSet::IntersectionCollision,
                GameUpdateSet::ExecuteBehavior,
                GameUpdateSet::ApplyDisconnectedBehaviors,
                GameUpdateSet::Movement,
                GameUpdateSet::Abilities,
                GameUpdateSet::ChangeState,
                GameUpdateSet::Cleanup,
            )
                .chain(),
        );

        app.add_systems(
            Update,
            open_pause_menu_system
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );

        /*
        app.add_systems(
            Update,
            start_game_system.run_if(in_state(AppStates::CharacterSelection)),
        );
        */

        app.add_systems(
            OnExit(AppStates::MainMenu),
            clear_state_system::<MainMenuCleanup>,
        );

        app.add_systems(OnExit(AppStates::Game), clear_state_system::<GameCleanup>);

        app.add_systems(
            OnExit(AppStates::GameOver),
            clear_state_system::<GameOverCleanup>,
        );

        app.add_systems(
            OnExit(AppStates::Victory),
            clear_state_system::<VictoryCleanup>,
        );

        app.add_systems(
            OnExit(AppStates::CharacterSelection),
            clear_state_system::<CharacterSelectionCleanup>,
        );

        app.add_systems(
            OnExit(GameStates::Paused),
            clear_state_system::<PauseCleanup>,
        );

        app.add_systems(
            Update,
            close_pause_menu_system.run_if(in_state(GameStates::Paused)),
        );

        app.add_systems(
            Update,
            start_mainmenu_system.run_if(in_state(AppStates::Victory)),
        );

        app.add_systems(
            Update,
            start_mainmenu_system.run_if(in_state(AppStates::GameOver)),
        );

        app.add_systems(
            Update,
            start_mainmenu_system
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Paused)),
        );
    }
}

// remove entities tagged for the current app state
fn clear_state_system<T: Component>(
    mut commands: Commands,
    despawn_entities_query: Query<Entity, With<T>>,
) {
    for entity in despawn_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn start_mainmenu_system(
    menu_input_query: Query<&ActionState<MenuAction>, With<MenuExplorer>>,
    mut next_app_state: ResMut<NextState<AppStates>>,
    mut next_game_state: ResMut<NextState<GameStates>>,
) {
    // read menu input action
    let action_state = menu_input_query.single();

    // if reset input provided reset th run
    if action_state.just_released(&MenuAction::Reset) {
        // go to the main menu state
        next_app_state.set(AppStates::MainMenu);
        next_game_state.set(GameStates::Playing);
    }
}
