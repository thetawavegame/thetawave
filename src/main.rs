use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::{pbr::AmbientLight, prelude::*};
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_kira_audio::prelude::*;

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

    // insert resources for all game states
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor::from(display_config),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .insert_resource(ClearColor(Color::BLACK))
    .insert_resource(
        from_bytes::<spawnable::MobBehaviorAttributesResource>(include_bytes!(
            "../assets/data/mob_behavior_attributes.ron"
        ))
        .unwrap(),
    )
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
        from_bytes::<player::CharactersResource>(include_bytes!("../assets/data/characters.ron"))
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
    .insert_resource(spawnable::MobSegmentsResource {
        mob_segments: from_bytes::<HashMap<spawnable::MobSegmentType, spawnable::MobSegmentData>>(
            include_bytes!("../assets/data/mob_segments.ron"),
        )
        .unwrap(),
    })
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
    .add_plugin(AudioPlugin)
    .add_plugin(EguiPlugin)
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
    .add_system_to_stage(CoreStage::Last, ui::position_stat_bar_label_system);

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(bevy_framepace::FramepacePlugin);
    }

    // add states
    app.add_state(states::AppStates::MainMenu); // start game in the main menu state
    app.add_loading_state(
        LoadingState::new(states::AppStates::LoadingGame)
            .continue_to_state(states::AppStates::Game)
            .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                "player_assets.assets",
                "projectile_assets.assets",
                "mob_assets.assets",
                "consumable_assets.assets",
                "effect_assets.assets",
                "game_audio_assets.assets",
            ])
            .with_collection::<assets::PlayerAssets>()
            .with_collection::<assets::ProjectileAssets>()
            .with_collection::<assets::MobAssets>()
            .with_collection::<assets::ConsumableAssets>()
            .with_collection::<assets::EffectAssets>()
            .with_collection::<assets::GameAudioAssets>(),
    );

    // game startup systems (perhaps exchange with app.add_startup_system_set)
    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Game)
            .with_system(audio::start_background_audio_system)
            .with_system(run::setup_first_level.after("init"))
            .with_system(setup_game.label("init"))
            .with_system(setup_physics.label("init"))
            .with_system(arena::spawn_barriers_system.after("init"))
            .with_system(arena::spawn_despawn_gates_system.after("init"))
            .with_system(background::create_background_system.after("init"))
            .with_system(
                player::spawn_player_system
                    .label("spawn_player")
                    .after("init"),
            )
            .with_system(ui::setup_game_ui_system.after("spawn_player")),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::PauseMenu).with_system(ui::setup_pause_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::PauseMenu).with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::GameOver)
            .with_system(ui::game_over_fade_in_system)
            .with_system(run::reset_run_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::GameOver).with_system(ui::setup_game_over_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::GameOver).with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Victory)
            .with_system(ui::victory_fade_in_system)
            .with_system(run::reset_run_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::Victory).with_system(ui::setup_victory_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::Victory).with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_enter(states::AppStates::MainMenu).with_system(ui::setup_main_menu_system), //.with_system(states::clear_game_state_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::MainMenu)
            .with_system(states::start_game_system)
            .with_system(states::quit_game_system),
    );

    app.add_system_set(
        SystemSet::on_exit(states::AppStates::MainMenu).with_system(states::clear_state_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::PauseMenu)
            .with_system(states::close_pause_menu_system)
            .with_system(run::reset_run_system),
    );

    app.add_system_set(
        SystemSet::on_update(states::AppStates::Game)
            .with_system(player::player_movement_system)
            .with_system(scanner::scanner_system)
            .with_system(options::toggle_zoom_system)
            .with_system(arena::despawn_gates_system)
            .with_system(animation::animate_sprite_system)
            .with_system(background::rotate_planet_system)
            .with_system(spawnable::despawn_timer_system)
            .with_system(
                spawnable::spawnable_set_target_behavior_system.label("set_target_behavior"),
            )
            .with_system(collision::intersection_collision_system.label("intersection_collision"))
            .with_system(collision::contact_collision_system.label("contact_collision"))
            .with_system(spawnable::mob_behavior_sequence_tracker_system)
            .with_system(spawnable::mob_behavior_sequence_update_system)
            .with_system(spawnable::spawnable_execute_behavior_system.after("set_target_behavior"))
            .with_system(
                spawnable::mob_execute_behavior_system, //.after("set_target_behavior")
                                                        //.after("intersection_collision")
                                                        //.after("contact_collision"),
            )
            .with_system(
                spawnable::mob_segment_execute_behavior_system
                    .after("set_target_behavior")
                    .after("intersection_collision")
                    .after("contact_collision"),
            )
            .with_system(
                spawnable::projectile_execute_behavior_system
                    .after("set_target_behavior")
                    .after("intersection_collision")
                    .after("contact_collision")
                    .label("projectile_execute_behavior"),
            )
            .with_system(
                spawnable::effect_execute_behavior_system
                    .after("set_target_behavior")
                    .after("intersection_collision")
                    .after("contact_collision"),
            )
            .with_system(
                spawnable::consumable_execute_behavior_system
                    .after("set_target_behavior")
                    .after("intersection_collision")
                    .after("contact_collision"),
            )
            .with_system(run::level_system.label("level"))
            .with_system(run::spawn_formation_system.after("level"))
            .with_system(run::next_level_system.label("next_level").after("level"))
            .with_system(player::player_fire_weapon_system)
            .with_system(spawnable::spawn_effect_system) // event generated in projectile execute behavior, consumable execute behavior
            .with_system(spawnable::spawn_projectile_system)
            .with_system(spawnable::spawn_consumable_system) // event generated in mob execute behavior
            .with_system(spawnable::spawn_mob_system) // event generated in mob execute behavior
            .with_system(states::open_pause_menu_system)
            .with_system(player::player_death_system)
            .with_system(ui::update_ui.after("next_level"))
            .with_system(ui::fade_out_system)
            .with_system(player::player_scale_fire_rate_system),
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
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(ui::setup_fps_ui_system)
            .add_system(ui::fps_system);
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
        .insert(AppStateComponent(AppStates::Game))
        .insert(Name::new("Game Fade"));

    // create run resource
    run_resource.create_level(&levels_resource);
}
