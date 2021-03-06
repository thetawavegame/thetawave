//! Systems for managing players

mod attacks;
mod movement;

use crate::game_over::EndGameTransitionResource;
use crate::spawnable::{EffectType, SpawnEffectEvent};
use crate::states::AppStates;
use crate::SoundEffectsAudioChannel;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_kira_audio::AudioChannel;

pub use self::attacks::player_fire_weapon_system;
pub use self::movement::player_movement_system;

use super::PlayerComponent;

pub fn player_death_system(
    mut commands: Commands,
    mut effect_event_writer: EventWriter<SpawnEffectEvent>,
    player_query: Query<(Entity, &PlayerComponent, &Transform)>,
    mut end_game_trans_resource: ResMut<EndGameTransitionResource>,
    asset_server: Res<AssetServer>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    for (entity, player, transform) in player_query.iter() {
        if player.health.is_dead() {
            commands.entity(entity).despawn_recursive();
            effect_event_writer.send(SpawnEffectEvent {
                effect_type: EffectType::MobExplosion,
                position: transform.translation.xy(),
                scale: Vec2::ZERO,
                rotation: 0.0,
            });
            audio_channel.play(asset_server.load("sounds/player_explosion.wav"));
            end_game_trans_resource.start(AppStates::GameOver);
        }
    }
}
