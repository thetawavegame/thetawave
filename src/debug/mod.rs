//! `thetawave` debug module
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

/// Draw lines over colliders
pub fn collider_debug_lines_system(
    mut lines: ResMut<DebugLines>,
    player_colliders: Query<(&ColliderPositionComponent, &ColliderShapeComponent)>,
    rapier_config: Res<RapierConfiguration>,
) {
    // draw colliders
    for (collider_position, collider_shape) in player_colliders.iter() {
        let collider_cuboid = collider_shape.as_cuboid().unwrap();
        let collider_translation = collider_position.translation;

        draw_debug_collider_cuboid(
            &mut lines,
            collider_cuboid,
            collider_translation,
            rapier_config.scale,
        );
    }
}

/// Draw colliders over cuboids
fn draw_debug_collider_cuboid(
    debug_lines: &mut ResMut<DebugLines>,
    cuboid: &Cuboid,
    translation: Translation<f32>,
    scale_multiplier: f32,
) {
    let half_extents = cuboid.half_extents;

    // draw right side
    let start = Vec3::new(
        (translation.x + half_extents.x) * scale_multiplier,
        (translation.y + half_extents.y) * scale_multiplier,
        0.0,
    );

    let end = Vec3::new(
        (translation.x + half_extents.x) * scale_multiplier,
        (translation.y - half_extents.y) * scale_multiplier,
        0.0,
    );

    debug_lines.line_colored(start, end, 0.0, Color::GREEN);

    // draw left side
    let start = Vec3::new(
        (translation.x - half_extents.x) * scale_multiplier,
        (translation.y - half_extents.y) * scale_multiplier,
        0.0,
    );

    let end = Vec3::new(
        (translation.x - half_extents.x) * scale_multiplier,
        (translation.y + half_extents.y) * scale_multiplier,
        0.0,
    );

    debug_lines.line_colored(start, end, 0.0, Color::GREEN);

    // draw top side
    let start = Vec3::new(
        (translation.x + half_extents.x) * scale_multiplier,
        (translation.y + half_extents.y) * scale_multiplier,
        0.0,
    );

    let end = Vec3::new(
        (translation.x - half_extents.x) * scale_multiplier,
        (translation.y + half_extents.y) * scale_multiplier,
        0.0,
    );

    debug_lines.line_colored(start, end, 0.0, Color::GREEN);

    // draw bottom side
    let start = Vec3::new(
        (translation.x + half_extents.x) * scale_multiplier,
        (translation.y - half_extents.y) * scale_multiplier,
        0.0,
    );

    let end = Vec3::new(
        (translation.x - half_extents.x) * scale_multiplier,
        (translation.y - half_extents.y) * scale_multiplier,
        0.0,
    );

    debug_lines.line_colored(start, end, 0.0, Color::GREEN);
}
