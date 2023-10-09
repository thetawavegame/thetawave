use bevy::prelude::*;
use thetawave_interface::spawnable::ItemType;

use self::behavior::ItemBehavior;

mod behavior;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {}
}

pub struct ItemComponent {
    pub item_type: ItemType,
    pub item_effects: Vec<ItemEffect>,
    pub behaviors: Vec<ItemBehavior>,
}

pub enum ItemEffect {
    GainDamage(usize),
    GainHealth(usize),
    GainFireRate(f32),
}
