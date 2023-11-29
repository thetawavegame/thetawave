use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use thetawave_interface::{
    audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent},
    states::AppStates,
};

use crate::assets::GameAudioAssets;

pub struct ThetawaveAudioPlugin;

impl Plugin for ThetawaveAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEffectEvent>();
        app.add_event::<ChangeBackgroundMusicEvent>();

        app.add_audio_channel::<BackgroundMusicAudioChannel>()
            .add_audio_channel::<MenuAudioChannel>()
            .add_audio_channel::<SoundEffectsAudioChannel>();

        app.add_systems(Startup, set_audio_volume_system);

        app.add_systems(
            Update,
            (play_sound_effect_system, change_bg_music_system)
                .run_if(not(in_state(AppStates::LoadingAssets))),
        );
    }
}

// audio channels
#[derive(Resource)]
pub struct BackgroundMusicAudioChannel;
#[derive(Resource)]
pub struct MenuAudioChannel;
#[derive(Resource)]
pub struct SoundEffectsAudioChannel;

/// Sets the volume of the audio channels
pub fn set_audio_volume_system(
    background_audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    menu_audio_channel: Res<AudioChannel<MenuAudioChannel>>,
    effects_audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    background_audio_channel.set_volume(0.70);
    menu_audio_channel.set_volume(0.05);
    effects_audio_channel.set_volume(0.60);
}

fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        audio_channel.play(audio_assets.get_sound_effect(&event.sound_effect_type));
    }
}

fn change_bg_music_system(
    mut change_bg_music_event_reader: EventReader<ChangeBackgroundMusicEvent>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in change_bg_music_event_reader.read() {
        // stop audio if playing sound
        if audio_channel.is_playing_sound() {
            let mut stop_command = audio_channel.stop();

            // use fade if specified
            if let Some(fade_out) = event.fade_out {
                stop_command.fade_out(AudioTween::new(fade_out, AudioEasing::Linear));
            }
        }

        // play music if provided a type
        if let Some(bg_music_type) = event.bg_music_type.clone() {
            let mut start_command =
                audio_channel.play(audio_assets.get_bg_music_asset(&bg_music_type));

            // loop if true
            if let Some(loop_from) = event.loop_from {
                start_command.loop_from(loop_from);
            }

            // use fade if specified
            if let Some(fade_in) = event.fade_in {
                start_command.fade_in(AudioTween::new(fade_in, AudioEasing::Linear));
            }
        }
    }
}
