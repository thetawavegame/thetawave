use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::spawnable::ProjectileType;

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(key = "juggernaut")]
    pub juggernaut: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct ProjectileAssets {
    #[asset(key = "ally_blast")]
    pub ally_blast: Handle<TextureAtlas>,
    #[asset(key = "enemy_blast")]
    pub enemy_blast: Handle<TextureAtlas>,
    #[asset(key = "neutral_blast")]
    pub neutral_blast: Handle<TextureAtlas>,
}

impl ProjectileAssets {
    pub fn get_asset(&self, projectile_type: &ProjectileType) -> Handle<TextureAtlas> {
        match projectile_type {
            ProjectileType::Blast(faction) => match faction {
                crate::spawnable::Faction::Ally => self.ally_blast.clone(),
                crate::spawnable::Faction::Enemy => self.enemy_blast.clone(),
                crate::spawnable::Faction::Neutral => self.neutral_blast.clone(),
            },
        }
    }
}
