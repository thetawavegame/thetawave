use bevy::{
    prelude::{Color, Handle, Resource, TextureAtlasLayout},
    render::texture::Image,
};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::spawnable::{Faction, ProjectileType};

#[derive(AssetCollection, Resource)]
pub struct ProjectileAssets {
    #[asset(key = "ally_blast.layout")]
    pub ally_blast_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_blast.image")]
    pub ally_blast_image: Handle<Image>,
    #[asset(key = "enemy_blast.layout")]
    pub enemy_blast_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_blast.image")]
    pub enemy_blast_image: Handle<Image>,
    #[asset(key = "neutral_blast.layout")]
    pub neutral_blast_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "neutral_blast.image")]
    pub neutral_blast_image: Handle<Image>,
    #[asset(key = "ally_bullet.layout")]
    pub ally_bullet_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "ally_bullet.image")]
    pub ally_bullet_image: Handle<Image>,
    #[asset(key = "enemy_bullet.layout")]
    pub enemy_bullet_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "enemy_bullet.image")]
    pub enemy_bullet_image: Handle<Image>,
}

impl ProjectileAssets {
    pub fn get_texture_atlas_layout(
        &self,
        projectile_type: &ProjectileType,
    ) -> Handle<TextureAtlasLayout> {
        match projectile_type {
            ProjectileType::Blast(faction) => match faction {
                Faction::Ally => self.ally_blast_layout.clone(),
                Faction::Enemy => self.enemy_blast_layout.clone(),
                Faction::Neutral => self.neutral_blast_layout.clone(),
            },
            ProjectileType::Bullet(faction) => match faction {
                Faction::Ally => self.ally_bullet_layout.clone(),
                Faction::Enemy => self.enemy_bullet_layout.clone(),
                Faction::Neutral => todo!(),
            },
        }
    }

    pub fn get_image(&self, projectile_type: &ProjectileType) -> Handle<Image> {
        match projectile_type {
            ProjectileType::Blast(faction) => match faction {
                Faction::Ally => self.ally_blast_image.clone(),
                Faction::Enemy => self.enemy_blast_image.clone(),
                Faction::Neutral => self.neutral_blast_image.clone(),
            },
            ProjectileType::Bullet(faction) => match faction {
                Faction::Ally => self.ally_bullet_image.clone(),
                Faction::Enemy => self.enemy_bullet_image.clone(),
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
