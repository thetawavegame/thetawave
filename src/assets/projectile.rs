use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::spawnable::{Faction, ProjectileType};

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
                Faction::Enemy => todo!(),
                Faction::Neutral => todo!(),
            },
        }
    }
}
