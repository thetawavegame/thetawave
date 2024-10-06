//! Exposes a plugin that starts, stops, and modulates in-game audio when events are emitted
use bevy::prelude::{
    in_state, not, App, EventReader, IntoSystemConfigs, Plugin, Res, Resource, Startup, Update,
};
use bevy_kira_audio::prelude::{AudioApp, AudioChannel, AudioControl, AudioEasing, AudioTween};
use thetawave_assets::GameAudioAssets;
use thetawave_interface::{
    audio::{ChangeBackgroundMusicEvent, PlaySoundEffectEvent},
    states::AppStates,
};

/// Starts, stops, and modulates in-game audio when we receive a
/// `thetawave_interface::audio::PlaySoundEffectEvent` or
/// `thetawave_interface::audio::ChangeBackgroundMusicEvent`.
pub(super) struct ThetawaveAudioPlugin;

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

/// Sets the volume of the audio channels to "sane defaults"
fn set_audio_volume_system(
    background_audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    menu_audio_channel: Res<AudioChannel<MenuAudioChannel>>,
    effects_audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    background_audio_channel.set_volume(0.20);
    menu_audio_channel.set_volume(0.05);
    effects_audio_channel.set_volume(0.80);
}

/// Play sound effects when we receive events. This should be called every frame for snappy audio.
fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.read() {
        audio_channel.play(audio_assets.get_sound_effect(&event.sound_effect_type));
    }
}

/// System to handle background music changes based on events.
///
/// This system listens for `ChangeBackgroundMusicEvent` events and updates
/// the background music accordingly. It can stop the current music, start new
/// music, handle looping, and apply fade-in and fade-out effects if specified in the event.
///
/// - If an event specifies a fade-out duration, the current track will fade out before stopping.
/// - If a new background music type is provided, it will play the corresponding track from the `GameAudioAssets`.
/// - The system supports looping the new track from a specified point and applying a fade-in effect if specified.
///
/// Parameters:
/// - `EventReader<ChangeBackgroundMusicEvent>`: Reads events that dictate when and how to change background music.
/// - `AudioChannel<BackgroundMusicAudioChannel>`: Controls the background music audio channel, allowing for stop, play, and fade effects.
/// - `GameAudioAssets`: A resource that holds all available audio assets.
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
