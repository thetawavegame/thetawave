use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use thetawave_interface::spawnable::{
    AllyMobType, EnemyMobType, MobSegmentType, MobType, NeutralMobType,
};

#[derive(AssetCollection, Resource)]
pub struct MobAssets {
    #[asset(key = "tutorial_drone")]
    pub tutorial_drone: Handle<TextureAtlas>,
    #[asset(key = "shelly")]
    pub shelly: Handle<TextureAtlas>,
    #[asset(key = "drone")]
    pub drone: Handle<TextureAtlas>,
    #[asset(key = "drone.thruster")]
    pub drone_thruster: Handle<TextureAtlas>,
    #[asset(key = "pawn")]
    pub pawn: Handle<TextureAtlas>,
    #[asset(key = "pawn.thruster")]
    pub pawn_thruster: Handle<TextureAtlas>,
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
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => self.strafer.clone(),
                EnemyMobType::MissileLauncher => self.missile_launcher.clone(),
                EnemyMobType::Missile => self.missile.clone(),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => {
                    self.crustling_head.clone()
                }
                EnemyMobType::Repeater => self.repeater_head.clone(),
                EnemyMobType::Shelly => self.shelly.clone(),
            },
            MobType::Ally(ally_type) => match ally_type {
                thetawave_interface::spawnable::AllyMobType::Hauler2 => self.hauler_front.clone(),
                thetawave_interface::spawnable::AllyMobType::Hauler3 => self.hauler_front.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                thetawave_interface::spawnable::NeutralMobType::MoneyAsteroid => {
                    self.money_asteroid.clone()
                }
                thetawave_interface::spawnable::NeutralMobType::TutorialDrone => {
                    self.tutorial_drone.clone()
                }
            },
        }
    }

    pub fn get_mob_segment_asset(&self, mob_segment_type: &MobSegmentType) -> Handle<TextureAtlas> {
        match mob_segment_type {
            MobSegmentType::Neutral(neutral_type) => match neutral_type {
                thetawave_interface::spawnable::NeutralMobSegmentType::HaulerBack => {
                    self.hauler_back.clone()
                }
                thetawave_interface::spawnable::NeutralMobSegmentType::HaulerMiddle => {
                    self.hauler_middle.clone()
                }
            },
            MobSegmentType::Enemy(enemy_type) => match enemy_type {
                thetawave_interface::spawnable::EnemyMobSegmentType::CrustlingTentacle1 => {
                    self.crustling_tentacle1.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::CrustlingTentacle2 => {
                    self.crustling_tentacle2.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::CrustlingTentacle3 => {
                    self.crustling_tentacle3.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterBody => {
                    self.repeater_body.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterRightShoulder => {
                    self.repeater_right_shoulder.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterLeftShoulder => {
                    self.repeater_left_shoulder.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterRightArm => {
                    self.repeater_right_arm.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterLeftArm => {
                    self.repeater_left_arm.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterRightClaw => {
                    self.repeater_right_claw.clone()
                }
                thetawave_interface::spawnable::EnemyMobSegmentType::RepeaterLeftClaw => {
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
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    Some(self.strafer_thruster.clone())
                }
                EnemyMobType::MissileLauncher => Some(self.missile_launcher_thruster.clone()),
                EnemyMobType::Missile => Some(self.missile_thruster.clone()),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => None,
                EnemyMobType::Repeater => None,
                EnemyMobType::Shelly => None,
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::Hauler2 => Some(self.hauler_thruster.clone()),
                AllyMobType::Hauler3 => Some(self.hauler_thruster.clone()),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => None,
                NeutralMobType::TutorialDrone => None,
            },
        }
    }

    pub fn get_thruster_color(&self, mob_type: &MobType) -> Color {
        match mob_type {
            MobType::Enemy(_) => Color::rgb(3.8, 2.2, 1.0),
            MobType::Ally(_) => Color::rgb(3.8, 2.2, 1.0),
            MobType::Neutral(_) => Color::rgb(3.8, 2.2, 1.0),
        }
    }
}
