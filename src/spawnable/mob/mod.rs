use crate::{
    game::GameParametersResource,
    spawnable::{EnemyType, MobType, SpawnableType},
    TextureAtlasHandleIds,
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

pub fn spawn_mob_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    texture_handle_atlas_ids: Res<TextureAtlasHandleIds>,
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
) {
    // scale collider to align with the sprite
    let collider_size_hx = 14.0 * game_parameters.sprite_scale / rapier_config.scale / 2.0;
    let collider_size_hy = 14.0 * game_parameters.sprite_scale / rapier_config.scale / 2.0;

    let texture_atlas_handle = texture_atlases.get_handle(texture_handle_atlas_ids["drone"]);

    let transform = Transform::from_scale(Vec3::new(
        game_parameters.sprite_scale,
        game_parameters.sprite_scale,
        1.0,
    ));

    println!("{:?}", texture_atlas_handle);

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
            position: Vec2::new(0.0, 20.0).into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(collider_size_hx, collider_size_hy),
            material: ColliderMaterial {
                friction: 0.0,
                restitution: 1.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ColliderPositionSync::Discrete)
        .insert(MobComponent {
            mob_type: MobType::Enemy(EnemyType::Drone),
            acceleration: Vec2::new(0.0, 0.02),
            deceleration: Vec2::new(0.05, 0.005),
            speed: Vec2::new(0.0, 1.0),
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
