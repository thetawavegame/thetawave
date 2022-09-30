use bevy::prelude::*;
use serde::Deserialize;
use strum_macros::Display;

use crate::game::GameParametersResource;

use super::{MobComponent, MobsResource};

mod repeater;
pub use self::repeater::{spawn_boss, RepeaterPart, RepeaterPartsData, RepeaterResource};

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum BossType {
    Repeater,
}

/// Event for spawning a boss
pub struct SpawnBossEvent {
    // type representing a kind of boss
    pub boss_type: BossType,
    // position of the boss to be spawned
    pub position: Vec2,
}

/// Read spawn boss events and spawn bosses accordingly
pub fn spawn_boss_system(
    mut commands: Commands,
    repeater_resource: Res<RepeaterResource>,
    game_parameters: Res<GameParametersResource>,
    mut spawn_boss_event: EventReader<SpawnBossEvent>,
) {
    for event in spawn_boss_event.iter() {
        spawn_boss(
            &event.boss_type,
            &repeater_resource,
            event.position,
            &mut commands,
            &game_parameters,
        );
    }
}
