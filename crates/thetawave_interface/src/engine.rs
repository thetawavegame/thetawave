use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }
}

impl From<&Vec2> for bevy_math::Vec2 {
    fn from(val: &Vec2) -> Self {
        bevy_math::Vec2 { x: val.x, y: val.y }
    }
}
