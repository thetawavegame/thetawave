use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use thetawave_interface::spawnable::{Faction, ProjectileType};

#[derive(AssetCollection, Resource)]
pub struct ProjectileAssets {
    #[asset(key = "ally_blast")]
    pub ally_blast: Handle<TextureAtlas>,
    #[asset(key = "enemy_blast")]
    pub enemy_blast: Handle<TextureAtlas>,
    #[asset(key = "neutral_blast")]
    pub neutral_blast: Handle<TextureAtlas>,
    #[asset(key = "ally_bullet")]
    pub ally_bullet: Handle<TextureAtlas>,
    #[asset(key = "enemy_bullet")]
    pub enemy_bullet: Handle<TextureAtlas>,
    #[asset(key = "ally_beam")]
    pub ally_beam: Handle<TextureAtlas>,
    #[asset(key = "enemy_beam")]
    pub enemy_beam: Handle<TextureAtlas>,
}

impl ProjectileAssets {
    pub fn get_asset(&self, projectile_type: &ProjectileType) -> Handle<TextureAtlas> {
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
            ProjectileType::Beam(faction) => match faction {
                Faction::Ally => self.ally_beam.clone(),
                Faction::Enemy => self.enemy_beam.clone(),
                Faction::Neutral => todo!(),
            },
        }
    }
    pub fn get_color(&self, projectile_type: &ProjectileType) -> Color {
        match projectile_type {
            ProjectileType::Blast(_) => Color::rgb(3.0, 3.0, 3.0),
            ProjectileType::Bullet(_) => Color::rgb(2.0, 2.0, 2.0),
            ProjectileType::Beam(_) => Color::rgb(3.0, 3.0, 3.0),
        }
    }
}
