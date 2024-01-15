use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventWriter,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res},
    },
    math::Vec2,
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::dynamics::Velocity;
use std::time::Duration;
use thetawave_interface::{
    states::{AppStates, GameStates},
    weapon::{FireMode, SpreadPattern, WeaponComponent, WeaponProjectileData},
};

use crate::spawnable::{FireWeaponEvent, InitialMotion};
use rand::{thread_rng, Rng};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_weapon_system
                .run_if(in_state(AppStates::Game))
                .run_if(in_state(GameStates::Playing)),
        );
    }
}
trait WeaponExt {
    /// Updates the weapon's timers. Returns Some iff the weapon can be fired
    fn update(&mut self, delta_time: Duration) -> Option<WeaponProjectileData>;
}
impl WeaponExt for WeaponComponent {
    fn update(&mut self, delta_time: Duration) -> Option<WeaponProjectileData> {
        if self.is_enabled {
            // tick the initial timer if there is still time remaining
            // if the initial timer is finished then the reload timer is ticked
            if !self.initial_timer.finished() {
                self.initial_timer.tick(delta_time);
                None
            } else {
                self.reload_timer.tick(delta_time);

                // fire the weapon and return the projectile data if automatic
                // othewise return none
                match self.fire_mode {
                    FireMode::Automatic => self.fire_weapon(),
                    FireMode::Manual => None,
                }
            }
        } else {
            None
        }
    }
}

/// Update all weapons, and fire weapons with the automatic fire mode
pub fn update_weapon_system(
    mut weapon_query: Query<(Entity, &mut WeaponComponent, &Transform, &Velocity)>,
    time: Res<Time>,
    mut fire_weapon: EventWriter<FireWeaponEvent>,
) {
    for (entity, mut weapon, transform, velocity) in weapon_query.iter_mut() {
        if let Some(weapon_projectile_data) = weapon.update(time.delta()) {
            // pass velocity into the spawned blast
            let initial_motion = InitialMotion {
                linvel: Some(velocity.linvel),
                ..Default::default()
            };

            fire_weapon.send(FireWeaponEvent {
                weapon_projectile_data,
                source_transform: *transform,
                source_entity: entity,
                initial_motion,
            })
        }
    }
}

pub(crate) trait WeaponProjectileInitialVelocitiesExt {
    /// The initial velocities of `n` projectiles using existing/'partially evaluated' params.
    /// Could be evenly spaced, or something else based on the struct params.
    fn get_linvels(&self, max_projectiles: f32) -> Vec<Vec2>;
}
impl WeaponProjectileInitialVelocitiesExt for WeaponProjectileData {
    fn get_linvels(&self, max_projectiles: f32) -> Vec<Vec2> {
        match &self.spread_pattern {
            SpreadPattern::Arc(arc_pattern) => {
                // Get the segment of a spread angle
                let spread_angle_segment = {
                    // percentage of the game's maximum amount of projectiles being spawned
                    let total_projectiles_percent =
                        (self.count as f32 - 1.) / (max_projectiles - 1.);
                    // indicates the angle between the first and last projectile
                    let spread_arc = arc_pattern
                        .max_spread
                        .min(total_projectiles_percent * arc_pattern.projectile_gap);
                    // indicates the angle between each projectile
                    spread_arc / (self.count as f32 - 1.).max(1.)
                };

                let mut linvels = vec![];

                for p in 0..self.count {
                    // Calculate the angle for the current projectile.
                    // The first projectile is spread_angle_segment/2 radians to the left of the direction,
                    // and the last projectile is spread_angle_segment/2 radians to the right.
                    let angle_offset =
                        (p as f32 - (self.count as f32 - 1.) / 2.) * spread_angle_segment;
                    let projectile_angle = self.direction + angle_offset;

                    linvels.push(
                        Vec2::from_angle(projectile_angle)
                            * self.speed
                            * arc_pattern.spread_weights,
                    );
                }

                linvels
            }
            SpreadPattern::Random(random_pattern) => {
                let mut linvels = vec![];

                for _ in 0..self.count {
                    linvels.push(
                        // multiply the speed the projectile by a random angle and velocity multiplier
                        Vec2::from_angle(
                            thread_rng().gen_range(random_pattern.angle_range.clone()),
                        ) * self.speed
                            * thread_rng().gen_range(random_pattern.speed_range.clone()),
                    );
                }

                linvels
            }
        }
    }
}
