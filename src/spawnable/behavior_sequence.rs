use super::{
    mob::BehaviorSequenceTracker, MobBehavior, MobComponent, SpawnableBehavior, SpawnableComponent,
};
use bevy::prelude::*;
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
    mut behavior_sequence_resource: Res<BehaviorSequenceResource>,
    time: Res<Time>,
    mut behavior_update_event_writer: EventWriter<MobBehaviorUpdateEvent>,
    mut mob_query: Query<(Entity, &mut MobComponent, &mut SpawnableComponent)>,
) {
    for (entity, mut mob_component, mut spawnable_component) in mob_query.iter_mut() {
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
                    spawnable_behaviors: behavior_sequence.behaviors[0].spawnable_behaviors.clone(),
                    entity,
                });
            }
        }
    }
}

pub struct MobBehaviorUpdateEvent {
    pub mob_behaviors: Vec<MobBehavior>,
    pub spawnable_behaviors: Vec<SpawnableBehavior>,
    pub entity: Entity,
}

pub fn mob_behavior_sequence_update_system(
    mut behavior_update_event_reader: EventReader<MobBehaviorUpdateEvent>,
    mut mob_query: Query<(Entity, &mut MobComponent, &mut SpawnableComponent)>,
) {
    for event in behavior_update_event_reader.iter() {
        for (entity, mut mob_component, mut spawnable_component) in mob_query.iter_mut() {
            if entity == event.entity {
                mob_component.behaviors = event.mob_behaviors.clone();
                spawnable_component.behaviors = event.spawnable_behaviors.clone();
            }
        }
    }
}
