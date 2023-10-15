use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use thetawave_interface::input::MenuAction;
use thetawave_interface::input::MenuExplorer;
use thetawave_interface::states::CharacterSelectionCleanup;
use thetawave_interface::states::GameCleanup;
use thetawave_interface::states::GameOverCleanup;
use thetawave_interface::states::InstructionsCleanup;
use thetawave_interface::states::MainMenuCleanup;
use thetawave_interface::states::PauseCleanup;
use thetawave_interface::states::VictoryCleanup;
pub use thetawave_interface::states::{AppStates, GameStates};

mod game;
mod pause_menu;

use crate::assets;
use crate::GameEnterSet;
use crate::GameUpdateSet;

pub use self::game::*;
pub use self::pause_menu::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppStates::LoadingAssets).continue_to_state(AppStates::MainMenu),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "player_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "projectile_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "mob_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "consumable_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "item_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "effect_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "game_audio_assets.assets.ron",
        )
        .add_collection_to_loading_state::<_, assets::PlayerAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::ProjectileAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::MobAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::ItemAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::ConsumableAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::EffectAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::GameAudioAssets>(AppStates::LoadingAssets);

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

        app.add_systems(
            Update,
            start_instructions_system.run_if(in_state(AppStates::MainMenu)),
        );

        app.add_systems(
            Update,
            start_character_selection_system.run_if(in_state(AppStates::Instructions)),
        );

        app.add_systems(
            Update,
            start_game_system.run_if(in_state(AppStates::CharacterSelection)),
        );

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
            OnExit(AppStates::Instructions),
            clear_state_system::<InstructionsCleanup>,
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
pub fn clear_state_system<T: Component>(
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
    if action_state.just_released(MenuAction::Reset) {
        // go to the main menu state
        next_app_state.set(AppStates::MainMenu);
        next_game_state.set(GameStates::Playing);
    }
}
