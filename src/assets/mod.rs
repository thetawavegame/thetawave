use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    player::{Character, CharacterType},
    spawnable::{ConsumableType, EffectType, EnemyType, MobType, ProjectileType},
};

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(key = "juggernaut")]
    pub juggernaut: Handle<Image>,
}

impl PlayerAssets {
    pub fn get_asset(&self, character_type: &CharacterType) -> Handle<Image> {
        match character_type {
            CharacterType::Juggernaut => self.juggernaut.clone(),
        }
    }
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

#[derive(AssetCollection, Resource)]
pub struct ConsumableAssets {
    #[asset(key = "health_wrench")]
    pub health_wrench: Handle<TextureAtlas>,
    #[asset(key = "defense_wrench")]
    pub defense_wrench: Handle<TextureAtlas>,
    #[asset(key = "money5")]
    pub money5: Handle<TextureAtlas>,
    #[asset(key = "money1")]
    pub money1: Handle<TextureAtlas>,
    #[asset(key = "armor")]
    pub armor: Handle<TextureAtlas>,
}

impl ConsumableAssets {
    pub fn get_asset(&self, consumable_type: &ConsumableType) -> Handle<TextureAtlas> {
        match consumable_type {
            ConsumableType::DefenseWrench => self.health_wrench.clone(),
            ConsumableType::Money1 => self.money1.clone(),
            ConsumableType::Money5 => self.money5.clone(),
            ConsumableType::HealthWrench => self.health_wrench.clone(),
            ConsumableType::Armor => self.armor.clone(),
        }
    }
}

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
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct MobAssets {
    #[asset(key = "drone")]
    pub drone: Handle<TextureAtlas>,
    #[asset(key = "drone.thruster")]
    pub drone_thruster: Handle<TextureAtlas>,
    #[asset(key = "pawn")]
    pub pawn: Handle<TextureAtlas>,
    #[asset(key = "pawn.thruster")]
    pub pawn_thruster: Handle<TextureAtlas>,
    #[asset(key = "hauler")]
    pub hauler: Handle<TextureAtlas>,
    #[asset(key = "hauler.thruster")]
    pub hauler_thruster: Handle<TextureAtlas>,
    #[asset(key = "missile_launcher")]
    pub missile_launcher: Handle<TextureAtlas>,
    #[asset(key = "missile_launcher.thruster")]
    pub missile_launcher_thruster: Handle<TextureAtlas>,
    #[asset(key = "missile")]
    pub missile: Handle<TextureAtlas>,
    #[asset(key = "missile.thruster")]
    pub missile_thruster: Handle<TextureAtlas>,
    #[asset(key = "strafer")]
    pub strafer: Handle<TextureAtlas>,
    #[asset(key = "strafer.thruster")]
    pub strafer_thruster: Handle<TextureAtlas>,
    #[asset(key = "money_asteroid")]
    pub money_asteroid: Handle<TextureAtlas>,
}

impl MobAssets {
    pub fn get_mob_asset(&self, mob_type: &MobType) -> Handle<TextureAtlas> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyType::Pawn => self.pawn.clone(),
                EnemyType::Drone => self.drone.clone(),
                EnemyType::StraferRight => self.strafer.clone(),
                EnemyType::StraferLeft => self.strafer.clone(),
                EnemyType::MissileLauncher => self.missile_launcher.clone(),
                EnemyType::Missile => self.missile.clone(),
            },
            MobType::Ally(ally_type) => match ally_type {
                crate::spawnable::AllyType::Hauler => self.hauler.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                crate::spawnable::NeutralType::MoneyAsteroid => self.money_asteroid.clone(),
            },
        }
    }

    pub fn get_thruster_asset(&self, mob_type: &MobType) -> Option<Handle<TextureAtlas>> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyType::Pawn => Some(self.pawn_thruster.clone()),
                EnemyType::Drone => Some(self.drone_thruster.clone()),
                EnemyType::StraferRight => Some(self.strafer_thruster.clone()),
                EnemyType::StraferLeft => Some(self.strafer_thruster.clone()),
                EnemyType::MissileLauncher => Some(self.missile_launcher_thruster.clone()),
                EnemyType::Missile => Some(self.missile_thruster.clone()),
            },
            MobType::Ally(ally_type) => match ally_type {
                crate::spawnable::AllyType::Hauler => Some(self.hauler_thruster.clone()),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                crate::spawnable::NeutralType::MoneyAsteroid => None,
            },
        }
    }
}

#[derive(AssetCollection, Resource)]

pub struct GameAudioAssets {
    #[asset(key = "sounds.game_music")]
    pub game_music: Handle<AudioSource>,
    #[asset(key = "sounds.barrier_bounce")]
    pub barrier_bounce: Handle<AudioSource>,
    #[asset(key = "sounds.collision")]
    pub collision: Handle<AudioSource>,
    #[asset(key = "sounds.consumable_pickup")]
    pub consumable_pickup: Handle<AudioSource>,
    #[asset(key = "sounds.defense_damage")]
    pub defense_damage: Handle<AudioSource>,
    #[asset(key = "sounds.defense_heal")]
    pub defense_heal: Handle<AudioSource>,
    #[asset(key = "sounds.enemy_fire_blast")]
    pub enemy_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.menu_input_success")]
    pub menu_input_success: Handle<AudioSource>,
    #[asset(key = "sounds.mob_explosion")]
    pub mob_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.mob_hit")]
    pub mob_hit: Handle<AudioSource>,
    #[asset(key = "sounds.player_explosion")]
    pub player_explosion: Handle<AudioSource>,
    #[asset(key = "sounds.player_fire_blast")]
    pub player_fire_blast: Handle<AudioSource>,
    #[asset(key = "sounds.player_hit")]
    pub player_hit: Handle<AudioSource>,
}
