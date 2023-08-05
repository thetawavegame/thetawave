use bevy::{math::Vec3Swizzles, prelude::*, window::PrimaryWindow};

use crate::{game::GameParametersResource, spawnable::MobComponent, states};

pub struct ScannerPlugin;

impl Plugin for ScannerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            scanner_system.run_if(in_state(states::AppStates::Game)),
        );
    }
}

/// Manages scanning of entities using the cursor
pub fn scanner_system(
    //windows: Res<Windows>,
    windows: Query<&Window, With<PrimaryWindow>>,
    game_params: Res<GameParametersResource>,
    mob_query: Query<(Entity, &MobComponent, &Transform)>,
) {
    // get the primary window
    let primary_window = windows.get_single().unwrap();

    // get the cursor position in the window
    if let Some(mouse_pos) = primary_window.cursor_position() {
        // query the mobs
        for (mob_entity, mob_component, transform) in mob_query.iter() {
            // check if the mob is in scanning range of the mouse
            if mouse_pos_to_rapier_pos(mouse_pos, primary_window)
                .distance(transform.translation.xy())
                < game_params.scan_range
            {
                debug!(
                    "Mob near mouse: Entity: {:?}\t Health: {}/{}",
                    mob_entity,
                    mob_component.health.get_health(),
                    mob_component.health.get_max_health()
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
