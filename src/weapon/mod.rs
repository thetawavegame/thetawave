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
use thetawave_interface::{
    states::{AppStates, GameStates},
    weapon::WeaponComponent,
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
