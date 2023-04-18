//! `thetawave` player module
use bevy::prelude::*;
use ron::de::from_bytes;

mod components;
mod resources;
mod spawn;
mod systems;

use crate::{states, GameEnterSet, GameUpdateSet};

pub use self::{
    components::PlayerComponent,
    resources::{Character, CharacterType, CharactersResource, PlayersResource},
    spawn::spawn_player_system,
    systems::{
        player_ability_system, player_death_system, player_fire_weapon_system,
        player_movement_system, player_scale_fire_rate_system,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            from_bytes::<CharactersResource>(include_bytes!("../../assets/data/characters.ron"))
                .unwrap(),
        );

        app.insert_resource(PlayersResource::default());

        app.add_system(
            spawn_player_system
                .in_set(GameEnterSet::SpawnPlayer)
                .in_schedule(OnEnter(states::AppStates::Game)),
        )
        .add_systems(
            (
                player_fire_weapon_system,
                player_death_system,
                player_scale_fire_rate_system,
                player_movement_system.in_set(GameUpdateSet::Movement),
                player_ability_system.in_set(GameUpdateSet::Abilities),
            )
                .in_set(OnUpdate(states::AppStates::Game))
                .in_set(OnUpdate(states::GameStates::Playing)),
        );
    }
}
