//! Minimal components for indicating when entities should be despawned from the game

use bevy_ecs::component::Component;

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
