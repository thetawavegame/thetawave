//! Systems for managing players

mod abilities;
mod movement;
mod upgrades;

use crate::{game::GameParametersResource, spawnable::SpawnEffectEvent};

use bevy::ecs::entity::Entity;
use bevy::ecs::event::EventWriter;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::math::Vec3;
use bevy::transform::components::Transform;
use bevy::utils::default;
use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};
use thetawave_interface::health::HealthComponent;
use thetawave_interface::player::PlayerComponent;
use thetawave_interface::run::{RunDefeatType, RunEndEvent, RunOutcomeType};
use thetawave_interface::spawnable::EffectType;

pub use self::abilities::{
    player_ability_cooldown_system, player_ability_input_system, standard_weapon_ability_system,
    start_charge_ability_system, update_charge_ability_system,
};
pub use self::movement::{player_movement_system, player_tilt_system};
pub use self::upgrades::scale_ability_cooldowns_system;

use super::PlayersResource;

/// Handle player reaching zero health
pub fn player_death_system(
    mut commands: Commands,
    mut effect_event_writer: EventWriter<SpawnEffectEvent>,
    player_query: Query<(Entity, &Transform, &HealthComponent), With<PlayerComponent>>,
    mut sound_effect_event_writer: EventWriter<PlaySoundEffectEvent>,
    game_parameters: Res<GameParametersResource>,
    mut run_end_event_writer: EventWriter<RunEndEvent>,
) {
    // end the game if no players are alive
    if player_query.iter().count() == 0 {
        run_end_event_writer.send(RunEndEvent {
            outcome: RunOutcomeType::Defeat(RunDefeatType::PlayersDestroyed),
        });
    }

    // handle death of player entities
    for (entity, transform, health) in player_query.iter() {
        if health.is_dead() {
            // despawn the player
            commands.entity(entity).despawn_recursive();

            // spawn explosion effect
            effect_event_writer.send(SpawnEffectEvent {
                effect_type: EffectType::MobExplosion,
                transform: Transform {
                    translation: transform.translation,
                    scale: Vec3::new(
                        game_parameters.sprite_scale,
                        game_parameters.sprite_scale,
                        1.0,
                    ),
                    ..Default::default()
                },
                ..default()
            });

            // play explosion sound effect
            sound_effect_event_writer.send(PlaySoundEffectEvent {
                sound_effect_type: SoundEffectType::PlayerExplosion,
            });
        }
    }
}

pub fn players_reset_system(mut players_resource: ResMut<PlayersResource>) {
    *players_resource = PlayersResource::default();
}
