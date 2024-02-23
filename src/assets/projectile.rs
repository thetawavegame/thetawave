use bevy::prelude::{Color, Handle, Resource, TextureAtlasLayout};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::spawnable::{Faction, ProjectileType};

#[derive(AssetCollection, Resource)]
pub struct ProjectileAssets {
    #[asset(key = "ally_blast")]
    pub ally_blast: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast")]
    pub enemy_blast: Handle<TextureAtlasLayout>,
    #[asset(key = "neutral_blast")]
    pub neutral_blast: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet")]
    pub ally_bullet: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet")]
    pub enemy_bullet: Handle<TextureAtlasLayout>,
}

impl ProjectileAssets {
    pub fn get_asset(&self, projectile_type: &ProjectileType) -> Handle<TextureAtlasLayout> {
        match projectile_type {
            ProjectileType::Blast(faction) => match faction {
                Faction::Ally => self.ally_blast.clone(),
                Faction::Enemy => self.enemy_blast.clone(),
                Faction::Neutral => self.neutral_blast.clone(),
            },
            ProjectileType::Bullet(faction) => match faction {
                Faction::Ally => self.ally_bullet.clone(),
                Faction::Enemy => self.enemy_bullet.clone(),
                Faction::Neutral => todo!(),
            },
        }
    }
    pub fn get_color(&self, projectile_type: &ProjectileType, bloom_intensity: f32) -> Color {
        match projectile_type {
            ProjectileType::Blast(_) => Color::rgb(
                1.0 + 2.0 * bloom_intensity,
                1.0 + 2.0 * bloom_intensity,
                1.0 + 2.0 * bloom_intensity,
            ),
            ProjectileType::Bullet(_) => Color::rgb(
                1.0 + 1.0 * bloom_intensity,
                1.0 + 1.0 * bloom_intensity,
                1.0 + 1.0 * bloom_intensity,
            ),
        }
    }
}
