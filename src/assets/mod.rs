use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::{
    player::{Character, CharacterType},
    spawnable::{
        ConsumableType, EffectType, EnemyMobType, MobSegmentType, MobType, ProjectileType,
    },
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
    #[asset(key = "hauler.front")]
    pub hauler_front: Handle<TextureAtlas>,
    #[asset(key = "hauler.back")]
    pub hauler_back: Handle<TextureAtlas>,
    #[asset(key = "hauler.middle")]
    pub hauler_middle: Handle<TextureAtlas>,
    #[asset(key = "crustling.head")]
    pub crustling_head: Handle<TextureAtlas>,
    #[asset(key = "crustling.tentacle1")]
    pub crustling_tentacle1: Handle<TextureAtlas>,
    #[asset(key = "crustling.tentacle2")]
    pub crustling_tentacle2: Handle<TextureAtlas>,
    #[asset(key = "crustling.tentacle3")]
    pub crustling_tentacle3: Handle<TextureAtlas>,
    #[asset(key = "repeater.head")]
    pub repeater_head: Handle<TextureAtlas>,
    #[asset(key = "repeater.body")]
    pub repeater_body: Handle<TextureAtlas>,
    #[asset(key = "repeater.right_shoulder")]
    pub repeater_right_shoulder: Handle<TextureAtlas>,
    #[asset(key = "repeater.left_shoulder")]
    pub repeater_left_shoulder: Handle<TextureAtlas>,
    #[asset(key = "repeater.right_arm")]
    pub repeater_right_arm: Handle<TextureAtlas>,
    #[asset(key = "repeater.left_arm")]
    pub repeater_left_arm: Handle<TextureAtlas>,
    #[asset(key = "repeater.right_claw")]
    pub repeater_right_claw: Handle<TextureAtlas>,
    #[asset(key = "repeater.left_claw")]
    pub repeater_left_claw: Handle<TextureAtlas>,
}

impl MobAssets {
    pub fn get_mob_asset(&self, mob_type: &MobType) -> Handle<TextureAtlas> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => self.pawn.clone(),
                EnemyMobType::Drone => self.drone.clone(),
                EnemyMobType::StraferRight => self.strafer.clone(),
                EnemyMobType::StraferLeft => self.strafer.clone(),
                EnemyMobType::MissileLauncher => self.missile_launcher.clone(),
                EnemyMobType::Missile => self.missile.clone(),
                EnemyMobType::Crustling => self.crustling_head.clone(),
                EnemyMobType::Repeater => self.repeater_head.clone(),
            },
            MobType::Ally(ally_type) => match ally_type {
                crate::spawnable::AllyMobType::Hauler2 => self.hauler_front.clone(),
                crate::spawnable::AllyMobType::Hauler3 => self.hauler_front.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                crate::spawnable::NeutralMobType::MoneyAsteroid => self.money_asteroid.clone(),
            },
        }
    }

    pub fn get_mob_segment_asset(&self, mob_segment_type: &MobSegmentType) -> Handle<TextureAtlas> {
        match mob_segment_type {
            MobSegmentType::Neutral(neutral_type) => match neutral_type {
                crate::spawnable::NeutralMobSegmentType::HaulerBack => self.hauler_back.clone(),
                crate::spawnable::NeutralMobSegmentType::HaulerMiddle => self.hauler_middle.clone(),
            },
            MobSegmentType::Enemy(enemy_type) => match enemy_type {
                crate::spawnable::EnemyMobSegmentType::CrustlingTentacle1 => {
                    self.crustling_tentacle1.clone()
                }
                crate::spawnable::EnemyMobSegmentType::CrustlingTentacle2 => {
                    self.crustling_tentacle2.clone()
                }
                crate::spawnable::EnemyMobSegmentType::CrustlingTentacle3 => {
                    self.crustling_tentacle3.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterBody => self.repeater_body.clone(),
                crate::spawnable::EnemyMobSegmentType::RepeaterRightShoulder => {
                    self.repeater_right_shoulder.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterLeftShoulder => {
                    self.repeater_left_shoulder.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterRightArm => {
                    self.repeater_right_arm.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterLeftArm => {
                    self.repeater_left_arm.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterRightClaw => {
                    self.repeater_right_claw.clone()
                }
                crate::spawnable::EnemyMobSegmentType::RepeaterLeftClaw => {
                    self.repeater_left_claw.clone()
                }
            },
        }
    }

    pub fn get_thruster_asset(&self, mob_type: &MobType) -> Option<Handle<TextureAtlas>> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => Some(self.pawn_thruster.clone()),
                EnemyMobType::Drone => Some(self.drone_thruster.clone()),
                EnemyMobType::StraferRight => Some(self.strafer_thruster.clone()),
                EnemyMobType::StraferLeft => Some(self.strafer_thruster.clone()),
                EnemyMobType::MissileLauncher => Some(self.missile_launcher_thruster.clone()),
                EnemyMobType::Missile => Some(self.missile_thruster.clone()),
                EnemyMobType::Crustling => None,
                EnemyMobType::Repeater => None,
            },
            MobType::Ally(ally_type) => match ally_type {
                crate::spawnable::AllyMobType::Hauler2 => Some(self.hauler_thruster.clone()),
                crate::spawnable::AllyMobType::Hauler3 => Some(self.hauler_thruster.clone()),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                crate::spawnable::NeutralMobType::MoneyAsteroid => None,
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
