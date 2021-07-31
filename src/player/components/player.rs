use bevy::prelude::*;

/// Component for managing core attributes of the player
pub struct PlayerComponent {
    pub acceleration: Vec2,
    pub deceleration: Vec2,
    pub speed: Vec2, // Amount of money the player has
                     //pub money: i32,
                     // Amount of collision damage the player deals
                     //pub collision_damage: f32,
                     // All the items the player has collected
                     //pub items: Vec<ItemType>
}
