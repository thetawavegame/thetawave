use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use thetawave_interface::spawnable::EffectType;

#[derive(AssetCollection, Resource)]
pub struct EffectAssets {
    #[asset(key = "ally_blast_explosion")]
    pub ally_blast_explosion: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast_explosion")]
    pub enemy_blast_explosion: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_blast_despawn")]
    pub ally_blast_despawn: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast_despawn")]
    pub enemy_blast_despawn: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet_despawn")]
    pub ally_bullet_despawn: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet_despawn")]
    pub enemy_bullet_despawn: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet_explosion")]
    pub ally_bullet_explosion: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet_explosion")]
    pub enemy_bullet_explosion: Handle<TextureAtlasLayout>,
    #[asset(key = "mob_explosion")]
    pub mob_explosion: Handle<TextureAtlasLayout>,
    #[asset(key = "consumable_despawn")]
    pub consumable_despawn: Handle<TextureAtlasLayout>,
    #[asset(key = "barrier_glow")]
    pub barrier_glow: Handle<TextureAtlasLayout>,
}

impl EffectAssets {
    pub fn get_asset(&self, effect_type: &EffectType) -> Option<Handle<TextureAtlasLayout>> {
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
        }
    }

    pub fn get_color(&self, effect_type: &EffectType, bloom_intensity: f32) -> Color {
        match effect_type {
            EffectType::BarrierGlow => Color::rgb(1.0, 1.0, 1.0 + 0.4 * bloom_intensity),
            EffectType::AllyBlastExplosion => Color::rgb(
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
            ),
            EffectType::EnemyBlastExplosion => Color::rgb(
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
            ),
            EffectType::AllyBulletExplosion => Color::rgb(
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
            ),
            EffectType::EnemyBulletExplosion => Color::rgb(
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
            ),
            EffectType::MobExplosion => Color::rgb(
                1.0 + 5.0 * bloom_intensity,
                1.0 + 5.0 * bloom_intensity,
                1.0 + 5.0 * bloom_intensity,
            ),
            EffectType::AllyBlastDespawn => Color::rgb(
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
            ),
            EffectType::ConsumableDespawn => Color::rgb(
                1.0 + 2.0 * bloom_intensity,
                1.0 + 2.0 * bloom_intensity,
                1.0 + 2.0 * bloom_intensity,
            ),
            EffectType::EnemyBlastDespawn => Color::rgb(
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
                1.0 + 3.0 * bloom_intensity,
            ),
            EffectType::AllyBulletDespawn => Color::rgb(
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
            ),
            EffectType::EnemyBulletDespawn => Color::rgb(
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
                1.0 + 4.0 * bloom_intensity,
            ),
            EffectType::Text(_) => Color::rgb(0.0, 0.0, 0.0),
        }
    }
}
