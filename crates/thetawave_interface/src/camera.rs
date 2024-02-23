use bevy_ecs::event::Event;

#[derive(Event)]
pub struct ScreenShakeEvent {
    pub trauma: f32,
}
