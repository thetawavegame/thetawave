use bevy_ecs::event::Event;

/// An event that triggers the user's screen/camera to shake, as if the player's ship was just hit.
#[derive(Event)]
pub struct ScreenShakeEvent {
    /// This should be between 0 and 1. 0 is no screen shake; 1 is a very aggressive shake.
    pub trauma: f32,
}
