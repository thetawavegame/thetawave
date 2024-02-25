use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use thetawave_interface::spawnable::EffectType;

#[derive(AssetCollection, Resource)]
pub struct EffectAssets {
    #[asset(key = "ally_blast_explosion.layout")]
    pub ally_blast_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_blast_explosion.image")]
    pub ally_blast_explosion_image: Handle<Image>,
    #[asset(key = "enemy_blast_explosion.layout")]
    pub enemy_blast_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast_explosion.image")]
    pub enemy_blast_explosion_image: Handle<Image>,
    #[asset(key = "ally_blast_despawn.layout")]
    pub ally_blast_despawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_blast_despawn.image")]
    pub ally_blast_despawn_image: Handle<Image>,
    #[asset(key = "enemy_blast_despawn.layout")]
    pub enemy_blast_despawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast_despawn.image")]
    pub enemy_blast_despawn_image: Handle<Image>,
    #[asset(key = "ally_bullet_despawn.layout")]
    pub ally_bullet_despawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet_despawn.image")]
    pub ally_bullet_despawn_image: Handle<Image>,
    #[asset(key = "enemy_bullet_despawn.layout")]
    pub enemy_bullet_despawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet_despawn.image")]
    pub enemy_bullet_despawn_image: Handle<Image>,
    #[asset(key = "ally_bullet_explosion.layout")]
    pub ally_bullet_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet_explosion.image")]
    pub ally_bullet_explosion_image: Handle<Image>,
    #[asset(key = "enemy_bullet_explosion.layout")]
    pub enemy_bullet_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet_explosion.image")]
    pub enemy_bullet_explosion_image: Handle<Image>,
    #[asset(key = "mob_explosion.layout")]
    pub mob_explosion_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mob_explosion.image")]
    pub mob_explosion_image: Handle<Image>,
    #[asset(key = "consumable_despawn.layout")]
    pub consumable_despawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "consumable_despawn.image")]
    pub consumable_despawn_image: Handle<Image>,
    #[asset(key = "barrier_glow.layout")]
    pub barrier_glow_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "barrier_glow.image")]
    pub barrier_glow_image: Handle<Image>,
}

impl EffectAssets {
    pub fn get_texture_atlas_layout(
        &self,
        effect_type: &EffectType,
    ) -> Option<Handle<TextureAtlasLayout>> {
        match effect_type {
            EffectType::AllyBlastExplosion => Some(self.ally_blast_explosion_layout.clone()),
            EffectType::AllyBlastDespawn => Some(self.ally_blast_despawn_layout.clone()),
            EffectType::MobExplosion => Some(self.mob_explosion_layout.clone()),
            EffectType::ConsumableDespawn => Some(self.consumable_despawn_layout.clone()),
            EffectType::EnemyBlastExplosion => Some(self.enemy_blast_explosion_layout.clone()),
            EffectType::EnemyBlastDespawn => Some(self.enemy_blast_despawn_layout.clone()),
            EffectType::BarrierGlow => Some(self.barrier_glow_layout.clone()),
            EffectType::AllyBulletDespawn => Some(self.ally_bullet_despawn_layout.clone()),
            EffectType::EnemyBulletDespawn => Some(self.enemy_bullet_despawn_layout.clone()),
            EffectType::AllyBulletExplosion => Some(self.ally_bullet_explosion_layout.clone()),
            EffectType::EnemyBulletExplosion => Some(self.enemy_bullet_explosion_layout.clone()),
            EffectType::Text(_) => None,
        }
    }

    pub fn get_image(&self, effect_type: &EffectType) -> Option<Handle<Image>> {
        match effect_type {
            EffectType::AllyBlastExplosion => Some(self.ally_blast_explosion_image.clone()),
            EffectType::AllyBlastDespawn => Some(self.ally_blast_despawn_image.clone()),
            EffectType::MobExplosion => Some(self.mob_explosion_image.clone()),
            EffectType::ConsumableDespawn => Some(self.consumable_despawn_image.clone()),
            EffectType::EnemyBlastExplosion => Some(self.enemy_blast_explosion_image.clone()),
            EffectType::EnemyBlastDespawn => Some(self.enemy_blast_despawn_image.clone()),
            EffectType::BarrierGlow => Some(self.barrier_glow_image.clone()),
            EffectType::AllyBulletDespawn => Some(self.ally_bullet_despawn_image.clone()),
            EffectType::EnemyBulletDespawn => Some(self.enemy_bullet_despawn_image.clone()),
            EffectType::AllyBulletExplosion => Some(self.ally_bullet_explosion_image.clone()),
            EffectType::EnemyBulletExplosion => Some(self.enemy_bullet_explosion_image.clone()),
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
