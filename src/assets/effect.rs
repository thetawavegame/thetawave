use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use thetawave_interface::spawnable::EffectType;

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
    #[asset(key = "ally_bullet_despawn")]
    pub ally_bullet_despawn: Handle<TextureAtlas>,
    #[asset(key = "enemy_bullet_despawn")]
    pub enemy_bullet_despawn: Handle<TextureAtlas>,
    #[asset(key = "ally_beam_despawn")]
    pub ally_beam_despawn: Handle<TextureAtlas>,
    #[asset(key = "ally_bullet_explosion")]
    pub ally_bullet_explosion: Handle<TextureAtlas>,
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
    pub fn get_asset(&self, effect_type: &EffectType) -> Option<Handle<TextureAtlas>> {
        match effect_type {
            EffectType::AllyBlastExplosion => Some(self.ally_blast_explosion.clone()),
            EffectType::AllyBlastDespawn => Some(self.ally_blast_despawn.clone()),
            EffectType::MobExplosion => Some(self.mob_explosion.clone()),
            EffectType::ConsumableDespawn => Some(self.consumable_despawn.clone()),
            EffectType::EnemyBlastExplosion => Some(self.enemy_blast_explosion.clone()),
            EffectType::EnemyBlastDespawn => Some(self.enemy_blast_despawn.clone()),
            EffectType::BarrierGlow => Some(self.barrier_glow.clone()),
            EffectType::AllyBulletDespawn => Some(self.ally_bullet_despawn.clone()),
            EffectType::EnemyBulletDespawn => Some(self.enemy_bullet_despawn.clone()),
            EffectType::AllyBulletExplosion => Some(self.ally_bullet_explosion.clone()),
            EffectType::EnemyBulletExplosion => Some(self.enemy_bullet_explosion.clone()),
            EffectType::Text(_) => None,
            EffectType::AllyBeamDespawn => Some(self.ally_beam_despawn.clone()),
        }
    }

    pub fn get_color(&self, effect_type: &EffectType) -> Color {
        match effect_type {
            EffectType::BarrierGlow => Color::rgb(1.0, 1.0, 1.4),
            EffectType::AllyBlastExplosion => Color::rgb(4.0, 4.0, 4.0),
            EffectType::EnemyBlastExplosion => Color::rgb(4.0, 4.0, 4.0),
            EffectType::AllyBulletExplosion => Color::rgb(5.0, 5.0, 5.0),
            EffectType::EnemyBulletExplosion => Color::rgb(5.0, 5.0, 5.0),
            EffectType::MobExplosion => Color::rgb(6.0, 6.0, 6.0),
            EffectType::AllyBlastDespawn => Color::rgb(4.0, 4.0, 4.0),
            EffectType::ConsumableDespawn => Color::rgb(3.0, 3.0, 3.0),
            EffectType::EnemyBlastDespawn => Color::rgb(4.0, 4.0, 4.0),
            EffectType::AllyBulletDespawn => Color::rgb(5.0, 5.0, 5.0),
            EffectType::EnemyBulletDespawn => Color::rgb(5.0, 5.0, 5.0),
            EffectType::Text(_) => Color::rgb(0.0, 0.0, 0.0),
            EffectType::AllyBeamDespawn => Color::rgb(2.0, 2.0, 2.0),
        }
    }
}
