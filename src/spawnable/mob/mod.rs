use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    game::GameParametersResource,
    spawnable::{AllyType, EnemyType, MobType, SpawnableType},
    SpawnableTextureAtlasHandleIds, HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
    SPAWNABLE_COL_GROUP_MEMBERSHIP,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct MobComponent {
    /// Type of mob
    pub mob_type: MobType,
    /// Acceleration of the player
    pub acceleration: Vec2,
    /// Deceleration of the player
    pub deceleration: Vec2,
    /// Maximum speed of the player
    pub speed: Vec2,
}

#[derive(Deserialize)]
pub struct MobData {
    pub mob_type: MobType,
    pub acceleration: Vec2,
    pub deceleration: Vec2,
    pub speed: Vec2,
    pub collider_dimensions: Vec2,
    pub sprite_dimensions: Vec2,
    pub texture_path: String,
    pub texture_atlas_cols: usize,
    pub texture_atlas_rows: usize,
}

#[derive(Deserialize)]
pub struct MobsResource {
    pub mobs: HashMap<MobType, MobData>,
}

pub fn spawn_mob_system(
    mut commands: Commands,
    mobs: Res<MobsResource>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_atlas_handle_ids: Res<SpawnableTextureAtlasHandleIds>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    let mob_data = &mobs.mobs[&MobType::Enemy(EnemyType::MissileLauncher)];

    spawn_mob(
        mob_data,
        Vec2::new(0.0, 20.0),
        &mut commands,
        &texture_atlases,
        &texture_atlas_handle_ids,
        &rapier_config,
        &game_parameters,
    );
}

pub fn spawn_mob(
    mob_data: &MobData,
    position: Vec2,
    commands: &mut Commands,
    texture_atlases: &Res<Assets<TextureAtlas>>,
    texture_atlas_handle_ids: &Res<SpawnableTextureAtlasHandleIds>,
    rapier_config: &Res<RapierConfiguration>,
    game_parameters: &Res<GameParametersResource>,
) {
    // scale collider to align with the sprite
    let collider_size_hx =
        mob_data.collider_dimensions.x * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy =
        mob_data.collider_dimensions.y * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    let texture_atlas_handle = texture_atlases
        .get_handle(texture_atlas_handle_ids[&SpawnableType::Mob(mob_data.mob_type.clone())]);

    let transform = Transform::from_scale(Vec3::new(
        game_parameters.sprite_scale,
        game_parameters.sprite_scale,
        1.0,
    ));

    commands
        .spawn()
        .insert_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform,
            ..Default::default()
        })
        .insert(Timer::from_seconds(0.1, true))
        .insert_bundle(RigidBodyBundle {
            body_type: RigidBodyType::Dynamic,
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            position: position.into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
            material: ColliderMaterial {
                friction: 1.0,
                restitution: 1.0,
                restitution_combine_rule: CoefficientCombineRule::Max,
                ..Default::default()
            },
            flags: ColliderFlags {
                collision_groups: InteractionGroups::new(
                    SPAWNABLE_COL_GROUP_MEMBERSHIP,
                    u32::MAX ^ HORIZONTAL_BARRIER_COL_GROUP_MEMBERSHIP,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(MobComponent {
            mob_type: MobType::Enemy(EnemyType::Drone),
            acceleration: mob_data.acceleration,
            deceleration: mob_data.deceleration,
            speed: mob_data.speed,
        });
}

pub fn mob_movement_system(
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut mob_query: Query<(&MobComponent, &mut RigidBodyVelocity)>,
) {
    for (enemy, mut rb_vels) in mob_query.iter_mut() {
        //move down
        if rb_vels.linvel.y > enemy.speed.y * rapier_config.scale * -1.0 {
            rb_vels.linvel.y -= enemy.acceleration.y * rapier_config.scale;
        } else {
            rb_vels.linvel.y += enemy.deceleration.y * rapier_config.scale;
        }

        // decelerate in x direction
        if rb_vels.linvel.x > game_parameters.stop_threshold {
            rb_vels.linvel.x -= enemy.deceleration.x;
        } else if rb_vels.linvel.x < game_parameters.stop_threshold * -1.0 {
            rb_vels.linvel.x += enemy.deceleration.x;
        } else {
            rb_vels.linvel.x = 0.0;
        }
    }
}
