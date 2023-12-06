//! Systems for managing players

mod ability;
mod attacks;
mod movement;

use crate::{game::GameParametersResource, spawnable::SpawnEffectEvent};

use bevy::prelude::*;
use thetawave_interface::audio::{PlaySoundEffectEvent, SoundEffectType};
use thetawave_interface::health::HealthComponent;
use thetawave_interface::player::PlayerComponent;
use thetawave_interface::run::{RunDefeatType, RunEndEvent, RunOutcomeType};
use thetawave_interface::spawnable::EffectType;

pub use self::ability::*;
pub use self::attacks::{player_fire_weapon_system, player_scale_fire_rate_system};
pub use self::movement::{player_movement_system, player_tilt_system};

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
