use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use rand::Rng;
use thetawave_interface::audio::{BGMusicType, CollisionSoundType, SoundEffectType};

#[derive(AssetCollection, Resource)]
pub struct GameAudioAssets {
    #[asset(key = "sounds.main_music")]
    pub main_music: Handle<AudioSource>,
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
    #[asset(key = "sounds.objective_completed")]
    pub objective_completed: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_1")]
    pub button_select_1: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_2")]
    pub button_select_2: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_3")]
    pub button_select_3: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_4")]
    pub button_select_4: Handle<AudioSource>,
    #[asset(key = "sounds.button_select_5")]
    pub button_select_5: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_1")]
    pub button_release_1: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_2")]
    pub button_release_2: Handle<AudioSource>,
    #[asset(key = "sounds.button_release_3")]
    pub button_release_3: Handle<AudioSource>,
    #[asset(key = "sounds.button_confirm")]
    pub button_confirm: Handle<AudioSource>,
}

impl GameAudioAssets {
    pub fn get_bg_music_asset(&self, bg_music_type: &BGMusicType) -> Handle<AudioSource> {
        match bg_music_type {
            BGMusicType::Game => self.game_music.clone(),
            BGMusicType::Boss => self.boss_music.clone(),
            BGMusicType::BossTransition => self.boss_trans_music.clone(),
            BGMusicType::Main => self.main_music.clone(),
        }
    }

    pub fn get_sound_effect(&self, sound_type: &SoundEffectType) -> Handle<AudioSource> {
        match sound_type {
            SoundEffectType::Collision(collsion_type) => match collsion_type {
                CollisionSoundType::Squishy => self.squishy_collision.clone(),
                CollisionSoundType::Normal => self.collision.clone(),
            },
            SoundEffectType::BarrierBounce => self.barrier_bounce.clone(),
            SoundEffectType::ConsumablePickup => self.consumable_pickup.clone(),
            SoundEffectType::DefenseDamage => self.defense_damage.clone(),
            SoundEffectType::DefenseHeal => self.defense_heal.clone(),
            SoundEffectType::EnemyFireBlast => self.enemy_fire_blast.clone(),
            SoundEffectType::MenuInputSuccess => self.menu_input_success.clone(),
            SoundEffectType::MobExplosion => self.mob_explosion.clone(),
            SoundEffectType::MobHit => self.mob_hit.clone(),
            SoundEffectType::PlayerExplosion => self.player_explosion.clone(),
            SoundEffectType::PlayerFireBlast => self.player_fire_blast.clone(),
            SoundEffectType::PlayerHit => self.player_hit.clone(),
            SoundEffectType::BulletDing => self.bullet_ding.clone(),
            SoundEffectType::BulletBounce => self.bullet_bounce.clone(),
            SoundEffectType::MegaBlastAbility => self.megablast_ability.clone(),
            SoundEffectType::ObjectiveCompleted => self.objective_completed.clone(),
            SoundEffectType::ButtonRelease => {
                let idx: u8 = rand::thread_rng().gen_range(1..=3);
                match idx {
                    1 => self.button_release_1.clone(),
                    2 => self.button_release_2.clone(),
                    _ => self.button_release_3.clone(),
                }
            }
            SoundEffectType::ButtonSelect => {
                let idx: u8 = rand::thread_rng().gen_range(1..=5);
                match idx {
                    1 => self.button_select_1.clone(),
                    2 => self.button_select_2.clone(),
                    3 => self.button_select_3.clone(),
                    4 => self.button_select_4.clone(),
                    _ => self.button_select_5.clone(),
                }
            }
            SoundEffectType::ButtonConfirm => self.button_confirm.clone(),
        }
    }
}
