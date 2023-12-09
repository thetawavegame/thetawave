use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use thetawave_interface::spawnable::{
    AllyMobType, EnemyMobSegmentType, EnemyMobType, MobSegmentType, MobType, NeutralMobSegmentType,
    NeutralMobType,
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
    #[asset(key = "enemy_cargo_ship")]
    pub enemy_cargo_ship: Handle<TextureAtlas>,
    #[asset(key = "enemy_cargo_ship.thruster")]
    pub enemy_cargo_ship_thruster: Handle<TextureAtlas>,
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
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::Hauler2 => self.hauler_front.clone(),
                AllyMobType::Hauler3 => self.hauler_front.clone(),
                AllyMobType::TutorialHauler2 => self.hauler_front.clone(),
            },
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => {
                    self.crustling_head.clone()
                }
                EnemyMobType::Drone => self.drone.clone(),
                EnemyMobType::EnemyCargoShip => self.enemy_cargo_ship.clone(),
                EnemyMobType::MissileLauncher => self.missile_launcher.clone(),
                EnemyMobType::Missile => self.missile.clone(),
                EnemyMobType::Pawn => self.pawn.clone(),
                EnemyMobType::Repeater => self.repeater_head.clone(),
                EnemyMobType::Shelly => self.shelly.clone(),
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => self.strafer.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => self.money_asteroid.clone(),
                NeutralMobType::TutorialDrone => self.tutorial_drone.clone(),
            },
        }
    }

    pub fn get_mob_segment_asset(&self, mob_segment_type: &MobSegmentType) -> Handle<TextureAtlas> {
        match mob_segment_type {
            MobSegmentType::Neutral(neutral_type) => match neutral_type {
                NeutralMobSegmentType::HaulerBack => self.hauler_back.clone(),
                NeutralMobSegmentType::HaulerMiddle => self.hauler_middle.clone(),
                NeutralMobSegmentType::TutorialHaulerBack => self.hauler_back.clone(),
            },
            MobSegmentType::Enemy(enemy_type) => match enemy_type {
                EnemyMobSegmentType::CrustlingTentacle1 => self.crustling_tentacle1.clone(),
                EnemyMobSegmentType::CrustlingTentacle2 => self.crustling_tentacle2.clone(),
                EnemyMobSegmentType::CrustlingTentacle3 => self.crustling_tentacle3.clone(),
                EnemyMobSegmentType::RepeaterBody => self.repeater_body.clone(),
                EnemyMobSegmentType::RepeaterRightShoulder => self.repeater_right_shoulder.clone(),
                EnemyMobSegmentType::RepeaterLeftShoulder => self.repeater_left_shoulder.clone(),
                EnemyMobSegmentType::RepeaterRightArm => self.repeater_right_arm.clone(),
                EnemyMobSegmentType::RepeaterLeftArm => self.repeater_left_arm.clone(),
                EnemyMobSegmentType::RepeaterRightClaw => self.repeater_right_claw.clone(),
                EnemyMobSegmentType::RepeaterLeftClaw => self.repeater_left_claw.clone(),
            },
        }
    }

    pub fn get_thruster_asset(&self, mob_type: &MobType) -> Option<Handle<TextureAtlas>> {
        match mob_type {
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::TutorialHauler2 => Some(self.hauler_thruster.clone()),
                AllyMobType::Hauler2 => Some(self.hauler_thruster.clone()),
                AllyMobType::Hauler3 => Some(self.hauler_thruster.clone()),
            },
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => None,
                EnemyMobType::Drone => Some(self.drone_thruster.clone()),
                EnemyMobType::EnemyCargoShip => Some(self.enemy_cargo_ship_thruster.clone()),
                EnemyMobType::MissileLauncher => Some(self.missile_launcher_thruster.clone()),
                EnemyMobType::Missile => Some(self.missile_thruster.clone()),
                EnemyMobType::Repeater => None,
                EnemyMobType::Pawn => Some(self.pawn_thruster.clone()),
                EnemyMobType::Shelly => None,
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    Some(self.strafer_thruster.clone())
                }
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
