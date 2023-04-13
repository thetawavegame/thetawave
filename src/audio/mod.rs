use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{assets::GameAudioAssets, states, GameEnterSet};

pub struct ThetawaveAudioPlugin;

impl Plugin for ThetawaveAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<BackgroundMusicAudioChannel>()
            .add_audio_channel::<MenuAudioChannel>()
            .add_audio_channel::<SoundEffectsAudioChannel>();

        app.add_startup_system(set_audio_volume_system);

        app.add_systems(
            (start_background_audio_system.in_set(GameEnterSet::BuildLevel),)
                .in_schedule(OnEnter(states::AppStates::Game)),
        );

        app.add_systems(
            (stop_background_audio_system,).in_schedule(OnEnter(states::AppStates::MainMenu)),
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

pub fn start_background_audio_system(
    audio_assets: Res<GameAudioAssets>,
    audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
) {
    audio_channel.play(audio_assets.game_music.clone()).looped();
}

pub fn stop_background_audio_system(audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>) {
    audio_channel.stop();
}

pub fn set_audio_volume_system(
    background_audio_channel: Res<AudioChannel<BackgroundMusicAudioChannel>>,
    menu_audio_channel: Res<AudioChannel<MenuAudioChannel>>,
    effects_audio_channel: Res<AudioChannel<SoundEffectsAudioChannel>>,
) {
    background_audio_channel.set_volume(0.50);
    menu_audio_channel.set_volume(0.05);
    effects_audio_channel.set_volume(0.70);
}
