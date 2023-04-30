use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct ThetawaveAudioPlugin;

impl Plugin for ThetawaveAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<BackgroundMusicAudioChannel>()
            .add_audio_channel::<MenuAudioChannel>()
            .add_audio_channel::<SoundEffectsAudioChannel>();

        app.add_startup_system(set_audio_volume_system);
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
