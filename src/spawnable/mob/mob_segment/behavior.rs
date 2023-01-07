use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_rapier2d::{prelude::*, rapier::prelude::JointAxis};
use rand::{thread_rng, Rng};
use serde::Deserialize;

use crate::{
    assets::GameAudioAssets,
    audio,
    collision::SortedCollisionEvent,
    loot::LootDropsResource,
    player::PlayerComponent,
    spawnable::{
        behavior_sequence::EntityPair, mob::behavior::SpawnMobBehaviorData, EffectType,
        MobDestroyedEvent, SpawnConsumableEvent, SpawnEffectEvent, SpawnMobEvent, SpawnPosition,
    },
};

use super::{MobSegmentComponent, MobSegmentsResource};

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum MobSegmentBehavior {
    DealDamageToPlayerOnImpact,
    ReceiveDamageOnImpact,
    DieAtZeroHealth,
    RandomRotation(RandomRotationData),
    RepeaterProtectHead(RepeaterSegmentProtectHeadData), // takes in angle to protect head
    RepeaterAttack(RepeaterSegmentAttackData),
    SpawnMob(String),
}

#[derive(Deserialize, Clone)]
pub struct RepeaterSegmentProtectHeadData {
    pub angle: f32,
    pub damping: f32,
    pub stiffness: f32,
}

#[derive(Deserialize, Clone)]
pub struct RepeaterSegmentAttackData {
    pub angle: f32,
    pub damping: f32,
    pub stiffness: f32,
}

#[derive(Deserialize, Clone)]
pub struct RandomRotationData {
    pub low_angle: f32,
    pub high_angle: f32,
    pub damping: f32,
    pub stiffness: f32,
}

pub fn mob_segment_execute_behavior_system(
    mut commands: Commands,
    mut collision_events: EventReader<SortedCollisionEvent>,
    mut mob_segment_query: Query<(
        Entity,
        &mut MobSegmentComponent,
        &Transform,
        &mut ImpulseJoint,
    )>,
    mut spawn_effect_event_writer: EventWriter<SpawnEffectEvent>,
    mut player_query: Query<(Entity, &mut PlayerComponent)>,
    loot_drops_resource: Res<LootDropsResource>,
    mut spawn_consumable_event_writer: EventWriter<SpawnConsumableEvent>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
    time: Res<Time>,
    mut spawn_mob_event_writer: EventWriter<SpawnMobEvent>,
    mut mob_segment_destroyed_event_writer: EventWriter<MobSegmentDestroyedEvent>,
) {
    let mut collision_events_vec = vec![];
    for collision_event in collision_events.iter() {
        collision_events_vec.push(collision_event);
    }

    for (entity, mut mob_segment_component, mob_segment_transform, mut joint) in
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

                        mob_segment_destroyed_event_writer
                            .send(MobSegmentDestroyedEvent { entity });
                    }
                }
                MobSegmentBehavior::RandomRotation(data) => {
                    let rand_ang = thread_rng().gen_range(data.low_angle..=data.high_angle);

                    joint.data.set_motor_position(
                        JointAxis::AngX,
                        rand_ang,
                        data.stiffness,
                        data.damping,
                    );
                }

                MobSegmentBehavior::RepeaterProtectHead(data) => {
                    joint.data.set_motor_position(
                        JointAxis::AngX,
                        data.angle,
                        data.stiffness,
                        data.damping,
                    );
                }

                MobSegmentBehavior::RepeaterAttack(data) => {
                    joint.data.set_motor_position(
                        JointAxis::AngX,
                        data.angle,
                        data.stiffness,
                        data.damping,
                    );
                }

                MobSegmentBehavior::SpawnMob(mob_spawner_key) => {
                    // if mob component does not have a timer initialize timer
                    // otherwise tick timer and spawn mob on completion
                    let mob_spawners = mob_segment_component
                        .mob_spawners
                        .get_mut(&mob_spawner_key)
                        .unwrap();

                    for mob_spawner in mob_spawners.iter_mut() {
                        mob_spawner.timer.tick(time.delta());

                        if mob_spawner.timer.just_finished() {
                            // spawn mob
                            let position = match mob_spawner.position {
                                SpawnPosition::Global(coords) => coords,
                                SpawnPosition::Local(coords) => {
                                    mob_segment_transform.translation.xy()
                                        + mob_segment_transform.local_x().xy() * coords.x
                                        + mob_segment_transform.local_y().xy() * coords.y
                                }
                            };

                            spawn_mob_event_writer.send(SpawnMobEvent {
                                mob_type: mob_spawner.mob_type.clone(),
                                position,
                                rotation: mob_segment_transform.rotation, // passed rotation of the parent mob
                            })
                        }
                    }
                }
            }
        }
    }
}

pub struct MobSegmentDestroyedEvent {
    pub entity: Entity,
}

pub fn mob_segment_apply_disconnected_behaviors_system(
    mut mob_destroyed_event_reader: EventReader<MobDestroyedEvent>,
    mut mob_segment_destroyed_event_reader: EventReader<MobSegmentDestroyedEvent>,
    mut mob_segment_query: Query<(Entity, &mut MobSegmentComponent, &ImpulseJoint)>,
    mob_segments_resource: Res<MobSegmentsResource>,
) {
    let mut entities: Vec<Entity> = mob_destroyed_event_reader
        .iter()
        .map(|event| event.entity)
        .collect();

    let mut mob_segment_entities: Vec<Entity> = mob_segment_destroyed_event_reader
        .iter()
        .map(|event| event.entity)
        .collect();

    entities.append(&mut mob_segment_entities);

    for entity in entities.iter() {
        let mut entity_pairs = vec![];
        // find all mob segments attached to mob entity from event
        for (mob_segment_entity, _, joint) in mob_segment_query.iter_mut() {
            entity_pairs.push(EntityPair {
                parent: joint.parent,
                entity: mob_segment_entity,
            });
        }

        // collected joint mob entities
        let mut mob_segment_entities: Vec<Entity> = vec![];
        while true {
            let mut remove_entities = vec![];

            for pair in entity_pairs.iter_mut() {
                // add entities to mob segment entities if they are the mob, or their parent is in the vector aleady
                if pair.parent == *entity
                    || mob_segment_entities
                        .iter()
                        .any(|mob_segment_entity| *mob_segment_entity == pair.parent)
                {
                    mob_segment_entities.push(pair.entity);
                    remove_entities.push(pair.entity);
                }
            }

            if remove_entities.len() == 0 {
                break;
            }

            entity_pairs.retain(|entity_pair| {
                !remove_entities
                    .iter()
                    .any(|remove_entity| *remove_entity == entity_pair.entity)
            });
        }

        for (mob_segment_entity, mut mob_segment_comoponent, _) in mob_segment_query.iter_mut() {
            if mob_segment_entities
                .iter()
                .any(|check_entity| *check_entity == mob_segment_entity)
            {
                if let Some(disconnected_behaviors) = mob_segments_resource.mob_segments
                    [&mob_segment_comoponent.mob_segment_type]
                    .disconnected_behaviors
                    .clone()
                {
                    mob_segment_comoponent.behaviors = disconnected_behaviors.clone();
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
