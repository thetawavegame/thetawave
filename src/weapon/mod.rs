use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventWriter,
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Query, Res},
    },
    time::Time,
    transform::components::Transform,
};
use bevy_rapier2d::dynamics::Velocity;
use std::time::Duration;
use thetawave_interface::{
    states::{AppStates, GameStates},
    weapon::{FireMode, WeaponComponent, WeaponProjectileData},
};

use crate::spawnable::{FireWeaponEvent, InitialMotion};

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
    fn update(&mut self, delta_time: Duration) -> Option<WeaponProjectileData>;
}
impl WeaponExt for WeaponComponent {
    /// Updates the weapon's timers
    /// Returns true if the weapon can be fired
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
