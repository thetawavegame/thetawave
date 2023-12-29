use bevy::{app::{Plugin, App, Update}, ecs::{schedule::{common_conditions::in_state, IntoSystemConfigs}, component::Component, system::{Res, Query}, entity::Entity, event::EventWriter}, math::Vec2, time::{Timer, Time}, asset::{Assets, Handle}, sprite::{TextureAtlas, TextureAtlasSprite}};
use serde::Deserialize;
use thetawave_interface::{states, animation::AnimationCompletedEvent};

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_sprite_system
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );

        app.add_event::<AnimationCompletedEvent>();
    }
}

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

/// Describes an animation
#[derive(Deserialize)]
pub struct AnimationData {
    pub direction: AnimationDirection,
    pub frame_duration: f32,
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
    mut animation_complete_event_writer: EventWriter<AnimationCompletedEvent>,
    mut query: Query<(
        Entity,
        &mut AnimationComponent,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (entity, mut animation, mut sprite, texture_atlas_handle) in query.iter_mut() {
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
                    let new_idx = (sprite.index + 1) % texture_atlas.textures.len();
                    if new_idx == 0 {
                        animation_complete_event_writer.send(AnimationCompletedEvent(entity))
                    }
                    sprite.index = new_idx;
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
