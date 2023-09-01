use bevy_ecs::prelude::Event;
use serde::Deserialize;

// Event for sending damage dealt from mob reaching bottom of arena
#[derive(Event)]
pub struct MobReachedBottomGateEvent(pub DefenseInteraction);

#[derive(Deserialize, Clone)]
pub enum DefenseInteraction {
    Heal(usize),
    Damage(usize),
}
