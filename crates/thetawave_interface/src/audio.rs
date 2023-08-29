use bevy_ecs::prelude::Event;
use bevy_kira_audio::AudioTween;
use serde::Deserialize;
use strum_macros::Display;

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum BGMusicType {
    Game,
    Boss,
    BossTransition,
    Main,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display)]
pub enum SoundEffectType {
    Collision(CollisionSoundType),
    BarrierBounce,
    ConsumablePickup,
    DefenseDamage,
    DefenseHeal,
    EnemyFireBlast,
    MenuInputSuccess,
    MobExplosion,
    MobHit,
    PlayerExplosion,
    PlayerFireBlast,
    PlayerHit,
    BulletDing,
    BulletBounce,
    MegablastAbility,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum CollisionSoundType {
    Squishy,
    #[default]
    Normal,
}

#[derive(Event, Default)]
pub struct ChangeBackgroundMusicEvent {
    pub bg_music_type: Option<BGMusicType>,
    pub loop_from: Option<f64>,
    pub fade_in_tween: Option<AudioTween>,
    pub fade_out_tween: Option<AudioTween>,
}

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub sound_effect_type: SoundEffectType,
}
