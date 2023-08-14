use bevy_ecs::prelude::Entity;
use bevy_ecs_macros::Event;

#[derive(Event)]
pub struct DamageDealtEvent {
    pub damage: f32,
    pub target: Entity,
}
