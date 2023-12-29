use bevy_ecs::{entity::Entity, event::Event};

/// This event is used for notifying systems when an animation for an entity has been completed
/// Can be used for despawning entities after animations finish
#[derive(Event)]
pub struct AnimationCompletedEvent(pub Entity);
