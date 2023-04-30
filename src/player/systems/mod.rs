//! Systems for managing players

mod ability;
mod attacks;
mod movement;

use crate::assets::GameAudioAssets;
use crate::audio;
use crate::game::GameParametersResource;
use crate::spawnable::{EffectType, InitialMotion, SpawnEffectEvent};
use crate::states::AppStates;
use crate::ui::EndGameTransitionResource;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub use self::ability::*;
pub use self::attacks::{player_fire_weapon_system, player_scale_fire_rate_system};
pub use self::movement::player_movement_system;

use super::PlayerComponent;

/// Handle player reaching zero health
pub fn player_death_system(
    mut commands: Commands,
    mut effect_event_writer: EventWriter<SpawnEffectEvent>,
    player_query: Query<(Entity, &PlayerComponent, &Transform)>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    audio_channel: Res<AudioChannel<audio::SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
    game_parameters: Res<GameParametersResource>,
) {
    // end the game if no players are alive
    if player_query.iter().count() == 0 {
        // transition to the game over state
        end_game_trans_resource.start(AppStates::GameOver);
    }

    // handle death of player entities
    for (entity, player, transform) in player_query.iter() {
        if player.health.is_dead() {
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
                initial_motion: InitialMotion::default(),
            });

            // play explosion sound effect
            audio_channel.play(audio_assets.player_explosion.clone());
        }
    }
}

/// Handle player health regeneration
pub fn player_health_system(mut player_query: Query<&mut PlayerComponent>) {
    for mut player_component in player_query.iter_mut() {
        player_component.health.regenerate_shields();
    }
}
