use bevy_ecs_macros::{Component, States};

// states of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum AppStates {
    #[default]
    LoadingAssets,
    MainMenu,
    CharacterSelection,
    InitializeRun,
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
pub struct CharacterSelectionCleanup;
