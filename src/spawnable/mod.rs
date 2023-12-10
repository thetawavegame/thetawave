use crate::spawnable::effect::EffectPlugin;
use std::collections::HashMap;

use crate::GameUpdateSet;
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;
use rand::{thread_rng, Rng};
use ron::de::from_bytes;
use serde::Deserialize;
use thetawave_interface::spawnable::{ConsumableType, MobType, ProjectileType};
use thetawave_interface::spawnable::{
    MobDestroyedEvent, MobSegmentDestroyedEvent, SpawnMobEvent, SpawnableType,
};
use thetawave_interface::states;

mod behavior;
mod behavior_sequence;
mod consumable;
mod effect;
mod item;
mod mob;
mod projectile;

use self::behavior::attract_to_player_system;
use self::item::ItemPlugin;
pub use self::mob::*;
pub use self::projectile::{
    projectile_execute_behavior_system, spawn_projectile_system, ProjectileComponent,
    ProjectileData, ProjectileResource, SpawnProjectileEvent,
};

pub use self::behavior::{
    spawnable_execute_behavior_system, spawnable_set_target_behavior_system, SpawnableBehavior,
};

pub use self::behavior_sequence::{
    mob_behavior_sequence_tracker_system, mob_behavior_sequence_update_system,
    BehaviorSequenceResource, MobBehaviorUpdateEvent,
};

pub use self::effect::{EffectsResource, SpawnEffectEvent};

pub use self::consumable::{
    consumable_execute_behavior_system, spawn_consumable_system, ConsumableComponent,
    ConsumableData, ConsumableResource, SpawnConsumableEvent,
};

pub struct SpawnablePlugin;

impl Plugin for SpawnablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<BehaviorSequenceResource>(include_bytes!(
                "../../assets/data/behavior_sequences.ron"
            ))
            .expect("Failed to parse BehaviorSequenceResource from 'behavior_sequences.ron'"),
        )
        .insert_resource(MobsResource {
            mobs: from_bytes::<HashMap<MobType, MobData>>(include_bytes!(
                "../../assets/data/mobs.ron"
            ))
            .expect("Failed to parse MobsResource from 'mobs.ron'"),
            texture_atlas_handle: HashMap::new(),
        })
        .insert_resource(
            from_bytes::<MobSegmentsResource>(include_bytes!("../../assets/data/mob_segments.ron"))
                .expect("Failed to parse MobSegmentsResource from 'mob_segments.ron'"),
        )
        .insert_resource(ProjectileResource {
            projectiles: from_bytes::<HashMap<ProjectileType, ProjectileData>>(include_bytes!(
                "../../assets/data/projectiles.ron"
            ))
            .expect("Failed to parse ProjectileResource from 'projectiles.ron'"),
        })
        .insert_resource(ConsumableResource {
            consumables: from_bytes::<HashMap<ConsumableType, ConsumableData>>(include_bytes!(
                "../../assets/data/consumables.ron"
            ))
            .expect("Failed to parse ConsumableResource from 'consumables.ron'"),
        });

        app.add_event::<SpawnConsumableEvent>()
            .add_event::<SpawnProjectileEvent>()
            .add_event::<SpawnMobEvent>()
            .add_event::<MobBehaviorUpdateEvent>()
            .add_event::<MobDestroyedEvent>()
            .add_event::<MobSegmentDestroyedEvent>()
            .add_event::<BossesDestroyedEvent>();

        app.add_plugins((EffectPlugin, ItemPlugin));

        app.add_systems(
            Update,
            (
                despawn_timer_system,
                spawnable_set_target_behavior_system.in_set(GameUpdateSet::SetTargetBehavior),
                mob_behavior_sequence_tracker_system,
                mob_behavior_sequence_update_system,
                spawnable_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                mob_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                mob_segment_apply_disconnected_behaviors_system
                    .in_set(GameUpdateSet::ApplyDisconnectedBehaviors),
                mob_segment_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                projectile_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                consumable_execute_behavior_system.in_set(GameUpdateSet::ExecuteBehavior),
                spawn_projectile_system,
                spawn_consumable_system, // event generated in mob execute behavior
                spawn_mob_system,        // event generated in mob execute behavior
                check_boss_mobs_system,
                attract_to_player_system,
            )
                .run_if(in_state(states::AppStates::Game))
                .run_if(in_state(states::GameStates::Playing)),
        );
    }
}

/// Core component of spawnable entities
#[derive(Component, Default)]
pub struct SpawnableComponent {
    /// Type of spawnable
    pub spawnable_type: SpawnableType,
    /// Acceleration stat
    pub acceleration: Vec2,
    /// Deceleration stat
    pub deceleration: Vec2,
    /// Maximum speed stat
    pub speed: Vec2,
    /// Angular acceleration stat
    pub angular_acceleration: f32,
    /// Angular deceleration stat
    pub angular_deceleration: f32,
    /// Maximum angular speed stat
    pub angular_speed: f32,
    /// List of behaviors that are performed
    pub behaviors: Vec<SpawnableBehavior>,
}

impl From<&MobData> for SpawnableComponent {
    fn from(mob_data: &MobData) -> Self {
        SpawnableComponent {
            spawnable_type: SpawnableType::Mob(mob_data.mob_type.clone()),
            acceleration: mob_data.acceleration,
            deceleration: mob_data.deceleration,
            speed: mob_data.speed,
            angular_acceleration: mob_data.angular_acceleration,
            angular_deceleration: mob_data.angular_deceleration,
            angular_speed: mob_data.angular_speed,
            behaviors: mob_data.spawnable_behaviors.clone(),
        }
    }
}

impl SpawnableComponent {
    fn new(spawnable_type: SpawnableType) -> Self {
        SpawnableComponent {
            spawnable_type,
            acceleration: Vec2::ZERO,
            deceleration: Vec2::ZERO,
            speed: Vec2::ZERO,
            angular_acceleration: 0.0,
            angular_deceleration: 0.0,
            angular_speed: 0.0,
            behaviors: vec![],
        }
    }
}

/// Initial motion that entity is spawned in with
#[derive(Deserialize, Clone, Default, Debug)]
pub struct InitialMotion {
    /// Optional angular velocity
    pub angvel: Option<f32>,
    /// Optional random range of angular velocity
    pub random_angvel: Option<(f32, f32)>,
    /// Optional linear velocity
    pub linvel: Option<Vec2>,
    /// Optional random range of linear velocity
    pub random_linvel: Option<(Vec2, Vec2)>,
}

impl From<InitialMotion> for Velocity {
    fn from(im: InitialMotion) -> Self {
        let random_linvel = if let Some((lower, upper)) = im.random_linvel {
            let x = thread_rng().gen_range(lower.x..=upper.x);
            let y = thread_rng().gen_range(lower.y..=upper.y);
            Vec2::new(x, y)
        } else {
            Vec2::ZERO
        };

        let random_angvel = if let Some((lower, upper)) = im.random_angvel {
            thread_rng().gen_range(lower..=upper)
        } else {
            0.0
        };

        Velocity {
            linvel: im.linvel.unwrap_or_default() + random_linvel,
            angvel: im.angvel.unwrap_or_default() + random_angvel,
        }
    }
}

/// Component that despawns entity after amount of time has passed

#[derive(Component)]
pub struct DespawnTimerComponent {
    despawn_timer: Timer,
}

/// Manages despawn timer components
pub fn despawn_timer_system(
    mut commands: Commands,
    time: Res<Time>,
    mut despawn_timer_query: Query<(Entity, &mut DespawnTimerComponent)>,
) {
    for (entity, mut despawn_timer) in despawn_timer_query.iter_mut() {
        despawn_timer.despawn_timer.tick(time.delta());
        if despawn_timer.despawn_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
