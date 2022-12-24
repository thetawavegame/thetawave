use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use serde::Deserialize;

use crate::{
    assets::GameAudioAssets,
    audio,
    collision::SortedCollisionEvent,
    loot::LootDropsResource,
    player::PlayerComponent,
    spawnable::{behavior, EffectType, SpawnConsumableEvent, SpawnEffectEvent, SpawnableComponent},
};

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum MobSegmentBehavior {
    DealDamageToPlayerOnImpact,
    ReceiveDamageOnImpact,
    DieAtZeroHealth,
}

pub fn mob_segment_execute_behavior_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut mob_segment_query: Query<(
        Entity,
        &mut SpawnableComponent,
        &mut super::MobSegmentComponent,
        &Transform,
    )>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    loot_drops_resource: Res<LootDropsResource>,
    mut spawn_consumable_event_writer: EventWriter<SpawnConsumableEvent>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (entity, mut spawnable_component, mut mob_segment_component, mob_segment_transform) in
        mob_segment_query.iter_mut()
    {
        let behaviors = mob_segment_component.behaviors.clone();
        for behavior in behaviors {
            match behavior {
                MobSegmentBehavior::DealDamageToPlayerOnImpact => {
                    deal_damage_to_player_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut player_query,
                    );
                }
                MobSegmentBehavior::ReceiveDamageOnImpact => {
                    receive_damage_on_impact(
                        entity,
                        &collision_events_vec,
                        &mut mob_segment_component,
                        &mut player_query,
                    );
                }
                MobSegmentBehavior::DieAtZeroHealth => {
                    if mob_segment_component.health.is_dead() {
                        audio_channel.play(audio_assets.mob_explosion.clone());

                        // spawn mob explosion
                        spawn_effect_event_writer.send(SpawnEffectEvent {
                            effect_type: EffectType::MobExplosion,
                            position: mob_segment_transform.translation.xy(),
                            scale: Vec2::ZERO,
                            rotation: 0.0,
                        });

                        // drop loot
                        loot_drops_resource.roll_and_spawn_consumables(
                            &mob_segment_component.consumable_drops,
                            &mut spawn_consumable_event_writer,
                            mob_segment_transform.translation.xy(),
                        );

                        // despawn mob
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
        }
    }
}

/// Deal damage to colliding entity on impact
fn deal_damage_to_player_on_impact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        if let SortedCollisionEvent::PlayerToMobSegmentContact {
            player_entity,
            mob_segment_entity,
            mob_segment_faction: _,
            player_damage: _,
            mob_segment_damage,
        } = collision_event
        {
            if entity == *mob_segment_entity {
                // deal damage to player
                for (player_entity_q, mut player_component) in player_query.iter_mut() {
                    if player_entity_q == *player_entity {
                        player_component.health.take_damage(*mob_segment_damage);
                    }
                }
            }
        }
    }
}

/// Take damage from colliding entity on impact
fn receive_damage_on_impact(
    entity: Entity,
    collision_events: &[&SortedCollisionEvent],
    mob_segment_component: &mut super::MobSegmentComponent,
    player_query: &mut Query<(Entity, &mut PlayerComponent)>,
) {
    for collision_event in collision_events.iter() {
        match collision_event {
            SortedCollisionEvent::PlayerToMobSegmentContact {
                player_entity,
                mob_segment_entity,
                mob_segment_faction: _,
                player_damage,
                mob_segment_damage: _,
            } => {
                if entity == *mob_segment_entity {
                    for (player_entity_q, mut _player_component) in player_query.iter_mut() {
                        if player_entity_q == *player_entity {
                            mob_segment_component.health.take_damage(*player_damage);
                        }
                    }
                }
            }
            SortedCollisionEvent::MobToMobSegmentContact {
                mob_segment_entity,
                mob_segment_faction: _,
                mob_segment_damage: _,
                mob_entity: _,
                mob_faction: _,
                mob_damage,
            } => {
                if entity == *mob_segment_entity {
                    mob_segment_component.health.take_damage(*mob_damage);
                }
            }
            SortedCollisionEvent::MobSegmentToMobSegmentContact {
                mob_segment_entity_1,
                mob_segment_faction_1: _,
                mob_segment_damage_1: _,
                mob_segment_entity_2: _,
                mob_segment_faction_2: _,
                mob_segment_damage_2,
            } => {
                if entity == *mob_segment_entity_1 {
                    mob_segment_component
                        .health
                        .take_damage(*mob_segment_damage_2);
                }
            }

            _ => {}
        }
    }
}
