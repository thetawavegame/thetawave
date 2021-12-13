use crate::{
    collision::CollisionEvent,
    spawnable::{Faction, MobComponent, PlayerComponent, SpawnableComponent},
};
use bevy::prelude::*;
use serde::Deserialize;

/// Types of behaviors that can be performed by projectiles
#[derive(Deserialize, Clone)]
pub enum ProjectileBehavior {
    ExplodeOnImpact,
}

/// Manages executing behaviors of mobs
pub fn projectile_execute_behavior_system(
    mut projectile_query: Query<(Entity, &mut SpawnableComponent, &super::ProjectileComponent)>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    mut mob_query: Query<(Entity, &mut MobComponent)>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (entity, mut spawnable_component, projectile_component) in projectile_query.iter_mut() {
        let behaviors = projectile_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                ProjectileBehavior::ExplodeOnImpact => explode_on_impact(
                    entity,
                    &mut spawnable_component,
                    &collision_events_vec,
                    &mut player_query,
                    &mut mob_query,
                ),
            }
        }
    }
}

/// Explode projectile on impact
fn explode_on_impact(
    entity: Entity,
    spawnable_component: &mut SpawnableComponent,
    collision_events: &[&CollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
    mob_query: &mut Query<(Entity, &mut MobComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            CollisionEvent::PlayerToProjectileIntersection {
                player_entity,
                projectile_entity,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && matches!(
                        projectile_faction.clone(),
                        Faction::Neutral | Faction::Enemy
                    )
                {
                    // despawn blast
                    spawnable_component.should_despawn = true;
                    // spawn explosion
                    // deal damage to player
                    for (player_entity_q, mut player_component) in player_query.iter_mut() {
                        if *player_entity == player_entity_q {
                            player_component.health.take_damage(*projectile_damage);
                        }
                    }
                    continue;
                }
            }

            CollisionEvent::MobToProjectileIntersection {
                mob_entity,
                projectile_entity,
                mob_faction,
                projectile_faction,
                projectile_damage,
            } => {
                if entity == *projectile_entity
                    && !match mob_faction {
                        Faction::Ally => matches!(projectile_faction, Faction::Ally),
                        Faction::Enemy => matches!(projectile_faction, Faction::Enemy),
                        Faction::Neutral => matches!(projectile_faction, Faction::Neutral),
                    }
                {
                    // despawn blast
                    spawnable_component.should_despawn = true;
                    // spawn explosion
                    // deal damage to mob
                    for (mob_entity_q, mut mob_component) in mob_query.iter_mut() {
                        if *mob_entity == mob_entity_q {
                            mob_component.health.take_damage(*projectile_damage);
                        }
                    }
                    continue;
                }
            }
            _ => {}
        }
    }
}
