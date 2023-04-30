use bevy::{pbr::AmbientLight, prelude::*};
use bevy_editor_pls::prelude::*;
use bevy_kira_audio::prelude::*;

use bevy_rapier2d::geometry::Group;
use bevy_rapier2d::prelude::*;
use states::{AppStates, GameCleanup, GameStates};
use ui::EndGameTransitionResource;

pub const PHYSICS_SCALE: f32 = 10.0;
pub const SPAWNABLE_COL_GROUP_MEMBERSHIP: Group = Group::GROUP_1;
pub const HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP: Group = Group::GROUP_2;
pub const VERTICAL_BARRIER_COL_GROUP_MEMBERSHIP: Group = Group::GROUP_3;

mod animation;
mod arena;
mod assets;
mod audio;
mod background;
mod camera;
mod collision;
mod game;
mod loot;
mod misc;
mod options;
mod player;
mod run;
mod scanner;
mod spawnable;
mod states;
mod tools;
mod ui;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameEnterSet {
    Initialize,
    BuildLevel,
    SpawnPlayer,
    BuildUi,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameUpdateSet {
    Enter,
    Level,
    Spawn,
    NextLevel,
    UpdateUi,
    Movement,
    Abilities,
    SetTargetBehavior, // TODO: replace with more general set
    ExecuteBehavior,
    ContactCollision,
    IntersectionCollision,
    ApplyDisconnectedBehaviors,
    ChangeState,
    Cleanup,
}

// Don't generate a display config for wasm
#[cfg(target_arch = "wasm32")]
fn get_display_config() -> options::DisplayConfig {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook)); // pushes rust errors to the browser console
    options::DisplayConfig {
        width: 1280.0,
        height: 720.0,
        fullscreen: false,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn get_display_config() -> options::DisplayConfig {
    use ron::de::from_str;
    use std::{env::current_dir, fs::read_to_string};

    options::generate_config_files();

    let config_path = current_dir().unwrap().join("config");

    from_str::<options::DisplayConfig>(&read_to_string(config_path.join("display.ron")).unwrap())
        .unwrap()
}

fn main() {
    let display_config = get_display_config();

    let mut app = App::new();

    app.add_state::<AppStates>(); // start game in the main menu state
    app.add_state::<GameStates>(); // start the game in playing state

    // insert resources for all game states
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window::from(display_config)),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugin(player::PlayerPlugin)
    .add_plugin(spawnable::SpawnablePlugin)
    .add_plugin(run::RunPlugin)
    .add_plugin(loot::LootPlugin)
    .add_plugin(game::GamePlugin)
    .add_plugin(background::BackgroundPlugin)
    .add_plugin(audio::ThetawaveAudioPlugin)
    .add_plugin(options::OptionsPlugin)
    .add_plugin(camera::CameraPlugin)
    .add_plugin(ui::UiPlugin)
    .add_plugin(arena::ArenaPlugin)
    .add_plugin(collision::CollisionPlugin)
    .add_plugin(scanner::ScannerPlugin)
    .add_plugin(animation::AnimationPlugin)
    .add_plugin(states::StatesPlugin)
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    })
    .add_plugin(AudioPlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PHYSICS_SCALE,
    ));

    app.add_systems(
        (
            setup_game.in_set(GameEnterSet::Initialize),
            setup_physics.in_set(GameEnterSet::Initialize),
        )
            .in_schedule(OnEnter(states::AppStates::Game)),
    );

    if cfg!(debug_assertions) {
        app.add_plugin(EditorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}

// setup rapier
fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
    rapier_config.gravity = Vec2::ZERO;
}

/// Initialize values for the game
#[allow(clippy::too_many_arguments)]
fn setup_game(
    mut commands: Commands,
    mut run_resource: ResMut<run::RunResource>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    levels_resource: Res<run::LevelsResource>,
) {
    *end_game_trans_resource = EndGameTransitionResource::new(2.0, 3.0, 2.5, 0.5, 0.5, 30.0);

    // spawn game fade entity
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(16000.0, 9000.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 100.0),
            ..default()
        })
        .insert(ui::GameFadeComponent)
        .insert(GameCleanup)
        .insert(Name::new("Game Fade"));

    // create run resource
    run_resource.create_level(&levels_resource);
}
