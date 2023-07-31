//! `thetawave` game module
use bevy::prelude::*;
use ron::de::from_bytes;
mod resources;

use crate::{player::PlayerComponent, spawnable::SpawnProjectileEvent};

pub use self::resources::{CurrentGameMetrics, GameParametersResource};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<GameParametersResource>(include_bytes!(
                "../../assets/data/game_parameters.ron"
            ))
            .unwrap(),
        );
        app.insert_resource(CurrentGameMetrics::default());
        app.add_systems(Update, count_shots_fired_by_player_1_system);
    }
}
fn count_shots_fired_by_player_1_system(
    mut shots_fired_event_reader: EventReader<SpawnProjectileEvent>,
    query: Query<(Entity, &PlayerComponent)>,
    mut current_game_metrics: ResMut<CurrentGameMetrics>,
) {
    let player_1_entity_id = query
        .iter()
        .find(|(_, pc)| pc.player_index == 0)
        .map(|(x, _)| x);
    let n_p1_shots_fired = match player_1_entity_id {
        Some(entity) => shots_fired_event_reader
            .iter()
            .filter(|x| x.source == entity)
            .count(),
        None => 0,
    };
    if n_p1_shots_fired > 0 {
        debug!(
            "Incrementing total player 1 shots by {}, n_p1_shots_fired",
            n_p1_shots_fired
        );
        current_game_metrics.n_shots_fired += n_p1_shots_fired;
    }
}
