use bevy::prelude::*;
use serde::Deserialize;

/// Describes how to change frames of animation
#[derive(Deserialize, Clone)]
pub enum AnimationDirection {
    None,
    Forward,
    PingPong(PingPongDirection),
}

/// Current direction of a pingping animation
#[derive(Deserialize, Clone)]
pub enum PingPongDirection {
    Forward,
    Backward,
}

/// Data describing texture
#[derive(Deserialize)]
pub struct TextureData {
    /// Path to the texture
    pub path: String,
    /// Dimensions of the texture (single frame)
    pub dimensions: Vec2,
    /// Columns in the spritesheet
    pub cols: usize,
    /// Rows in the spritesheet
    pub rows: usize,
    /// Duration of a frame of animation
    pub frame_duration: f32,
    /// How the animation switches frames
    pub animation_direction: AnimationDirection,
}

/// Component for managing animation
#[derive(Component)]
pub struct AnimationComponent {
    /// Timer to track frame duration,
    pub timer: Timer,
    /// Direction of the animation
    pub direction: AnimationDirection,
}

/// Handles animation of sprites
pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationComponent,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut animation, mut sprite, texture_atlas_handle) in query.iter_mut() {
        // tick the animation timer
        animation.timer.tick(time.delta());

        // check if frame has completed
        if animation.timer.finished() {
            // get the texture atlas
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            // update animation based on the animation direction
            match &animation.direction {
                AnimationDirection::None => {}
                AnimationDirection::Forward => {
                    sprite.index = (sprite.index as usize + 1) % texture_atlas.textures.len()
                }
                AnimationDirection::PingPong(direction) => match direction {
                    PingPongDirection::Forward => {
                        if sprite.index < (texture_atlas.textures.len() - 1) {
                            sprite.index += 1;
                        }

                        if sprite.index == (texture_atlas.textures.len() - 1) {
                            animation.direction =
                                AnimationDirection::PingPong(PingPongDirection::Backward)
                        }
                    }
                    PingPongDirection::Backward => {
                        if sprite.index > 0 {
                            sprite.index -= 1;
                        }

                        if sprite.index == 0 {
                            animation.direction =
                                AnimationDirection::PingPong(PingPongDirection::Forward)
                        }
                    }
                },
            };
        }
    }
}
