use bevy::{pbr::AmbientLight, prelude::*, render::camera::PerspectiveProjection};
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Vector2, prelude::*};
use ron::de::{from_bytes, from_str};
use std::{collections::HashMap, env::current_dir, fs::read_to_string};

use crate::spawnable::{MobData, MobType, MobsResource};

pub const SPAWNABLE_COL_GROUP_MEMBERSHIP: u32 = 0b0010;
pub const HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b0100;
pub const VERTICAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b1000;

mod arena;
mod background;
mod debug;
mod game;
mod options;
mod player;
mod spawnable;
mod tools;
mod visual;

fn main() {
    options::generate_config_files();

    let config_path = current_dir().unwrap().join("config");

    let display_config = from_str::<options::DisplayConfig>(
        &read_to_string(config_path.join("display.ron")).unwrap(),
    )
    .unwrap();

    let mut app = App::build();

    app.insert_resource(WindowDescriptor::from(display_config))
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .insert_resource(
            from_bytes::<player::CharactersResource>(include_bytes!("../data/characters.ron"))
                .unwrap(),
        )
        .insert_resource(
            from_bytes::<game::GameParametersResource>(include_bytes!(
                "../data/game_parameters.ron"
            ))
            .unwrap(),
        )
        .insert_resource(MobsResource {
            mobs: from_bytes::<HashMap<MobType, MobData>>(include_bytes!("../data/mobs.ron"))
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
        .insert_resource(spawnable::SpawnerResource::from(
            from_bytes::<spawnable::SpawnerResourceData>(include_bytes!("../data/spawner.ron"))
                .unwrap(),
        ))
        .insert_resource(
            from_bytes::<background::BackgroundsResource>(include_bytes!(
                "../data/backgrounds.ron"
            ))
            .unwrap(),
        )
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup_game.system().label("init"))
        .add_startup_system(arena::spawn_barriers_system.system().after("init"))
        .add_startup_system(arena::spawn_despawn_gates_system.system().after("init"))
        .add_startup_system(background::create_background_system.system().after("init"))
        .add_startup_system(player::spawn_player_system.system().after("init"))
        .add_system_to_stage(CoreStage::First, spawnable::spawner_system.system())
        .add_system(player::player_movement_system.system())
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::spawnable_set_target_behavior_system
                .system()
                .label("set_target_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::spawnable_set_contact_behavior_system
                .system()
                .label("set_contact_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::spawnable_execute_behavior_system
                .system()
                .after("set_contact_behavior")
                .after("set_target_behavior"),
        )
        .add_system_to_stage(
            CoreStage::PostUpdate,
            spawnable::mob_execute_behavior_system
                .system()
                .after("set_contact_behavior")
                .after("set_target_behavior"),
        )
        .add_system(spawnable::despawn_spawnable_system.system())
        .add_system(options::toggle_fullscreen_system.system())
        .add_system(options::toggle_zoom_system.system())
        .add_system(arena::despawn_gates_system.system())
        .add_system(visual::animate_sprite_system.system())
        .add_system(background::rotate_planet_system.system())
        .add_system(spawnable::display_events.system());

    if cfg!(debug_assertions) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_system(debug::collider_debug_lines_system.system());
    }

    app.run();
}

/// Initialize values for the game
fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut mobs: ResMut<MobsResource>,
    mut projectiles: ResMut<spawnable::ProjectileResource>,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_parameters: Res<game::GameParametersResource>,
) {
    // setup camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 0.0, game_parameters.camera_z)
            .looking_at(Vec3::ZERO, Vec3::Y),
        perspective_projection: PerspectiveProjection {
            far: 10000.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // setup rapier
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = game_parameters.physics_scale;

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

    // add texture atlas dict to the projectiles resource
    projectiles.texture_atlas_handle = projectile_texture_atlas_dict;

    // add texture atlas dict to the mobs resource
    mobs.texture_atlas_handle = mob_texture_atlas_dict;
}
