use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Vector2, prelude::*};
use ron::de::{from_bytes, from_str};
use std::{collections::HashMap, env::current_dir, fs::read_to_string};

use crate::spawnable::{MobData, MobType, MobsResource};

pub const SPAWNABLE_COL_GROUP_MEMBERSHIP: u32 = 0b0010;
pub const HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b0100;
pub const VERTICAL_BARRIER_COL_GROUP_MEMBERSHIP: u32 = 0b1000;

mod debug;
mod game;
mod misc;
mod options;
mod player;
mod spawnable;

fn main() {
    options::generate_config_files();

    let config_path = current_dir().unwrap().join("config");

    let display_config = from_str::<options::DisplayConfig>(
        &read_to_string(config_path.join("display.ron")).unwrap(),
    )
    .unwrap();

    let mut app = App::build();

    app.insert_resource(WindowDescriptor::from(display_config))
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
        .insert_resource(
            from_bytes::<spawnable::SpawnerResource>(include_bytes!("../data/spawner.ron"))
                .unwrap(),
        )
        //.insert_resource(SpawnableTextureAtlasHandleIds::new())
        .insert_resource(spawnable::SpawnerTimer(Timer::from_seconds(5.0, true)))
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(DebugLinesPlugin)
        .add_startup_system(setup_game.system().label("init"))
        //.add_startup_system(spawnable::init_spawner_system.system())
        .add_startup_system(misc::spawn_barrier_system.system().after("init"))
        .add_startup_system(player::spawn_player_system.system().after("init"))
        .add_system_to_stage(CoreStage::First, spawnable::spawn_formation_system.system())
        //.add_startup_system(spawnable::spawn_mob_system.system().after("init"))
        .add_system(player::player_movement_system.system())
        .add_system(spawnable::mob_movement_system.system())
        .add_system(options::toggle_fullscreen_system.system())
        .add_system(misc::despawn_spawnables_system.system())
        .add_system(animate_sprite_system.system());

    if cfg!(debug_assertions) {
        app.add_system(debug::collider_debug_lines_system.system());
    }

    app.run();
}

fn setup_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut mobs: ResMut<MobsResource>,
    mut rapier_config: ResMut<RapierConfiguration>,
    game_parameters: Res<game::GameParametersResource>,
) {
    // spawn camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // setup rapier
    rapier_config.gravity = Vector2::zeros();
    rapier_config.scale = game_parameters.physics_scale;

    // load assets
    let mut mob_texture_atlas_dict = HashMap::new();
    for (mob_type, mob_data) in mobs.mobs.iter() {
        let texture_handle = asset_server.load(&mob_data.texture_path[..]);
        let atlas = TextureAtlas::from_grid(
            texture_handle,
            mob_data.sprite_dimensions,
            mob_data.texture_atlas_cols,
            mob_data.texture_atlas_rows,
        );
        mob_texture_atlas_dict.insert(mob_type.clone(), texture_atlases.add(atlas));
    }

    mobs.texture_atlas_handle = mob_texture_atlas_dict;
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}
