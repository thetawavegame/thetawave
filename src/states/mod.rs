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
            LoadingState::new(AppStates::LoadingGame).continue_to_state(AppStates::Game),
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "player_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "projectile_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "mob_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "consumable_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "effect_assets.assets.ron",
        )
        .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
            AppStates::LoadingGame,
            "game_audio_assets.assets.ron",
        )
        .add_collection_to_loading_state::<_, assets::PlayerAssets>(AppStates::LoadingGame)
        .add_collection_to_loading_state::<_, assets::ProjectileAssets>(AppStates::LoadingGame)
        .add_collection_to_loading_state::<_, assets::MobAssets>(AppStates::LoadingGame)
        .add_collection_to_loading_state::<_, assets::ConsumableAssets>(AppStates::LoadingGame)
        .add_collection_to_loading_state::<_, assets::EffectAssets>(AppStates::LoadingGame)
        .add_collection_to_loading_state::<_, assets::GameAudioAssets>(AppStates::LoadingGame);

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

        app.add_systems((open_pause_menu_system,).in_set(OnUpdate(AppStates::Game)));

        app.add_systems(
            (start_game_system, quit_game_system).in_set(OnUpdate(AppStates::MainMenu)), //.distributive_run_if(in_state(AppStates::MainMenu))
                                                                                         //.in_base_set(CoreSet::PreUpdate),
        );

        app.add_systems(
            (clear_state_system::<MainMenuCleanup>,)
                //.after(OnUpdate(AppStates::MainMenu))
                .in_schedule(OnExit(AppStates::MainMenu)),
        );
    }
}

// states of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum AppStates {
    #[default]
    MainMenu,
    PauseMenu,
    LoadingGame,
    Game,
    GameOver,
    Victory,
}

#[derive(Component)]
pub struct MainMenuCleanup;

#[derive(Component)]
pub struct GameCleanup;

// remove entities tagged for the current app state
pub fn clear_state_system<T: Component>(
    mut commands: Commands,
    mut despawn_entities_query: Query<Entity, With<T>>,
    //app_state: Res<State<AppStates>>,
) {
    //println!("clearing state: {:?}", app_state.0);
    /*
    for (entity, entity_app_state) in despawn_entities_query.iter_mut() {
        if app_state.0 == entity_app_state.0 {
            commands.entity(entity).despawn_recursive();
        }
    }*/

    for entity in despawn_entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
