use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
            "effect_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingAssets,
            "game_audio_assets.assets.ron",
        )
        .add_collection_to_loading_state::<_, assets::PlayerAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::ProjectileAssets>(AppStates::LoadingAssets)
        .add_collection_to_loading_state::<_, assets::MobAssets>(AppStates::LoadingAssets)
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
            start_stats_system.run_if(in_state(AppStates::MainMenu)),
        );

        app.add_systems(
            Update,
            start_mainmenu_system.run_if(in_state(AppStates::Stats)),
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
        app.add_systems(OnExit(AppStates::Stats), clear_state_system::<StatsCleanup>);

        app.add_systems(
            Update,
            close_pause_menu_system.run_if(in_state(GameStates::Paused)),
        );
    }
}

#[derive(Component)]
pub struct MainMenuCleanup;

#[derive(Component)]
pub struct GameCleanup;

#[derive(Component)]
pub struct GameOverCleanup;

#[derive(Component)]
pub struct VictoryCleanup;

#[derive(Component)]
pub struct PauseCleanup;

#[derive(Component)]
pub struct InstructionsCleanup;

#[derive(Component)]
pub struct StatsCleanup;

#[derive(Component)]
pub struct CharacterSelectionCleanup;

// remove entities tagged for the current app state
pub fn clear_state_system<T: Component>(
    mut commands: Commands,
    despawn_entities_query: Query<Entity, With<T>>,
) {
    for entity in despawn_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn unload_game_assets(mut commands: Commands) {
    commands.remove_resource::<assets::PlayerAssets>();
    commands.remove_resource::<assets::ProjectileAssets>();
    commands.remove_resource::<assets::MobAssets>();
    commands.remove_resource::<assets::ConsumableAssets>();
    commands.remove_resource::<assets::EffectAssets>();
    commands.remove_resource::<assets::GameAudioAssets>();
}
