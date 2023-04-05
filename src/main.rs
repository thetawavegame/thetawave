use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::ecs::schedule;
use bevy::{pbr::AmbientLight, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use leafwing_input_manager::prelude::*;

use bevy_rapier2d::geometry::Group;
use bevy_rapier2d::prelude::*;
use ron::de::from_bytes;
use states::{AppStateComponent, AppStates};
use std::collections::HashMap;
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
mod input;
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
    UpdateUI,
    Movement,
    Abilities,
    SetTargetBehavior, // TODO: replace with more general set
    ExecuteBehavior,
    ContactCollision,
    IntersectionCollision,
    ApplyDisconnectedBehaviors,
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
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(
        from_bytes::<spawnable::BehaviorSequenceResource>(include_bytes!(
            "../assets/data/behavior_sequences.ron"
        ))
        .unwrap(),
    )
    .insert_resource(
        from_bytes::<loot::LootDropsResource>(include_bytes!("../assets/data/loot_drops.ron"))
            .unwrap(),
    )
    .insert_resource(
        from_bytes::<run::FormationPoolsResource>(include_bytes!(
            "../assets/data/formation_pools.ron"
        ))
        .unwrap(),
    )
    .insert_resource(
        from_bytes::<game::GameParametersResource>(include_bytes!(
            "../assets/data/game_parameters.ron"
        ))
        .unwrap(),
    )
    .insert_resource(run::RunResource::from(
        from_bytes::<run::RunResourceData>(include_bytes!("../assets/data/run.ron")).unwrap(),
    ))
    .insert_resource(run::LevelsResource::from(
        from_bytes::<run::LevelsResourceData>(include_bytes!("../assets/data/levels.ron")).unwrap(),
    ))
    .insert_resource(spawnable::MobsResource {
        mobs: from_bytes::<HashMap<spawnable::MobType, spawnable::MobData>>(include_bytes!(
            "../assets/data/mobs.ron"
        ))
        .unwrap(),
        texture_atlas_handle: HashMap::new(),
    })
    .insert_resource(
        from_bytes::<spawnable::MobSegmentsResource>(include_bytes!(
            "../assets/data/mob_segments.ron"
        ))
        .unwrap(),
    )
    .insert_resource(spawnable::EffectsResource {
        effects: from_bytes::<HashMap<spawnable::EffectType, spawnable::EffectData>>(
            include_bytes!("../assets/data/effects.ron"),
        )
        .unwrap(),
    })
    .insert_resource(spawnable::ProjectileResource {
        projectiles: from_bytes::<HashMap<spawnable::ProjectileType, spawnable::ProjectileData>>(
            include_bytes!("../assets/data/projectiles.ron"),
        )
        .unwrap(),
    })
    .insert_resource(spawnable::ConsumableResource {
        consumables: from_bytes::<HashMap<spawnable::ConsumableType, spawnable::ConsumableData>>(
            include_bytes!("../assets/data/consumables.ron"),
        )
        .unwrap(),
    })
    .insert_resource(
        from_bytes::<background::BackgroundsResource>(include_bytes!(
            "../assets/data/backgrounds.ron"
        ))
        .unwrap(),
    )
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    })
    .insert_resource(ui::EndGameTransitionResource::new(
        2.0, 3.0, 2.5, 0.5, 0.5, 30.0,
    ))
    .add_event::<collision::SortedCollisionEvent>()
    .add_event::<run::SpawnFormationEvent>()
    .add_event::<run::LevelCompletedEvent>()
    .add_event::<arena::MobReachedBottomGateEvent>()
    .add_event::<spawnable::SpawnEffectEvent>()
    .add_event::<spawnable::SpawnConsumableEvent>()
    .add_event::<spawnable::SpawnProjectileEvent>()
    .add_event::<spawnable::SpawnMobEvent>()
    .add_event::<spawnable::MobBehaviorUpdateEvent>()
    .add_event::<spawnable::MobDestroyedEvent>()
    .add_event::<spawnable::MobSegmentDestroyedEvent>()
    .add_plugin(AudioPlugin)
    .add_plugin(EguiPlugin)
    .add_plugin(InputManagerPlugin::<input::InputAction>::default())
    .add_audio_channel::<audio::BackgroundMusicAudioChannel>()
    .add_audio_channel::<audio::MenuAudioChannel>()
    .add_audio_channel::<audio::SoundEffectsAudioChannel>()
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
        PHYSICS_SCALE,
    ))
    .add_startup_system(camera::setup_cameras_system)
    .add_startup_system(audio::set_audio_volume_system)
    .add_system(ui::bouncing_prompt_system)
    .add_system(options::toggle_fullscreen_system)
    .add_system(ui::position_stat_bar_label_system.in_base_set(CoreSet::Last));

    #[cfg(not(target_arch = "wasm32"))]
    {
        //app.add_plugin(bevy_framepace::FramepacePlugin);
    }

    // add states
    app.add_loading_state(
        LoadingState::new(states::AppStates::LoadingGame)
            .continue_to_state(states::AppStates::Game),
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "player_assets.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "projectile_assets.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "mob_assets.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "consumable_assets.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "effect_assets.assets.ron",
    )
    .add_dynamic_collection_to_loading_state::<_, StandardDynamicAssetCollection>(
        states::AppStates::LoadingGame,
        "game_audio_assets.assets.ron",
    )
    .add_collection_to_loading_state::<_, assets::PlayerAssets>(states::AppStates::LoadingGame)
    .add_collection_to_loading_state::<_, assets::ProjectileAssets>(states::AppStates::LoadingGame)
    .add_collection_to_loading_state::<_, assets::MobAssets>(states::AppStates::LoadingGame)
    .add_collection_to_loading_state::<_, assets::ConsumableAssets>(states::AppStates::LoadingGame)
    .add_collection_to_loading_state::<_, assets::EffectAssets>(states::AppStates::LoadingGame)
    .add_collection_to_loading_state::<_, assets::GameAudioAssets>(states::AppStates::LoadingGame);

    app.edit_schedule(OnEnter(states::AppStates::Game), |schedule| {
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

    // game startup systems (perhaps exchange with app.add_startup_system_set)
    app.add_systems(
        (
            setup_game.in_set(GameEnterSet::Initialize),
            setup_physics.in_set(GameEnterSet::Initialize),
            audio::start_background_audio_system.in_set(GameEnterSet::BuildLevel),
            run::setup_first_level.in_set(GameEnterSet::BuildLevel),
            arena::spawn_barriers_system.in_set(GameEnterSet::BuildLevel),
            arena::spawn_despawn_gates_system.in_set(GameEnterSet::BuildLevel),
            background::create_background_system.in_set(GameEnterSet::BuildLevel),
            ui::setup_game_ui_system.after(GameEnterSet::BuildUi),
        )
            .in_schedule(OnEnter(states::AppStates::Game)),
    );

    app.configure_sets(
        (
            //GameUpdateSet::Enter,
            GameUpdateSet::Level,
            GameUpdateSet::Spawn,
            GameUpdateSet::NextLevel,
            GameUpdateSet::UpdateUI,
            GameUpdateSet::SetTargetBehavior,
            GameUpdateSet::ContactCollision,
            GameUpdateSet::IntersectionCollision,
            GameUpdateSet::ExecuteBehavior,
            GameUpdateSet::ApplyDisconnectedBehaviors,
            GameUpdateSet::Movement,
            GameUpdateSet::Abilities,
        )
            .chain(),
    );
    app.add_systems(
        (
            player::player_movement_system.in_set(GameUpdateSet::Movement),
            player::player_ability_system.in_set(GameUpdateSet::Abilities),
            scanner::scanner_system,
            options::toggle_zoom_system,
            arena::despawn_gates_system,
            animation::animate_sprite_system,
            background::rotate_planet_system,
            spawnable::despawn_timer_system,
            spawnable::spawnable_set_target_behavior_system
                .in_set(GameUpdateSet::SetTargetBehavior),
            collision::intersection_collision_system.in_set(GameUpdateSet::IntersectionCollision),
            collision::contact_collision_system.in_set(GameUpdateSet::ContactCollision),
            spawnable::mob_behavior_sequence_tracker_system,
            spawnable::mob_behavior_sequence_update_system,
            spawnable::spawnable_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
            spawnable::mob_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
        )
            .in_set(OnUpdate(states::AppStates::Game)),
    );

    app.add_systems(
        (
            spawnable::mob_segment_apply_disconnected_behaviors_system
                .in_set(GameUpdateSet::ApplyDisconnectedBehaviors),
            spawnable::mob_segment_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
            spawnable::projectile_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
            spawnable::effect_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
            spawnable::consumable_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
            //run::level_system.in_set(GameUpdateSet::Level),
            //run::spawn_formation_system.in_set(GameUpdateSet::Spawn),
            //run::next_level_system.in_set(GameUpdateSet::NextLevel),
            player::player_fire_weapon_system,
            spawnable::spawn_effect_system, // event generated in projectile execute behavior, consumable execute behavior
            spawnable::spawn_projectile_system,
            spawnable::spawn_consumable_system, // event generated in mob execute behavior
            spawnable::spawn_mob_system,        // event generated in mob execute behavior
            states::open_pause_menu_system,
            player::player_death_system,
            ui::update_ui.after(GameUpdateSet::UpdateUI),
            ui::fade_out_system,
            player::player_scale_fire_rate_system,
        )
            .in_set(OnUpdate(states::AppStates::Game)),
    );

    app.add_systems(
        (
            run::level_system.in_set(GameUpdateSet::Level),
            run::spawn_formation_system.in_set(GameUpdateSet::Spawn),
            run::next_level_system.in_set(GameUpdateSet::NextLevel),
        )
            .in_set(OnUpdate(states::AppStates::Game)),
    );

    /*

    app.add_systems(
        (
            //player::player_ability_system.after("movement"),
            //player::player_movement_system.in_set("movement"),
            //scanner::scanner_system,
            //options::toggle_zoom_system,
            //arena::despawn_gates_system,
            //animation::animate_sprite_system,
            //background::rotate_planet_system,
            //spawnable::despawn_timer_system,
            //spawnable::spawnable_set_target_behavior_system.in_set("set_target_behavior"),
            //collision::intersection_collision_system.in_set("intersection_collision"),
            //collision::contact_collision_system.in_set("contact_collision"),
            //spawnable::mob_behavior_sequence_tracker_system,
            //spawnable::mob_behavior_sequence_update_system,
            //spawnable::spawnable_execute_behavior_system.after("set_target_behavior"),
            //spawnable::mob_execute_behavior_system
            //    .in_set("mob_execute_behavior")
            //    .after("set_target_behavior")
            //    .after("intersection_collision")
            //    .after("contact_collision"),
            //spawnable::mob_segment_apply_disconnected_behaviors_system
            //    .after("mob_execute_behavior")
            //    .after("mob_segment_execute_behavior"),
            //spawnable::mob_segment_execute_behavior_system
            //    .in_set("mob_segment_execute_behavior")
            //    .after("set_target_behavior")
            //    .after("intersection_collision")
            //    .after("contact_collision"),
            //spawnable::projectile_execute_behavior_system
            //    .in_set("projectile_execute_behavior")
            //    .after("set_target_behavior")
            //    .after("intersection_collision")
            //    .after("contact_collision"),
            //spawnable::effect_execute_behavior_system
            //    .after("set_target_behavior")
            //    .after("intersection_collision")
            //    .after("contact_collision"),
            //spawnable::consumable_execute_behavior_system
            //    .after("set_target_behavior")
            //    .after("intersection_collision")
            //    .after("contact_collision"),
            //run::level_system.in_set("level"),
            //run::spawn_formation_system.after("level"),
            //run::next_level_system.in_set("next_level").after("level"),
            //player::player_fire_weapon_system,
            //spawnable::spawn_effect_system, // event generated in projectile execute behavior, consumable execute behavior
            //spawnable::spawn_projectile_system,
            //spawnable::spawn_consumable_system, // event generated in mob execute behavior
            //spawnable::spawn_mob_system,        // event generated in mob execute behavior
            //states::open_pause_menu_system,
            //player::player_death_system,
            //ui::update_ui.after("next_level"),
            //ui::fade_out_system,
            //player::player_scale_fire_rate_system,
        )
            .in_schedule(OnUpdate(states::AppStates::Game)),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Game)
            .with_system(player::player_scale_fire_rate_system),
    );

    app.add_systems((ui::setup_pause_system).in_schedule(OnEnter(states::AppStates::PauseMenu)));

    app.add_systems((states::clear_state_system).in_schedule(OnExit(states::AppStates::PauseMenu)));

    app.add_systems(
        (
            ui::game_over_fade_in_system,
            run::reset_run_system,
            states::quit_game_system,
        )
            .in_schedule(OnUpdate(states::AppStates::GameOver)),
    );

    app.add_systems((ui::setup_game_over_system).in_schedule(OnEnter(states::AppStates::GameOver)));

    app.add_systems((states::clear_state_system).in_schedule(OnExit(states::AppStates::GameOver)));

    app.add_systems(
        (
            ui::victory_fade_in_system,
            run::reset_run_system,
            states::quit_game_system,
        )
            .in_schedule(OnUpdate(states::AppStates::Victory)),
    );

    app.add_systems((ui::setup_victory_system).in_schedule(OnEnter(states::AppStates::Victory)));

    app.add_systems((states::clear_state_system).in_schedule(OnExit(states::AppStates::Victory)));

    app.add_systems(
        (
            ui::setup_main_menu_system,
            audio::stop_background_audio_system,
        )
            .in_schedule(OnEnter(states::AppStates::MainMenu)),
    );

    app.add_systems(
        (states::start_game_system, states::quit_game_system)
            .in_schedule(OnUpdate(states::AppStates::MainMenu)),
    );

    app.add_systems((states::clear_state_system).in_schedule(OnExit(states::AppStates::MainMenu)));

    app.add_systems(
        (states::close_pause_menu_system, run::reset_run_system)
            .in_schedule(OnUpdate(states::AppStates::PauseMenu)),
    );



    if cfg!(debug_assertions) {
        app.add_system_set(
            SystemSet::on_update(states::AppStates::Game).with_system(ui::game_debug_ui),
        );
    }

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Game).with_system(states::clear_state_system),
    );

    // plugins to use only in debug mode
    if cfg!(debug_assertions) {
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(EditorPlugin)
            .add_startup_system(ui::setup_fps_ui_system)
            .add_system(ui::fps_system);
    }
    */

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
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new("Game Fade"));

    // create run resource
    run_resource.create_level(&levels_resource);
}
