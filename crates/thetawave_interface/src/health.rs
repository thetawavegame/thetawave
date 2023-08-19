use bevy_ecs::prelude::Entity;
use bevy_ecs_macros::Event;

#[derive(Event)]
pub struct DamageDealtEvent {
    pub damage: usize,
    pub target: Entity,
}
