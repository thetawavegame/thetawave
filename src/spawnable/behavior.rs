use crate::{
    collision::SortedCollisionEvent, game::GameParametersResource, spawnable::SpawnableComponent,
    tools::signed_modulo,
};
use bevy::prelude::{Entity, EventReader, Query, Res, Transform, Vec2, Vec3Swizzles, With};
use bevy_rapier2d::prelude::Velocity;
use serde::Deserialize;
use thetawave_interface::player::PlayerAttractionComponent;
use thetawave_interface::spawnable::AttractToClosestPlayerComponent;
use thetawave_interface::{
    player::PlayerComponent,
    spawnable::{EnemyMobType, MobType, SpawnableType},
};

/// Types of behaviors that can be performed by spawnables
#[derive(Deserialize, Clone, PartialEq)]
pub enum SpawnableBehavior {
    RotateToTarget(Option<Vec2>),
    MoveForward,
    MoveDown,
    MoveRight,
    MoveLeft,
    BrakeHorizontal,
    ChangeHorizontalDirectionOnImpact,
    MoveToPosition(Vec2),
    AttractToPlayer,
}

/// Manages excuting behaviors of spawnables
pub fn spawnable_execute_behavior_system(
    game_parameters: Res<GameParametersResource>,
    mut spawnable_query: Query<(Entity, &mut SpawnableComponent, &mut Velocity, &Transform)>,
    mut collision_events: EventReader<SortedCollisionEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.read() {
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
                    move_down(&spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveRight => {
                    move_right(&spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveLeft => {
                    move_left(&spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::RotateToTarget(target_position) => {
                    if let Some(target_position) = target_position {
                        rotate_to_target(
                            spawnable_transform,
                            target_position,
                            &spawnable_component,
                            &mut rb_vel,
                        );
                    }
                }
                SpawnableBehavior::MoveForward => {
                    move_forward(spawnable_transform, &spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::BrakeHorizontal => {
                    brake_horizontal(&game_parameters, &spawnable_component, &mut rb_vel);
                }
                SpawnableBehavior::MoveToPosition(pos) => {
                    move_to_position(spawnable_transform, &spawnable_component, &mut rb_vel, pos);
                }
                SpawnableBehavior::ChangeHorizontalDirectionOnImpact => {
                    change_horizontal_direction_on_impact(
                        spawnable_entity,
                        &collision_events_vec,
                        &mut spawnable_component,
                    );
                }
                _ => {}
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
                        EnemyMobType::Missile => {
                            // set target to closest player
                            for behavior in spawnable_component.behaviors.iter_mut() {
                                *behavior = match behavior {
                                    SpawnableBehavior::RotateToTarget(target) => {
                                        let spawnable_position_vec2: Vec2 =
                                            spawnable_transform.translation.xy();
                                        let player_position_vec2: Vec2 =
                                            player_transform.translation.xy();
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

fn move_to_position(
    transform: &Transform,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut Velocity,
    position: Vec2,
) {
    let angle = ((transform.translation.y - position.y)
        .atan2(transform.translation.x - position.x))
        - (std::f32::consts::PI);

    let max_speed_x = (spawnable_component.speed.x * angle.cos()).abs();
    let max_speed_y = (spawnable_component.speed.y * angle.sin()).abs();

    if rb_vel.linvel.x > max_speed_x {
        rb_vel.linvel.x -= spawnable_component.deceleration.x;
    } else if rb_vel.linvel.x < -max_speed_x {
        rb_vel.linvel.x += spawnable_component.deceleration.x;
    } else {
        rb_vel.linvel.x += spawnable_component.acceleration.x * angle.cos();
    }

    if rb_vel.linvel.y > max_speed_y {
        rb_vel.linvel.y -= spawnable_component.deceleration.y;
    } else if rb_vel.linvel.y < -max_speed_y {
        rb_vel.linvel.y += spawnable_component.deceleration.y;
    } else {
        rb_vel.linvel.y += spawnable_component.acceleration.y * angle.sin();
    }
}

/// Toggles the horizontal direction of a spawnable on impact
fn change_horizontal_direction_on_impact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    spawnable_component: &mut SpawnableComponent,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToMobContact { mob_entity, .. }
            | SortedCollisionEvent::MobToMobContact {
                mob_entity_1: mob_entity,
                ..
            }
            | SortedCollisionEvent::MobToBarrierContact { mob_entity, .. }
            | SortedCollisionEvent::MobToMobSegmentContact { mob_entity, .. } => {
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
    rb_vel: &mut Velocity,
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
    transform: &Transform,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut Velocity,
) {
    let angle = (transform.rotation.to_axis_angle().1 * transform.rotation.to_axis_angle().0.z)
        - (std::f32::consts::FRAC_PI_2);

    let max_speed_x = (spawnable_component.speed.x * angle.cos()).abs();
    let max_speed_y = (spawnable_component.speed.y * angle.sin()).abs();

    if rb_vel.linvel.x > max_speed_x {
        rb_vel.linvel.x -= spawnable_component.deceleration.x;
    } else if rb_vel.linvel.x < -max_speed_x {
        rb_vel.linvel.x += spawnable_component.deceleration.x;
    } else {
        rb_vel.linvel.x += spawnable_component.acceleration.x * angle.cos();
    }

    if rb_vel.linvel.y > max_speed_y {
        rb_vel.linvel.y -= spawnable_component.deceleration.y;
    } else if rb_vel.linvel.y < -max_speed_y {
        rb_vel.linvel.y += spawnable_component.deceleration.y;
    } else {
        rb_vel.linvel.y += spawnable_component.acceleration.x * angle.sin();
    }
}

/// Moves entity down
fn move_down(spawnable_component: &SpawnableComponent, rb_vel: &mut Velocity) {
    //move down
    if rb_vel.linvel.y > spawnable_component.speed.y * -1.0 {
        rb_vel.linvel.y -= spawnable_component.acceleration.y;
    } else {
        rb_vel.linvel.y += spawnable_component.deceleration.y;
    }
}

/// Moves entity right
fn move_right(spawnable_component: &SpawnableComponent, rb_vel: &mut Velocity) {
    if rb_vel.linvel.x < spawnable_component.speed.x {
        rb_vel.linvel.x += spawnable_component.acceleration.x;
    } else {
        rb_vel.linvel.x -= spawnable_component.deceleration.x;
    }
}

/// Moves entity left
fn move_left(spawnable_component: &SpawnableComponent, rb_vel: &mut Velocity) {
    if rb_vel.linvel.x > spawnable_component.speed.x * -1.0 {
        rb_vel.linvel.x -= spawnable_component.acceleration.x;
    } else {
        rb_vel.linvel.x += spawnable_component.deceleration.x;
    }
}

/// Decelerates to 0 horizontal movement
fn brake_horizontal(
    game_parameters: &GameParametersResource,
    spawnable_component: &SpawnableComponent,
    rb_vel: &mut Velocity,
) {
    // decelerate in x direction
    if rb_vel.linvel.x > game_parameters.stop_threshold {
        rb_vel.linvel.x -= spawnable_component.deceleration.x;
    } else if rb_vel.linvel.x < game_parameters.stop_threshold * -1.0 {
        rb_vel.linvel.x += spawnable_component.deceleration.x;
    } else {
        rb_vel.linvel.x = 0.0;
    }
}
/// Nudge each "attractive" item toward the closest player based on that player's "gravity constant"
pub(super) fn attract_to_player_system(
    mut spawnable_query: Query<
        (&mut Velocity, &Transform),
        (
            With<AttractToClosestPlayerComponent>,
            With<SpawnableComponent>,
        ),
    >,
    player_query: Query<(&PlayerAttractionComponent, &Transform)>,
) {
    let player_positions_and_accels_and_cutoff_distances: Vec<(Vec2, f32, f32)> = player_query
        .iter()
        .map(|(player_attraction, transform)| {
            (
                transform.translation.xy(),
                player_attraction.acceleration,
                player_attraction.distance,
            )
        })
        .collect();

    for (mut spawnable_velocity, spawnable_transform) in spawnable_query.iter_mut() {
        let item_position = spawnable_transform.translation.xy();
        if let Some((
            distance_to_player,
            closest_player_position,
            attraction_accel,
            cutoff_distance,
        )) = player_positions_and_accels_and_cutoff_distances
            .iter()
            .map(|(player_position, accel, cutoff)| {
                (
                    player_position.distance(item_position),
                    player_position,
                    accel,
                    cutoff,
                )
            })
            // Approximation of distance since floats are not totally ordered b/c of NANs. Maybe use
            // https://crates.io/crates/ordered-float if the approximation is not good enough
            .min_by_key(|(distance, _, _, _)| (distance * 100.0) as i32)
        {
            if distance_to_player <= *cutoff_distance {
                let direction =
                    (*closest_player_position - spawnable_transform.translation.xy()).normalize();
                spawnable_velocity.linvel += *attraction_accel * direction;
            }
        }
    }
}
