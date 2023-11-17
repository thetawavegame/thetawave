use super::{
    mob::BehaviorSequenceTracker, MobBehavior, MobComponent, MobSegmentComponent,
    MobSegmentControlBehavior, SpawnableBehavior, SpawnableComponent,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, time::Duration};

#[derive(Deserialize, Clone)]
pub struct MobBehaviorSequence {
    pub behaviors: Vec<MobBehaviorSequenceElement>,
}

#[derive(Deserialize, Clone)]
pub struct MobBehaviorSequenceElement {
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    pub mob_behaviors: Vec<MobBehavior>,
    pub control_behaviors: Vec<MobSegmentControlBehavior>,
    pub time: f32,
}

#[derive(Deserialize, Resource)]
pub struct BehaviorSequenceResource {
    pub sequences: HashMap<MobBehaviorSequenceType, MobBehaviorSequence>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MobBehaviorSequenceType {
    Repeater,
}

pub fn mob_behavior_sequence_tracker_system(
    behavior_sequence_resource: Res<BehaviorSequenceResource>,
    time: Res<Time>,
    mut behavior_update_event_writer: EventWriter<MobBehaviorUpdateEvent>,
    mut mob_query: Query<(Entity, &mut MobComponent)>,
) {
    for (entity, mut mob_component) in mob_query.iter_mut() {
        if let Some(behavior_sequence_type) = mob_component.behavior_sequence.clone() {
            // get behavior sequence from resource
            let behavior_sequence =
                behavior_sequence_resource.sequences[&behavior_sequence_type].clone();
            if let Some(behavior_sequence_tracker) = &mut mob_component.behavior_sequence_tracker {
                // tick the timer
                behavior_sequence_tracker.timer.tick(time.delta());

                // if timer just finished update the behaviors
                if behavior_sequence_tracker.timer.just_finished() {
                    // update the index
                    behavior_sequence_tracker.index = if behavior_sequence_tracker.index
                        == behavior_sequence.behaviors.len() - 1
                    {
                        0
                    } else {
                        behavior_sequence_tracker.index + 1
                    };

                    // reset timer
                    behavior_sequence_tracker
                        .timer
                        .set_duration(Duration::from_secs_f32(
                            behavior_sequence.behaviors[behavior_sequence_tracker.index].time,
                        ));
                    behavior_sequence_tracker.timer.reset();

                    // update behaviors
                    behavior_update_event_writer.send(MobBehaviorUpdateEvent {
                        mob_behaviors: behavior_sequence.behaviors[behavior_sequence_tracker.index]
                            .mob_behaviors
                            .clone(),
                        control_behaviors: behavior_sequence.behaviors
                            [behavior_sequence_tracker.index]
                            .control_behaviors
                            .clone(),
                        spawnable_behaviors: behavior_sequence.behaviors
                            [behavior_sequence_tracker.index]
                            .spawnable_behaviors
                            .clone(),
                        entity,
                    });
                }
            } else {
                // initialize behavior sequence tracker
                mob_component.behavior_sequence_tracker = Some(BehaviorSequenceTracker {
                    timer: Timer::from_seconds(
                        behavior_sequence.behaviors[0].time,
                        TimerMode::Once,
                    ),
                    index: 0,
                });

                behavior_update_event_writer.send(MobBehaviorUpdateEvent {
                    mob_behaviors: behavior_sequence.behaviors[0].mob_behaviors.clone(),
                    control_behaviors: behavior_sequence.behaviors[0].control_behaviors.clone(),
                    spawnable_behaviors: behavior_sequence.behaviors[0].spawnable_behaviors.clone(),
                    entity,
                });
            }
        }
    }
}

#[derive(Event)]
pub struct MobBehaviorUpdateEvent {
    pub mob_behaviors: Vec<MobBehavior>,
    pub control_behaviors: Vec<MobSegmentControlBehavior>,
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    pub entity: Entity,
}

pub fn mob_behavior_sequence_update_system(
    mut behavior_update_event_reader: EventReader<MobBehaviorUpdateEvent>,
    mut mob_query: Query<(Entity, &mut MobComponent, &mut SpawnableComponent)>,
    mut mob_segment_query: Query<(Entity, &mut MobSegmentComponent, &ImpulseJoint)>,
) {
    for event in behavior_update_event_reader.read() {
        for (entity, mut mob_component, mut spawnable_component) in mob_query.iter_mut() {
            if entity == event.entity {
                mob_component.behaviors = event.mob_behaviors.clone();
                mob_component.control_behaviors = event.control_behaviors.clone();
                spawnable_component.behaviors = event.spawnable_behaviors.clone();

                let mut entity_pairs = vec![];
                // set behaviors of attached mob segments

                // get all entity pairs
                for (mob_segment_entity, _, joint) in mob_segment_query.iter_mut() {
                    entity_pairs.push(EntityPair {
                        parent: joint.parent,
                        entity: mob_segment_entity,
                    });
                }

                // collected jointed mob entities
                let mut mob_segment_entities: Vec<Entity> = vec![];
                loop {
                    let mut remove_entities = vec![];

                    for pair in entity_pairs.iter_mut() {
                        // add entities to mob segment entities if they are the mob, or their parent is in the vector aleady
                        if pair.parent == entity
                            || mob_segment_entities
                                .iter()
                                .any(|mob_segment_entity| *mob_segment_entity == pair.parent)
                        {
                            mob_segment_entities.push(pair.entity);
                            remove_entities.push(pair.entity);
                        }
                    }

                    if remove_entities.is_empty() {
                        break;
                    }

                    entity_pairs.retain(|entity_pair| {
                        !remove_entities
                            .iter()
                            .any(|remove_entity| *remove_entity == entity_pair.entity)
                    });
                }

                // add mob segment behaviors to mob segment from mob component based on mob joint behaviors
                for (mob_segment_entity, mut mob_segment_component, _) in
                    mob_segment_query.iter_mut()
                {
                    if mob_segment_entities
                        .iter()
                        .any(|check_entity| *check_entity == mob_segment_entity)
                    {
                        mob_segment_component.behaviors = vec![];
                        for control_behavior in event.control_behaviors.iter() {
                            if let Some(mob_segment_behaviors_map) =
                                mob_component.mob_segment_behaviors.clone()
                            {
                                if let Some(all_mob_segment_behaviors) =
                                    mob_segment_behaviors_map.get(control_behavior)
                                {
                                    if let Some(mob_segment_behaviors) = all_mob_segment_behaviors
                                        .get(&mob_segment_component.mob_segment_type)
                                    {
                                        mob_segment_component
                                            .behaviors
                                            .append(&mut mob_segment_behaviors.clone());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub struct EntityPair {
    pub parent: Entity,
    pub entity: Entity,
}
