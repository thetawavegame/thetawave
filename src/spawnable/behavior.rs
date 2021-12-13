use crate::{
    collision::CollisionEvent,
    game::GameParametersResource,
    player::PlayerComponent,
    spawnable::{EnemyType, MobType, SpawnableComponent, SpawnableType},
    tools::signed_modulo,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;

/// Types of behaviors that can be performed by spawnables
#[derive(Deserialize, Clone)]
pub enum SpawnableBehavior {
    RotateToTarget(Option<Vec2>),
    MoveForward,
    MoveDown,
    MoveRight,
    MoveLeft,
    BrakeHorizontal,
    ChangeHorizontalDirectionOnImpact,
}

/// Manages excuting behaviors of spawnables
pub fn spawnable_execute_behavior_system(
    rapier_config: Res<RapierConfiguration>,
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(
        Entity,
        &mut SpawnableComponent,
        &mut RigidBodyVelocity,
        &Transform,
    )>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    // Iterate through all spawnable entities and execute their behavior
    for (spawnable_entity, mut spawnable_component, mut rb_vel, spawnable_transform) in
        spawnable_query.iter_mut()
    {
        let behaviors = spawnable_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                SpawnableBehavior::MoveDown => {
                    move_down(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveRight => {
                    move_right(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveLeft => {
                    move_left(&rapier_config, &spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::RotateToTarget(target_position) => {
                    rotate_to_target(
                        spawnable_transform,
                        target_position.unwrap(),
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                SpawnableBehavior::MoveForward => {
                    move_forward(
                        &rapier_config,
                        spawnable_transform,
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                SpawnableBehavior::BrakeHorizontal => {
                    brake_horizontal(
                        &rapier_config,
                        &game_parameters,
                        &spawnable_component,
                        &mut rb_vel,
                    );
                }
                SpawnableBehavior::ChangeHorizontalDirectionOnImpact => {
                    change_horizontal_direction_on_impact(
                        spawnable_entity,
                        &collision_events_vec,
                        &mut spawnable_component,
                    );
                }
            }
        }
    }
}

/// Manages setting targeting of spawnables
pub fn spawnable_set_target_behavior_system(
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut spawnable_query: Query<(&mut SpawnableComponent, &Transform)>,
) {
    // Sets targetting to None
    for (mut spawnable_component, _) in spawnable_query.iter_mut() {
        for behavior in spawnable_component.behaviors.iter_mut() {
            if let SpawnableBehavior::RotateToTarget(_) = behavior {
                *behavior = SpawnableBehavior::RotateToTarget(None);
            }
        }
    }

    // Recalculates what the target should be
    for player_transform in player_query.iter() {
        for (mut spawnable_component, spawnable_transform) in spawnable_query.iter_mut() {
            match &spawnable_component.spawnable_type {
                SpawnableType::Mob(mob_type) => match mob_type {
                    MobType::Enemy(enemy_type) => match enemy_type {
                        EnemyType::Missile => {
                            // set target to closest player
                            for behavior in spawnable_component.behaviors.iter_mut() {
                                *behavior = match behavior {
                                    SpawnableBehavior::RotateToTarget(target) => {
                                        let spawnable_position_vec2: Vec2 =
                                            spawnable_transform.translation.into();
                                        let player_position_vec2: Vec2 =
                                            player_transform.translation.into();
                                        if target.is_none()
                                            || spawnable_position_vec2
                                                .distance(player_position_vec2)
                                                < spawnable_position_vec2.distance(target.unwrap())
                                        {
                                            SpawnableBehavior::RotateToTarget(Some(
                                                player_position_vec2,
                                            ))
                                        } else {
                                            behavior.clone()
                                        }
                                    }
                                    _ => behavior.clone(),
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

/// Toggles the horizontal direction of a spawnable on impact
fn change_horizontal_direction_on_impact(
    entity: Entity,
    collision_events: &[&CollisionEvent],
    spawnable_component: &mut SpawnableComponent,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToMobContact { mob_entity, .. }
            | CollisionEvent::MobToMobContact {
                mob_entity_1: mob_entity,
                ..
            }
            | CollisionEvent::MobToBarrierContact { mob_entity, .. } => {
                if entity == *mob_entity {
                    for behavior in spawnable_component.behaviors.iter_mut() {
                        *behavior = match behavior {
                            SpawnableBehavior::MoveRight => SpawnableBehavior::MoveLeft,
                            SpawnableBehavior::MoveLeft => SpawnableBehavior::MoveRight,
                            _ => behavior.clone(),
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

/// Rotates entity to face target
fn rotate_to_target(
    transform: &Transform,
    target_position: Vec2,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    let mut target_angle = ((transform.translation.y - target_position.y)
        .atan2(transform.translation.x - target_position.x))
    .to_degrees()
        + 90.0;

    if target_angle < 0.0 {
        target_angle += 360.0;
    }

    let current_angle = (transform.rotation.to_axis_angle().1.to_degrees()
        * transform.rotation.to_axis_angle().0.z)
        + 180.0;

    let mut smallest_angle = signed_modulo(target_angle - current_angle, 360.0);
    if smallest_angle > 180.0 {
        smallest_angle = -(360.0 - smallest_angle);
    }

    if smallest_angle < 0.0 {
        if rb_vel.angvel > -spawnable_component.angular_speed {
            rb_vel.angvel -= spawnable_component.angular_acceleration;
        }
    } else if rb_vel.angvel < spawnable_component.angular_speed {
        rb_vel.angvel += spawnable_component.angular_acceleration;
    }
}

/// Move entity forward along it's axis
fn move_forward(
    rapier_config: &RapierConfiguration,
    transform: &Transform,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    let angle = (transform.rotation.to_axis_angle().1 * transform.rotation.to_axis_angle().0.z)
        - (std::f32::consts::FRAC_PI_2);

    let max_speed_x = (spawnable_component.speed.x * angle.cos() * rapier_config.scale).abs();
    let max_speed_y = (spawnable_component.speed.y * angle.sin() * rapier_config.scale).abs();

    if rb_vel.linvel.x > max_speed_x {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    } else if rb_vel.linvel.x < -max_speed_x {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x += spawnable_component.acceleration.x * angle.cos() * rapier_config.scale;
    }

    if rb_vel.linvel.y > max_speed_y {
        rb_vel.linvel.y -= spawnable_component.deceleration.y * rapier_config.scale;
    } else if rb_vel.linvel.y < -max_speed_y {
        rb_vel.linvel.y += spawnable_component.deceleration.y * rapier_config.scale;
    } else {
        rb_vel.linvel.y += spawnable_component.acceleration.x * angle.sin() * rapier_config.scale;
    }
}

/// Moves entity down
fn move_down(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    //move down
    if rb_vel.linvel.y > spawnable_component.speed.y * rapier_config.scale * -1.0 {
        rb_vel.linvel.y -= spawnable_component.acceleration.y * rapier_config.scale;
    } else {
        rb_vel.linvel.y += spawnable_component.deceleration.y * rapier_config.scale;
    }
}

/// Moves entity right
fn move_right(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    if rb_vel.linvel.x < spawnable_component.speed.x * rapier_config.scale {
        rb_vel.linvel.x += spawnable_component.acceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    }
}

/// Moves entity left
fn move_left(
    rapier_config: &RapierConfiguration,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    if rb_vel.linvel.x > spawnable_component.speed.x * rapier_config.scale * -1.0 {
        rb_vel.linvel.x -= spawnable_component.acceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    }
}

/// Decelerates to 0 horizontal movement
fn brake_horizontal(
    rapier_config: &RapierConfiguration,
    game_parameters: &GameParametersResource,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut RigidBodyVelocity,
) {
    // decelerate in x direction
    if rb_vel.linvel.x > game_parameters.stop_threshold {
        rb_vel.linvel.x -= spawnable_component.deceleration.x * rapier_config.scale;
    } else if rb_vel.linvel.x < game_parameters.stop_threshold * -1.0 {
        rb_vel.linvel.x += spawnable_component.deceleration.x * rapier_config.scale;
    } else {
        rb_vel.linvel.x = 0.0;
    }
}
