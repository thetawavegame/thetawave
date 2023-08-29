use bevy_ecs::prelude::Event;
use bevy_kira_audio::AudioTween;
use serde::Deserialize;
use strum_macros::Display;

/// Background music types
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

/// Subtype of sound effect for collisions
#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone, Display, Default)]
pub enum CollisionSoundType {
    Squishy,
    #[default]
    Normal,
}

#[derive(Event, Default)]
pub struct ChangeBackgroundMusicEvent {
    /// Background music to change to, None will just stop the current music
    pub bg_music_type: Option<BGMusicType>,
    /// Loop from a specific time in the track, None will not loop the track
    pub loop_from: Option<f64>,
    /// Fade in the music cycling in (bg_music_type)
    pub fade_in_tween: Option<AudioTween>,
    /// Fade out the music currently being played
    pub fade_out_tween: Option<AudioTween>,
}

#[derive(Event)]
pub struct PlaySoundEffectEvent {
    pub sound_effect_type: SoundEffectType,
}
