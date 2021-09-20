use bevy::prelude::*;
use serde::Deserialize;

/// Describes how to change frames of animation
#[derive(Deserialize, Clone)]
pub enum AnimationDirection {
    Forward,
    PingPong(PingPongDirection),
}

/// Current direction of a pingping animation
#[derive(Deserialize, Clone)]
pub enum PingPongDirection {
    Forward,
    Backward,
}

/// Component for managing animation
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
        animation.timer.tick(time.delta());
        if animation.timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();

            match &animation.direction {
                AnimationDirection::Forward => {
                    sprite.index =
                        ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32
                }
                AnimationDirection::PingPong(direction) => match direction {
                    PingPongDirection::Forward => {
                        if sprite.index < (texture_atlas.textures.len() - 1) as u32 {
                            sprite.index += 1;
                        }

                        if sprite.index == (texture_atlas.textures.len() - 1) as u32 {
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
