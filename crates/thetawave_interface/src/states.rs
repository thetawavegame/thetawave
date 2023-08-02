use bevy_ecs_macros::States;

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
