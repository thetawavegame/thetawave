use bevy::prelude::*;

mod game;
mod pause_menu;

pub use self::game::*;
pub use self::pause_menu::*;

// states of the game
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppStates {
    MainMenu,
    PauseMenu,
    LoadingGame,
    Game,
    GameOver,
    Victory,
}

// used for tagging entities that are part of the game state
#[derive(Component)]
pub struct AppStateComponent(pub AppStates);

// remove entities tagged for the current app state
pub fn clear_state_system(
    mut commands: Commands,
    mut despawn_entities_query: Query<(Entity, &AppStateComponent)>,
    app_state: Res<State<AppStates>>,
) {
    for (entity, entity_app_state) in despawn_entities_query.iter_mut() {
        if *app_state.current() == entity_app_state.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
