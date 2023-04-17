use std::default;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
            (open_pause_menu_system,)
                .in_set(OnUpdate(AppStates::Game))
                .in_set(OnUpdate(GameStates::Playing)),
        );

        app.add_systems(
            (start_instructions_system, quit_game_system).in_set(OnUpdate(AppStates::MainMenu)), //.distributive_run_if(in_state(AppStates::MainMenu))
                                                                                                 //.in_base_set(CoreSet::PreUpdate),
        );

        app.add_systems(
            (start_character_selection_system,).in_set(OnUpdate(AppStates::Instructions)),
        );

        app.add_systems((start_game_system,).in_set(OnUpdate(AppStates::CharacterSelection)));

        app.add_systems(
            (clear_state_system::<MainMenuCleanup>,).in_schedule(OnExit(AppStates::MainMenu)),
        );

        app.add_systems((clear_state_system::<GameCleanup>,).in_schedule(OnExit(AppStates::Game)));

        app.add_systems((quit_game_system,).in_set(OnUpdate(AppStates::GameOver)));

        app.add_systems(
            (clear_state_system::<GameOverCleanup>,).in_schedule(OnExit(AppStates::GameOver)),
        );

        app.add_systems((quit_game_system,).in_set(OnUpdate(AppStates::Victory)));

        app.add_systems(
            (clear_state_system::<VictoryCleanup>,).in_schedule(OnExit(AppStates::Victory)),
        );

        app.add_systems(
            (clear_state_system::<CharacterSelectionCleanup>,)
                .in_schedule(OnExit(AppStates::CharacterSelection)),
        );

        app.add_systems(
            (clear_state_system::<PauseCleanup>,).in_schedule(OnExit(GameStates::Paused)),
        );

        app.add_systems(
            (clear_state_system::<InstructionsCleanup>,)
                .in_schedule(OnExit(AppStates::Instructions)),
        );

        app.add_systems((close_pause_menu_system,).in_set(OnUpdate(GameStates::Paused)));
    }
}

// states of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum AppStates {
    #[default]
    LoadingAssets,
    MainMenu,
    Instructions,
    CharacterSelection,
    //LoadingGame, // assets can currently only be loaded once
    Game,
    GameOver,
    Victory,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum GameStates {
    #[default]
    Playing,
    Paused,
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
