use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::GameParametersResource;
use crate::spawnable::MobComponent;

pub fn scanner_system(
    windows: Res<Windows>,
    game_params: Res<GameParametersResource>,
    mob_query: Query<(Entity, &MobComponent, &RigidBodyPosition)>,
    rapier_config: Res<RapierConfiguration>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        for (mob_entity, mob_component, mob_rb_pos) in mob_query.iter() {
            if mouse_pos_to_rapier_pos(mouse_pos, window, &rapier_config)
                .distance(mob_rb_pos.position.translation.into())
                < game_params.scan_range
            {
                println!(
                    "Entity: {:?}\t Health: {}",
                    mob_entity,
                    format!(
                        "{}/{}",
                        mob_component.health.get_health(),
                        mob_component.health.get_max_health()
                    )
                );
                return;
            }
        }
    }
}

fn mouse_pos_to_rapier_pos(
    mouse_pos: Vec2,
    window: &Window,
    rapier_config: &RapierConfiguration,
) -> Vec2 {
    Vec2::new(
        (mouse_pos.x - (window.width() / 2.0)) / rapier_config.scale,
        (mouse_pos.y - (window.height() / 2.0)) / rapier_config.scale,
    )
}
