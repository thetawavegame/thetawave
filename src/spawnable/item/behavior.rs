use bevy::prelude::{
    in_state, info, App, Commands, Component, EventReader, IntoSystemConfigs, Plugin, Query,
    Update, With,
};
use serde::Deserialize;
use thetawave_interface::{
    health::HealthComponent, player::PlayerComponent, spawnable::ItemComponent, states,
};

use crate::collision::SortedCollisionEvent;

pub struct ItemBehaviorPlugin;

impl Plugin for ItemBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            // We want to full heal __after__ increasing the max health to get up to a full health bar
            (
                on_collect_increase_max_health_system,
                on_collect_full_heal_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing))
                .chain(),
        );
    }
}

#[derive(Deserialize, Clone)]
pub enum ItemBehavior {
    OnCollectIncreaseMaxHealth(usize),
    OnCollectFullHeal,
    AttractToPlayer,
}

#[derive(Component)]
pub struct OnCollectIncreaseMaxHealth(pub usize);

#[derive(Component)]
pub struct OnCollectFullHeal;

#[derive(Component)]
pub struct AttractToPlayer;

pub fn on_collect_increase_max_health_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    item_query: Query<&OnCollectIncreaseMaxHealth, With<ItemComponent>>,
    mut player_query: Query<&mut HealthComponent, With<PlayerComponent>>,
) {
    for event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToItemIntersection {
            player_entity,
            item_entity,
        } = event
        {
            if let Ok(health_increase_component) = item_query.get(*item_entity) {
                if let Ok(mut health_component) = player_query.get_mut(*player_entity) {
                    health_component.increase_max_health(health_increase_component.0);
                    info!("Max health increased by {}", health_increase_component.0);
                    commands.entity(*item_entity).despawn();
                }
            }
        }
    }
}

pub fn on_collect_full_heal_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    item_query: Query<&OnCollectFullHeal, With<ItemComponent>>,
    mut player_query: Query<&mut HealthComponent, With<PlayerComponent>>,
) {
    for event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToItemIntersection {
            player_entity,
            item_entity,
        } = event
        {
            if item_query.get(*item_entity).is_ok() {
                if let Ok(mut health_component) = player_query.get_mut(*player_entity) {
                    health_component.full_heal();
                    info!("Fully healed player");
                    commands.entity(*item_entity).despawn();
                }
            }
        }
    }
}
