//! Systems for managing players

mod ability;
mod attacks;
mod movement;

use crate::assets::GameAudioAssets;
use crate::audio;
use crate::game::GameParametersResource;
use crate::misc::HealthComponent;
use crate::run::{RunDefeatType, RunEndEvent, RunOutcomeType};
use crate::spawnable::SpawnEffectEvent;
use crate::ui::EndGameTransitionResource;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use thetawave_interface::spawnable::EffectType;

pub use self::ability::*;
pub use self::attacks::{player_fire_weapon_system, player_scale_fire_rate_system};
pub use self::movement::player_movement_system;

use super::PlayerComponent;

/// Handle player reaching zero health
pub fn player_death_system(
    mut commands: Commands,
    mut effect_event_writer: EventWriter<SpawnEffectEvent>,
    player_query: Query<(Entity, &Transform, &HealthComponent), With<PlayerComponent>>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
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
            audio_channel.play(audio_assets.player_explosion.clone());
        }
    }
}
