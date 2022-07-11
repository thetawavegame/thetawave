use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{game::GameParametersResource, spawnable::MobComponent};

/// Manages scanning of entities using the cursor
pub fn scanner_system(
    windows: Res<Windows>,
    game_params: Res<GameParametersResource>,
    mob_query: Query<(Entity, &MobComponent, &Transform)>,
) {
    let window = windows.get_primary().unwrap();

    if let Some(mouse_pos) = window.cursor_position() {
        for (mob_entity, mob_component, transform) in mob_query.iter() {
            if mouse_pos_to_rapier_pos(mouse_pos, window).distance(transform.translation.xy())
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

/// Converts mouse position units to in-game physics units
fn mouse_pos_to_rapier_pos(mouse_pos: Vec2, window: &Window) -> Vec2 {
    Vec2::new(
        mouse_pos.x - (window.width() / 2.0),
        mouse_pos.y - (window.height() / 2.0),
    )
}
