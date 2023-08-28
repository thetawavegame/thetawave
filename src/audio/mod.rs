use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use thetawave_interface::states::AppStates;

use crate::assets::{BGMusicType, GameAudioAssets, SoundEffectType};

pub struct ThetawaveAudioPlugin;

impl Plugin for ThetawaveAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaySoundEffectEvent>();
        app.add_event::<PlayBackgroundMusicEvent>();

        app.add_audio_channel::<BackgroundMusicAudioChannel>()
            .add_audio_channel::<MenuAudioChannel>()
            .add_audio_channel::<SoundEffectsAudioChannel>();

        app.add_systems(Startup, set_audio_volume_system);

        app.add_systems(
            Update,
            (play_sound_effect_system, play_bg_music_system)
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

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub sound_effect_type: SoundEffectType,
}

fn play_sound_effect_system(
    mut play_sound_event_reader: EventReader<PlaySoundEffectEvent>,
    audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_sound_event_reader.iter() {
        audio_channel.play(audio_assets.get_sound_effect(&event.sound_effect_type));
    }
}

#[derive(Event)]
pub struct PlayBackgroundMusicEvent {
    pub bg_music_type: BGMusicType,
    pub looped: bool,
    pub fade: Option<AudioTween>,
}

fn play_bg_music_system(
    mut play_bg_music_event_reader: EventReader<PlayBackgroundMusicEvent>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    audio_assets: Res<GameAudioAssets>,
) {
    for event in play_bg_music_event_reader.iter() {
        let mut command = audio_channel.play(audio_assets.get_bg_music_asset(&event.bg_music_type));
        if event.looped {
            command.looped();
        }
        if let Some(fade) = event.fade.clone() {
            command.fade_in(fade);
        }
    }
}
