use bevy::{
    prelude::{Color, Handle, Resource, TextureAtlasLayout},
    render::texture::Image,
};
use bevy_asset_loader::prelude::AssetCollection;
use thetawave_interface::spawnable::{
    AllyMobType, EnemyMobSegmentType, EnemyMobType, MobSegmentType, MobType, NeutralMobSegmentType,
    NeutralMobType,
};

#[derive(AssetCollection, Resource)]
pub struct MobAssets {
    #[asset(key = "tutorial_drone.layout")]
    pub tutorial_drone_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "tutorial_drone.image")]
    pub tutorial_drone_image: Handle<Image>,
    #[asset(key = "shelly.layout")]
    pub shelly_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "shelly.image")]
    pub shelly_image: Handle<Image>,
    #[asset(key = "drone.layout")]
    pub drone_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "drone.image")]
    pub drone_image: Handle<Image>,
    #[asset(key = "drone.thruster.layout")]
    pub drone_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "drone.thruster.image")]
    pub drone_thruster_image: Handle<Image>,
    #[asset(key = "pawn.layout")]
    pub pawn_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "pawn.image")]
    pub pawn_image: Handle<Image>,
    #[asset(key = "pawn.thruster.layout")]
    pub pawn_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "pawn.thruster.image")]
    pub pawn_thruster_image: Handle<Image>,
    #[asset(key = "hauler.thruster.layout")]
    pub hauler_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "hauler.thruster.image")]
    pub hauler_thruster_image: Handle<Image>,
    #[asset(key = "missile_launcher.layout")]
    pub missile_launcher_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "missile_launcher.image")]
    pub missile_launcher_image: Handle<Image>,
    #[asset(key = "missile_launcher.thruster.layout")]
    pub missile_launcher_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "missile_launcher.thruster.image")]
    pub missile_launcher_thruster_image: Handle<Image>,
    #[asset(key = "missile.layout")]
    pub missile_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "missile.image")]
    pub missile_image: Handle<Image>,
    #[asset(key = "missile.thruster.layout")]
    pub missile_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "missile.thruster.image")]
    pub missile_thruster_image: Handle<Image>,
    #[asset(key = "strafer.layout")]
    pub strafer_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "strafer.image")]
    pub strafer_image: Handle<Image>,
    #[asset(key = "strafer.thruster.layout")]
    pub strafer_thruster_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "strafer.thruster.image")]
    pub strafer_thruster_image: Handle<Image>,
    #[asset(key = "money_asteroid.layout")]
    pub money_asteroid_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "money_asteroid.image")]
    pub money_asteroid_image: Handle<Image>,
    #[asset(key = "hauler.front.layout")]
    pub hauler_front_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "hauler.front.image")]
    pub hauler_front_image: Handle<Image>,
    #[asset(key = "hauler.back.layout")]
    pub hauler_back_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "hauler.back.image")]
    pub hauler_back_image: Handle<Image>,
    #[asset(key = "hauler.middle.layout")]
    pub hauler_middle_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "hauler.middle.image")]
    pub hauler_middle_image: Handle<Image>,
    #[asset(key = "crustling.head.layout")]
    pub crustling_head_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "crustling.head.image")]
    pub crustling_head_image: Handle<Image>,
    #[asset(key = "crustling.tentacle1.layout")]
    pub crustling_tentacle1_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "crustling.tentacle1.image")]
    pub crustling_tentacle1_image: Handle<Image>,
    #[asset(key = "crustling.tentacle2.layout")]
    pub crustling_tentacle2_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "crustling.tentacle2.image")]
    pub crustling_tentacle2_image: Handle<Image>,
    #[asset(key = "crustling.tentacle3.layout")]
    pub crustling_tentacle3_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "crustling.tentacle3.image")]
    pub crustling_tentacle3_image: Handle<Image>,
    #[asset(key = "repeater.head.layout")]
    pub repeater_head_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.head.image")]
    pub repeater_head_image: Handle<Image>,
    #[asset(key = "repeater.body.layout")]
    pub repeater_body_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.body.image")]
    pub repeater_body_image: Handle<Image>,
    #[asset(key = "repeater.right_shoulder.layout")]
    pub repeater_right_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.right_shoulder.image")]
    pub repeater_right_shoulder_image: Handle<Image>,
    #[asset(key = "repeater.left_shoulder.layout")]
    pub repeater_left_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.left_shoulder.image")]
    pub repeater_left_shoulder_image: Handle<Image>,
    #[asset(key = "repeater.right_arm.layout")]
    pub repeater_right_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.right_arm.image")]
    pub repeater_right_arm_image: Handle<Image>,
    #[asset(key = "repeater.left_arm.layout")]
    pub repeater_left_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.left_arm.image")]
    pub repeater_left_arm_image: Handle<Image>,
    #[asset(key = "repeater.right_claw.layout")]
    pub repeater_right_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.right_claw.image")]
    pub repeater_right_claw_image: Handle<Image>,
    #[asset(key = "repeater.left_claw.layout")]
    pub repeater_left_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "repeater.left_claw.image")]
    pub repeater_left_claw_image: Handle<Image>,
}

impl MobAssets {
    pub fn get_mob_texture_atlas_layout(&self, mob_type: &MobType) -> Handle<TextureAtlasLayout> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => self.pawn_layout.clone(),
                EnemyMobType::Drone => self.drone_layout.clone(),
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    self.strafer_layout.clone()
                }
                EnemyMobType::MissileLauncher => self.missile_launcher_layout.clone(),
                EnemyMobType::Missile => self.missile_layout.clone(),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => {
                    self.crustling_head_layout.clone()
                }
                EnemyMobType::Repeater => self.repeater_head_layout.clone(),
                EnemyMobType::Shelly => self.shelly_layout.clone(),
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::TutorialHauler2 => self.hauler_front_layout.clone(),
                AllyMobType::Hauler2 => self.hauler_front_layout.clone(),
                AllyMobType::Hauler3 => self.hauler_front_layout.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => self.money_asteroid_layout.clone(),
                NeutralMobType::TutorialDrone => self.tutorial_drone_layout.clone(),
            },
        }
    }

    pub fn get_mob_image(&self, mob_type: &MobType) -> Handle<Image> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => self.pawn_image.clone(),
                EnemyMobType::Drone => self.drone_image.clone(),
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    self.strafer_image.clone()
                }
                EnemyMobType::MissileLauncher => self.missile_launcher_image.clone(),
                EnemyMobType::Missile => self.missile_image.clone(),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => {
                    self.crustling_head_image.clone()
                }
                EnemyMobType::Repeater => self.repeater_head_image.clone(),
                EnemyMobType::Shelly => self.shelly_image.clone(),
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::TutorialHauler2 => self.hauler_front_image.clone(),
                AllyMobType::Hauler2 => self.hauler_front_image.clone(),
                AllyMobType::Hauler3 => self.hauler_front_image.clone(),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => self.money_asteroid_image.clone(),
                NeutralMobType::TutorialDrone => self.tutorial_drone_image.clone(),
            },
        }
    }

    pub fn get_mob_segment_texture_atlas_layout(
        &self,
        mob_segment_type: &MobSegmentType,
    ) -> Handle<TextureAtlasLayout> {
        match mob_segment_type {
            MobSegmentType::Neutral(neutral_type) => match neutral_type {
                NeutralMobSegmentType::HaulerBack => self.hauler_back_layout.clone(),
                NeutralMobSegmentType::HaulerMiddle => self.hauler_middle_layout.clone(),
                NeutralMobSegmentType::TutorialHaulerBack => self.hauler_back_layout.clone(),
            },
            MobSegmentType::Enemy(enemy_type) => match enemy_type {
                EnemyMobSegmentType::CrustlingTentacle1 => self.crustling_tentacle1_layout.clone(),
                EnemyMobSegmentType::CrustlingTentacle2 => self.crustling_tentacle2_layout.clone(),
                EnemyMobSegmentType::CrustlingTentacle3 => self.crustling_tentacle3_layout.clone(),
                EnemyMobSegmentType::RepeaterBody => self.repeater_body_layout.clone(),
                EnemyMobSegmentType::RepeaterRightShoulder => {
                    self.repeater_right_shoulder_layout.clone()
                }
                EnemyMobSegmentType::RepeaterLeftShoulder => {
                    self.repeater_left_shoulder_layout.clone()
                }
                EnemyMobSegmentType::RepeaterRightArm => self.repeater_right_arm_layout.clone(),
                EnemyMobSegmentType::RepeaterLeftArm => self.repeater_left_arm_layout.clone(),
                EnemyMobSegmentType::RepeaterRightClaw => self.repeater_right_claw_layout.clone(),
                EnemyMobSegmentType::RepeaterLeftClaw => self.repeater_left_claw_layout.clone(),
            },
        }
    }

    pub fn get_mob_segment_image(&self, mob_segment_type: &MobSegmentType) -> Handle<Image> {
        match mob_segment_type {
            MobSegmentType::Neutral(neutral_type) => match neutral_type {
                NeutralMobSegmentType::HaulerBack => self.hauler_back_image.clone(),
                NeutralMobSegmentType::HaulerMiddle => self.hauler_middle_image.clone(),
                NeutralMobSegmentType::TutorialHaulerBack => self.hauler_back_image.clone(),
            },
            MobSegmentType::Enemy(enemy_type) => match enemy_type {
                EnemyMobSegmentType::CrustlingTentacle1 => self.crustling_tentacle1_image.clone(),
                EnemyMobSegmentType::CrustlingTentacle2 => self.crustling_tentacle2_image.clone(),
                EnemyMobSegmentType::CrustlingTentacle3 => self.crustling_tentacle3_image.clone(),
                EnemyMobSegmentType::RepeaterBody => self.repeater_body_image.clone(),
                EnemyMobSegmentType::RepeaterRightShoulder => {
                    self.repeater_right_shoulder_image.clone()
                }
                EnemyMobSegmentType::RepeaterLeftShoulder => {
                    self.repeater_left_shoulder_image.clone()
                }
                EnemyMobSegmentType::RepeaterRightArm => self.repeater_right_arm_image.clone(),
                EnemyMobSegmentType::RepeaterLeftArm => self.repeater_left_arm_image.clone(),
                EnemyMobSegmentType::RepeaterRightClaw => self.repeater_right_claw_image.clone(),
                EnemyMobSegmentType::RepeaterLeftClaw => self.repeater_left_claw_image.clone(),
            },
        }
    }

    pub fn get_thruster_texture_atlas_layout(
        &self,
        mob_type: &MobType,
    ) -> Option<Handle<TextureAtlasLayout>> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => Some(self.pawn_thruster_layout.clone()),
                EnemyMobType::Drone => Some(self.drone_thruster_layout.clone()),
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    Some(self.strafer_thruster_layout.clone())
                }
                EnemyMobType::MissileLauncher => {
                    Some(self.missile_launcher_thruster_layout.clone())
                }
                EnemyMobType::Missile => Some(self.missile_thruster_layout.clone()),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => None,
                EnemyMobType::Repeater => None,
                EnemyMobType::Shelly => None,
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::TutorialHauler2 => Some(self.hauler_thruster_layout.clone()),
                AllyMobType::Hauler2 => Some(self.hauler_thruster_layout.clone()),
                AllyMobType::Hauler3 => Some(self.hauler_thruster_layout.clone()),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => None,
                NeutralMobType::TutorialDrone => None,
            },
        }
    }

    pub fn get_thruster_image(&self, mob_type: &MobType) -> Option<Handle<Image>> {
        match mob_type {
            MobType::Enemy(enemy_type) => match enemy_type {
                EnemyMobType::Pawn => Some(self.pawn_thruster_image.clone()),
                EnemyMobType::Drone => Some(self.drone_thruster_image.clone()),
                EnemyMobType::StraferRight | EnemyMobType::StraferLeft => {
                    Some(self.strafer_thruster_image.clone())
                }
                EnemyMobType::MissileLauncher => Some(self.missile_launcher_thruster_image.clone()),
                EnemyMobType::Missile => Some(self.missile_thruster_image.clone()),
                EnemyMobType::CrustlingRight | EnemyMobType::CrustlingLeft => None,
                EnemyMobType::Repeater => None,
                EnemyMobType::Shelly => None,
            },
            MobType::Ally(ally_type) => match ally_type {
                AllyMobType::TutorialHauler2 => Some(self.hauler_thruster_image.clone()),
                AllyMobType::Hauler2 => Some(self.hauler_thruster_image.clone()),
                AllyMobType::Hauler3 => Some(self.hauler_thruster_image.clone()),
            },
            MobType::Neutral(neutral_type) => match neutral_type {
                NeutralMobType::MoneyAsteroid => None,
                NeutralMobType::TutorialDrone => None,
            },
        }
    }

    pub fn get_thruster_color(&self, mob_type: &MobType, bloom_intensity: f32) -> Color {
        match mob_type {
            MobType::Enemy(_) => Color::rgb(
                1.0 + 2.8 * bloom_intensity,
                1.0 + 1.2 * bloom_intensity,
                1.0,
            ),
            MobType::Ally(_) => Color::rgb(
                1.0 + 2.8 * bloom_intensity,
                1.0 + 1.2 * bloom_intensity,
                1.0,
            ),
            MobType::Neutral(_) => Color::rgb(
                1.0 + 2.8 * bloom_intensity,
                1.0 + 1.2 * bloom_intensity,
                1.0,
            ),
        }
    }
}
