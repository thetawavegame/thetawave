use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use serde::Deserialize;
use strum_macros::Display;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum BGMusicType {
    Game,
    Boss,
    BossTransition,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum CollisionSoundType {
    Squishy,
    #[default]
    Normal,
}

#[derive(AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(key = "sounds.game_music")]
    pub game_music: Handle<AudioSource>,
    #[asset(key = "sounds.boss_music")]
    pub boss_music: Handle<AudioSource>,
    #[asset(key = "sounds.boss_trans_music")]
    pub boss_trans_music: Handle<AudioSource>,
    #[asset(key = "sounds.barrier_bounce")]
    pub barrier_bounce: Handle<AudioSource>,
    #[asset(key = "sounds.collision")]
    pub collision: Handle<AudioSource>,
    #[asset(key = "sounds.squishy_collision")]
    pub squishy_collision: Handle<AudioSource>,
    #[asset(key = "sounds.consumable_pickup")]
    pub consumable_pickup: Handle<AudioSource>,
    #[asset(key = "sounds.defense_damage")]
    pub defense_damage: Handle<AudioSource>,
    #[asset(key = "sounds.defense_heal")]
    pub defense_heal: Handle<AudioSource>,
    #[asset(key = "sounds.enemy_fire_blast")]
    pub enemy_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.menu_input_success")]
    pub menu_input_success: Handle<AudioSource>,
    #[asset(key = "sounds.mob_explosion")]
    pub mob_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.mob_hit")]
    pub mob_hit: Handle<AudioSource>,
    #[asset(key = "sounds.player_explosion")]
    pub player_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.player_fire_blast")]
    pub player_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.player_hit")]
    pub player_hit: Handle<AudioSource>,
    #[asset(key = "sounds.bullet_ding")]
    pub bullet_ding: Handle<AudioSource>,
    #[asset(key = "sounds.bullet_bounce")]
    pub bullet_bounce: Handle<AudioSource>,
    #[asset(key = "sounds.megablast_ability")]
    pub megablast_ability: Handle<AudioSource>,
}

impl GameAudioAssets {
    pub fn get_bg_music_asset(&self, bg_music_type: &BGMusicType) -> Handle<AudioSource> {
        match bg_music_type {
            BGMusicType::Game => self.game_music.clone(),
            BGMusicType::Boss => self.boss_music.clone(),
            BGMusicType::BossTransition => self.boss_trans_music.clone(),
        }
    }

    pub fn get_collision_sound_asset(
        &self,
        collision_sound_type: &CollisionSoundType,
    ) -> Handle<AudioSource> {
        match collision_sound_type {
            CollisionSoundType::Squishy => self.squishy_collision.clone(),
            CollisionSoundType::Normal => self.collision.clone(),
        }
    }
}
