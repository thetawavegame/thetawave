use bevy::{pbr::AmbientLight, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use ron::de::from_bytes;
use std::collections::HashMap;

pub const PHYSICS_SCALE: f32 = 10.0;
pub const SPAWNABLE_COL_GROUP_MEMBERSHIP: u32 = 0b0010;
pub const HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b0100;
pub const VERTICAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b1000;

mod animation;
mod arena;
mod background;
mod collision;
mod game;
mod loot;
mod misc;
mod options;
mod player;
mod run;
mod scanner;
mod spawnable;
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

    app.insert_resource(WindowDescriptor::from(display_config))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .insert_resource(
            from_bytes::<loot::LootDropsResource>(include_bytes!("../data/loot_drops.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<player::CharactersResource>(include_bytes!("../data/characters.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<run::FormationPoolsResource>(include_bytes!(
                "../data/formation_pools.ron"
            ))
            .unwrap(),
        )
        .insert_resource(
            from_bytes::<game::GameParametersResource>(include_bytes!(
                "../data/game_parameters.ron"
            ))
            .unwrap(),
        )
        .insert_resource(run::RunResource::from(
            from_bytes::<run::RunResourceData>(include_bytes!("../data/run.ron")).unwrap(),
        ))
        .insert_resource(run::LevelsResource::from(
            from_bytes::<run::LevelsResourceData>(include_bytes!("../data/levels.ron")).unwrap(),
        ))
        .insert_resource(spawnable::MobsResource {
            mobs: from_bytes::<HashMap<spawnable::MobType, spawnable::MobData>>(include_bytes!(
                "../data/mobs.ron"
            ))
            .unwrap(),
            texture_atlas_handle: HashMap::new(),
        })
        .insert_resource(spawnable::EffectsResource {
            effects: from_bytes::<HashMap<spawnable::EffectType, spawnable::EffectData>>(
                include_bytes!("../data/effects.ron"),
            )
            .unwrap(),
            texture_atlas_handle: HashMap::new(),
        })
        .insert_resource(spawnable::ProjectileResource {
            projectiles:
                from_bytes::<HashMap<spawnable::ProjectileType, spawnable::ProjectileData>>(
                    include_bytes!("../data/projectiles.ron"),
                )
                .unwrap(),
            texture_atlas_handle: HashMap::new(),
        })
        .insert_resource(spawnable::ConsumableResource {
            consumables:
                from_bytes::<HashMap<spawnable::ConsumableType, spawnable::ConsumableData>>(
                    include_bytes!("../data/consumables.ron"),
                )
                .unwrap(),
            texture_atlas_handle: HashMap::new(),
        })
        .insert_resource(
            from_bytes::<background::BackgroundsResource>(include_bytes!(
                "../data/backgrounds.ron"
            ))
            .unwrap(),
        )
        .add_event::<collision::SortedCollisionEvent>()
        .add_event::<run::SpawnFormationEvent>()
        .add_event::<run::LevelCompletedEvent>()
        .add_event::<arena::EnemyReachedBottomGateEvent>()
        .add_event::<spawnable::SpawnEffectEvent>()
        .add_event::<spawnable::SpawnConsumableEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PHYSICS_SCALE,
        ))
        .add_startup_system(setup_game.label("init"))
        .add_startup_system(arena::spawn_barriers_system.after("init"))
        .add_startup_system(arena::spawn_despawn_gates_system.after("init"))
        .add_startup_system(background::create_background_system.after("init"))
        .add_startup_system(
            player::spawn_player_system
                .label("spawn_player")
                .after("init"),
        )
        .add_startup_system(ui::setup_ui.after("spawn_player"))
        .add_system_to_stage(CoreStage::First, run::level_system.label("level"))
        .add_system_to_stage(CoreStage::First, run::spawn_formation_system.after("level"))
        .add_system_to_stage(CoreStage::First, run::next_level_system.after("level"))
        .add_system(player::player_movement_system)
        .add_system_to_stage(CoreStage::First, player::player_fire_weapon_system)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::spawnable_set_target_behavior_system.label("set_target_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::spawnable_execute_behavior_system.after("set_target_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::mob_execute_behavior_system
                .after("set_target_behavior")
                .after("intersection_collision")
                .after("contact_collision"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::projectile_execute_behavior_system
                .after("set_target_behavior")
                .after("intersection_collision")
                .after("contact_collision")
                .label("projectile_execute_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::effect_execute_behavior_system
                .after("set_target_behavior")
                .after("intersection_collision")
                .after("contact_collision")
                .label("effect_execute_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::consumable_execute_behavior_system
                .after("set_target_behavior")
                .after("intersection_collision")
                .after("contact_collision")
                .label("consumable_execute_behavior"),
        )
        .add_system_to_stage(CoreStage::First, spawnable::spawn_effect_system) // event generated in projectile execute behavior
        .add_system_to_stage(CoreStage::First, spawnable::spawn_consumable_system) // event generated in mob execute behavior
        .add_system_to_stage(
            CoreStage::PostUpdate,
            collision::intersection_collision_system.label("intersection_collision"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            collision::contact_collision_system.label("contact_collision"),
        )
        .add_system(scanner::scanner_system)
        .add_system(ui::update_ui)
        .add_system(spawnable::despawn_spawnable_system)
        .add_system(options::toggle_fullscreen_system)
        .add_system(options::toggle_zoom_system)
        .add_system(arena::despawn_gates_system)
        .add_system(animation::animate_sprite_system)
        .add_system(background::rotate_planet_system)
        .add_system(spawnable::despawn_timer_system);

    // plugins to use only in debug mode
    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default());
    }

    app.run();
}

/// Initialize values for the game
#[allow(clippy::too_many_arguments)]
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut mobs: ResMut<spawnable::MobsResource>,
    mut projectiles: ResMut<spawnable::ProjectileResource>,
    mut effects: ResMut<spawnable::EffectsResource>,
    mut consumables: ResMut<spawnable::ConsumableResource>,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut run_resource: ResMut<run::RunResource>,
    levels_resource: Res<run::LevelsResource>,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup cameras
    let mut camera_2d = OrthographicCameraBundle::new_2d();
    camera_2d.transform = Transform::from_xyz(0.0, 0.0, game_parameters.camera_z);
    commands.spawn_bundle(camera_2d);

    let camera_3d = PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        perspective_projection: PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(camera_3d);

    // setup rapier
    rapier_config.gravity = Vec2::ZERO;

    // load mob assets
    let mut mob_texture_atlas_dict = HashMap::new();
    for (mob_type, mob_data) in mobs.mobs.iter() {
        // mob texture
        let texture_handle = asset_server.load(&mob_data.texture.path[..]);
        let mob_atlas = TextureAtlas::from_grid(
            texture_handle,
            mob_data.texture.dimensions,
            mob_data.texture.cols,
            mob_data.texture.rows,
        );

        // thruster texture
        let thruster_atlas_handle = if let Some(thruster_data) = &mob_data.thruster {
            let thruster_texture_handle = asset_server.load(&thruster_data.texture.path[..]);
            Some(texture_atlases.add(TextureAtlas::from_grid(
                thruster_texture_handle,
                thruster_data.texture.dimensions,
                thruster_data.texture.cols,
                thruster_data.texture.rows,
            )))
        } else {
            None
        };

        // add mob and thruster texture handles to the dictionary
        mob_texture_atlas_dict.insert(
            mob_type.clone(),
            (texture_atlases.add(mob_atlas), thruster_atlas_handle),
        );
    }

    // load projectile assets
    let mut projectile_texture_atlas_dict = HashMap::new();
    for (projectile_type, projectile_data) in projectiles.projectiles.iter() {
        // projectile texture
        let texture_handle = asset_server.load(&projectile_data.texture.path[..]);
        let projectile_atlas = TextureAtlas::from_grid(
            texture_handle,
            projectile_data.texture.dimensions,
            projectile_data.texture.cols,
            projectile_data.texture.rows,
        );

        // add projectile texture handle to dictionary
        projectile_texture_atlas_dict.insert(
            projectile_type.clone(),
            texture_atlases.add(projectile_atlas),
        );
    }

    // load effect assets
    let mut effect_texture_atlas_dict = HashMap::new();
    for (effect_type, effect_data) in effects.effects.iter() {
        // effect texture
        let texture_handle = asset_server.load(&effect_data.texture.path[..]);
        let effect_atlas = TextureAtlas::from_grid(
            texture_handle,
            effect_data.texture.dimensions,
            effect_data.texture.cols,
            effect_data.texture.rows,
        );

        // add effect texture handle to dictionary
        effect_texture_atlas_dict.insert(effect_type.clone(), texture_atlases.add(effect_atlas));
    }

    // load consumable assets
    let mut consumable_texture_atlas_dict = HashMap::new();
    for (consumable_type, consumable_data) in consumables.consumables.iter() {
        // consumable texture
        let texture_handle = asset_server.load(&consumable_data.texture.path[..]);
        let consumable_atlas = TextureAtlas::from_grid(
            texture_handle,
            consumable_data.texture.dimensions,
            consumable_data.texture.cols,
            consumable_data.texture.rows,
        );

        // add consumable texture handle to dictionary
        consumable_texture_atlas_dict.insert(
            consumable_type.clone(),
            texture_atlases.add(consumable_atlas),
        );
    }

    // add texture atlas dict to the effects resource
    consumables.texture_atlas_handle = consumable_texture_atlas_dict;

    // add texture atlas dict to the effects resource
    effects.texture_atlas_handle = effect_texture_atlas_dict;

    // add texture atlas dict to the projectiles resource
    projectiles.texture_atlas_handle = projectile_texture_atlas_dict;

    // add texture atlas dict to the mobs resource
    mobs.texture_atlas_handle = mob_texture_atlas_dict;

    // create run resource
    run_resource.create_levels(&levels_resource);
}
