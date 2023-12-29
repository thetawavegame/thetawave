use bevy_ecs::{entity::Entity, event::Event};

#[derive(Event)]
pub struct AnimationCompletedEvent(pub Entity);
