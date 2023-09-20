use bevy::app::PluginGroupBuilder;
use bevy::{asset::AssetPlugin, pbr::AmbientLight, prelude::*};
use bevy_kira_audio::prelude::*;

use bevy_rapier2d::geometry::Group;
use bevy_rapier2d::prelude::*;
use thetawave_interface::states::{AppStates, GameStates};

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

/*
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
*/

#[cfg(not(target_arch = "wasm32"))]
fn get_display_config() -> options::DisplayConfig {
    use ron::de::from_str;
    use std::{env::current_dir, fs::read_to_string};

    options::generate_config_files();

    let config_path = current_dir().unwrap().join("config");

    from_str::<options::DisplayConfig>(&read_to_string(config_path.join("display.ron")).unwrap())
        .unwrap()
}

#[cfg(target_arch = "wasm32")]
fn get_display_config() -> options::DisplayConfig {
    use options::DisplayConfig;

    DisplayConfig {
        width: 1280.0,
        height: 1024.0,
        fullscreen: false,
    }
}

fn our_default_plugins(
    display_config: options::DisplayConfig,
    opts: options::GameInitCLIOptions,
) -> PluginGroupBuilder {
    let res = DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window::from(display_config)),
            ..default()
        })
        .set(ImagePlugin::default_nearest());

    match opts.assets_dir {
        Some(path_) => res.set(AssetPlugin {
            asset_folder: path_.to_string_lossy().to_string(),
            ..Default::default()
        }),
        None => res,
    }
}
fn main() {
    // pushes rust errors to the browser console
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let display_config = get_display_config();

    let mut app = build_app(
        our_default_plugins(
            display_config,
            options::GameInitCLIOptions::from_environ_on_supported_platforms_with_default_fallback(
            ),
        ),
        ThetawaveGamePlugins,
    );

    app.run();
}

/// Make the runnable platform-specific app. `base_plugins` describes "external dependencies"
/// outside the scope of the game itself. These typically come from `bevy::MinimalPlugins` or
/// `bevy::DefaultPlugins`. `game_plugins` comes from from `ThetawaveGamePlugins`.
fn build_app<P1: PluginGroup, P2: PluginGroup>(base_plugins: P1, game_plugins: P2) -> App {
    // Should everything besides adding the plugins be moved into a plugin?
    let mut app = App::new();
    app.add_state::<AppStates>() // start game in the main menu state
        .add_state::<GameStates>(); // start the game in playing state
    app.add_plugins(base_plugins);
    app.add_plugins(game_plugins);
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        });

    app.add_systems(
        OnEnter(AppStates::Game),
        setup_physics.in_set(GameEnterSet::Initialize),
    );
    if cfg!(debug_assertions) && !cfg!(test) {
        app.add_plugins(RapierDebugRenderPlugin::default());
    }
    app
}

// setup rapier
fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.physics_pipeline_active = true;
    rapier_config.query_pipeline_active = true;
    rapier_config.gravity = Vec2::ZERO;
}
/// This is the main collection of features and behaviors that define thetawave. 99% of the total
/// behavior of all executables comes from this and Bevy plugins. Ideally 100% of the functionality
/// of all Thetawave executables comes from this and Bevy plugins.
pub struct ThetawaveGamePlugins;
impl PluginGroup for ThetawaveGamePlugins {
    fn build(self) -> PluginGroupBuilder {
        #[allow(unused_mut)] // Allow because we might add more platform-specific features
        let mut res = PluginGroupBuilder::start::<Self>()
            .add(player::PlayerPlugin)
            .add(spawnable::SpawnablePlugin)
            .add(run::RunPlugin)
            .add(loot::LootPlugin)
            .add(game::GamePlugin)
            .add(background::BackgroundPlugin)
            .add(AudioPlugin)
            .add(camera::CameraPlugin)
            .add(arena::ArenaPlugin)
            .add(collision::CollisionPlugin)
            .add(scanner::ScannerPlugin)
            .add(animation::AnimationPlugin)
            .add(states::StatesPlugin)
            .add(game::counters::plugin::CountingMetricsPlugin)
            .add(misc::HealthPlugin)
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
                PHYSICS_SCALE,
            ))
            .add(ui::UiPlugin)
            .add(options::OptionsPlugin)
            .add(audio::ThetawaveAudioPlugin);
        #[cfg(feature = "arcade")]
        {
            res = res.add(thetawave_arcade::arduino::ArcadeArduinoPlugin);
        }
        #[cfg(feature = "storage")]
        {
            res = res.add(thetawave_storage::plugin::DBPlugin);
        }

        res
    }
}

#[cfg(test)]
mod test {
    use crate::audio::ThetawaveAudioPlugin;
    use crate::{build_app, options, ui, ThetawaveGamePlugins};
    use bevy::app::{App, PluginGroup};
    use bevy::asset::AssetPlugin;
    use bevy::input::InputPlugin;
    use bevy::prelude::{ImagePlugin, NextState, State};
    use bevy::MinimalPlugins;
    use bevy_kira_audio::AudioPlugin;
    use thetawave_interface::audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent};
    use thetawave_interface::states::AppStates;

    #[test]
    fn test_minimal_headless_audioless_game_gets_to_main_menu() {
        // This atleast tests that many resourses/events required by systems have already been
        // inserted, that many assets can be loaded, and that the game can kinda start.
        let mut app = minimal_headless_audioless_app_with_as_many_game_features_as_possible();
        app.update();
        app.world
            .get_resource_mut::<NextState<AppStates>>()
            .unwrap()
            .set(AppStates::LoadingAssets);
        for _ in 0..10 {
            // just update a few times. Not so important how many.
            app.update();
        }
        app.world
            .get_resource_mut::<NextState<AppStates>>()
            .unwrap()
            .set(AppStates::MainMenu);
        app.update();
        app.update();
        assert_eq!(
            app.world.get_resource::<State<AppStates>>().unwrap().get(),
            &AppStates::MainMenu
        );
    }

    fn minimal_headless_audioless_app_with_as_many_game_features_as_possible() -> App {
        let base_plugins = MinimalPlugins
            .build()
            .add(AssetPlugin::default())
            .add(ImagePlugin::default())
            .add(InputPlugin::default());
        // These features are basically untestable.
        let game_plugins = ThetawaveGamePlugins
            .build()
            // Windowing/display/UI stuff is hard to test and we dont have a screen in CI.
            .disable::<ui::UiPlugin>()
            .disable::<options::OptionsPlugin>()
            // Ideally audio is mostly handled via `thetawave_interface::audio` and events, so that
            // we really only skip testing 1 match statement and external audio deps.
            .disable::<ThetawaveAudioPlugin>()
            .disable::<AudioPlugin>();

        let mut app = build_app(base_plugins, game_plugins);
        app.add_event::<ChangeBackgroundMusicEvent>()
            .add_event::<PlaySoundEffectEvent>();
        app
    }
}
