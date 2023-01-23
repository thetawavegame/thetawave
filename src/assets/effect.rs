use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::spawnable::EffectType;

#[derive(AssetCollection, Resource)]
pub struct EffectAssets {
    #[asset(key = "ally_blast_explosion")]
    pub ally_blast_explosion: Handle<TextureAtlas>,
    #[asset(key = "enemy_blast_explosion")]
    pub enemy_blast_explosion: Handle<TextureAtlas>,
    #[asset(key = "ally_blast_despawn")]
    pub ally_blast_despawn: Handle<TextureAtlas>,
    #[asset(key = "enemy_blast_despawn")]
    pub enemy_blast_despawn: Handle<TextureAtlas>,
    #[asset(key = "enemy_bullet_despawn")]
    pub enemy_bullet_despawn: Handle<TextureAtlas>,
    #[asset(key = "enemy_bullet_explosion")]
    pub enemy_bullet_explosion: Handle<TextureAtlas>,
    #[asset(key = "mob_explosion")]
    pub mob_explosion: Handle<TextureAtlas>,
    #[asset(key = "consumable_despawn")]
    pub consumable_despawn: Handle<TextureAtlas>,
    #[asset(key = "barrier_glow")]
    pub barrier_glow: Handle<TextureAtlas>,
}

impl EffectAssets {
    pub fn get_asset(&self, effect_type: &EffectType) -> Handle<TextureAtlas> {
        match effect_type {
            EffectType::AllyBlastExplosion => self.ally_blast_explosion.clone(),
            EffectType::AllyBlastDespawn => self.ally_blast_despawn.clone(),
            EffectType::MobExplosion => self.mob_explosion.clone(),
            EffectType::ConsumableDespawn => self.consumable_despawn.clone(),
            EffectType::EnemyBlastExplosion => self.enemy_blast_explosion.clone(),
            EffectType::EnemyBlastDespawn => self.enemy_blast_despawn.clone(),
            EffectType::BarrierGlow => self.barrier_glow.clone(),
            EffectType::EnemyBulletDespawn => self.enemy_bullet_despawn.clone(),
            EffectType::EnemyBulletExplosion => self.enemy_bullet_explosion.clone(),
        }
    }
}
