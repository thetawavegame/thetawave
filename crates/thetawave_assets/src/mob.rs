use bevy::{
    asset::Handle,
    prelude::{Image, Res, Resource},
    sprite::TextureAtlasLayout,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use thetawave_interface::spawnable::{
    AllyMobType, EnemyMobSegmentType, EnemyMobType, MobSegmentType, MobType, NeutralMobSegmentType,
    NeutralMobType,
};

/// Collection of texture atlases and images for mob and mob segment sprites
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
    #[asset(key = "mecha_ferritharax.head.layout")]
    pub mecha_ferritharax_head_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.head.image")]
    pub mecha_ferritharax_head_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.body.layout")]
    pub mecha_ferritharax_body_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.body.image")]
    pub mecha_ferritharax_body_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.right_shoulder.layout")]
    pub mecha_ferritharax_right_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.right_shoulder.image")]
    pub mecha_ferritharax_right_shoulder_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.left_shoulder.layout")]
    pub mecha_ferritharax_left_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.left_shoulder.image")]
    pub mecha_ferritharax_left_shoulder_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.right_arm.layout")]
    pub mecha_ferritharax_right_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.right_arm.image")]
    pub mecha_ferritharax_right_arm_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.left_arm.layout")]
    pub mecha_ferritharax_left_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.left_arm.image")]
    pub mecha_ferritharax_left_arm_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.right_claw.layout")]
    pub mecha_ferritharax_right_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.right_claw.image")]
    pub mecha_ferritharax_right_claw_image: Handle<Image>,
    #[asset(key = "mecha_ferritharax.left_claw.layout")]
    pub mecha_ferritharax_left_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_ferritharax.left_claw.image")]
    pub mecha_ferritharax_left_claw_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.head.layout")]
    pub mecha_saucetron_head_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.head.image")]
    pub mecha_saucetron_head_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.body.layout")]
    pub mecha_saucetron_body_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.body.image")]
    pub mecha_saucetron_body_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.right_shoulder.layout")]
    pub mecha_saucetron_right_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.right_shoulder.image")]
    pub mecha_saucetron_right_shoulder_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.left_shoulder.layout")]
    pub mecha_saucetron_left_shoulder_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.left_shoulder.image")]
    pub mecha_saucetron_left_shoulder_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.right_arm.layout")]
    pub mecha_saucetron_right_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.right_arm.image")]
    pub mecha_saucetron_right_arm_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.left_arm.layout")]
    pub mecha_saucetron_left_arm_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.left_arm.image")]
    pub mecha_saucetron_left_arm_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.right_claw.layout")]
    pub mecha_saucetron_right_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.right_claw.image")]
    pub mecha_saucetron_right_claw_image: Handle<Image>,
    #[asset(key = "mecha_saucetron.left_claw.layout")]
    pub mecha_saucetron_left_claw_layout: Handle<TextureAtlasLayout>,
    #[asset(key = "mecha_saucetron.left_claw.image")]
    pub mecha_saucetron_left_claw_image: Handle<Image>,
}

impl MobAssets {
    /// Use a MobType enum to access a texture atlas layout
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
                EnemyMobType::Ferritharax => self.repeater_head_layout.clone(),
                EnemyMobType::MechaFerritharax => self.mecha_ferritharax_head_layout.clone(),
                EnemyMobType::MechaSaucetron => self.mecha_saucetron_head_layout.clone(),
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

    /// Use a MobType enum to access an image handle
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
                EnemyMobType::Ferritharax => self.repeater_head_image.clone(),
                EnemyMobType::MechaFerritharax => self.mecha_ferritharax_head_image.clone(),
                EnemyMobType::MechaSaucetron => self.mecha_saucetron_head_image.clone(),
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

    /// Use a MobSegmentType enum to access a texture atlas layout
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
                EnemyMobSegmentType::FerritharaxBody => self.repeater_body_layout.clone(),
                EnemyMobSegmentType::FerritharaxRightShoulder => {
                    self.repeater_right_shoulder_layout.clone()
                }
                EnemyMobSegmentType::FerritharaxLeftShoulder => {
                    self.repeater_left_shoulder_layout.clone()
                }
                EnemyMobSegmentType::FerritharaxRightArm => self.repeater_right_arm_layout.clone(),
                EnemyMobSegmentType::FerritharaxLeftArm => self.repeater_left_arm_layout.clone(),
                EnemyMobSegmentType::FerritharaxRightClaw => {
                    self.repeater_right_claw_layout.clone()
                }
                EnemyMobSegmentType::FerritharaxLeftClaw => self.repeater_left_claw_layout.clone(),
                EnemyMobSegmentType::MechaFerritharaxBody => {
                    self.mecha_ferritharax_body_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightShoulder => {
                    self.mecha_ferritharax_right_shoulder_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftShoulder => {
                    self.mecha_ferritharax_left_shoulder_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightArm => {
                    self.mecha_ferritharax_right_arm_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftArm => {
                    self.mecha_ferritharax_left_arm_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightClaw => {
                    self.mecha_ferritharax_right_claw_layout.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftClaw => {
                    self.mecha_ferritharax_left_claw_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronBody => self.mecha_saucetron_body_layout.clone(),
                EnemyMobSegmentType::MechaSaucetronRightShoulder => {
                    self.mecha_saucetron_right_shoulder_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftShoulder => {
                    self.mecha_saucetron_left_shoulder_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronRightArm => {
                    self.mecha_saucetron_right_arm_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftArm => {
                    self.mecha_saucetron_left_arm_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronRightClaw => {
                    self.mecha_saucetron_right_claw_layout.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftClaw => {
                    self.mecha_saucetron_left_claw_layout.clone()
                }
            },
        }
    }

    /// Use a MobSegmentType enum to access an image handle
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
                EnemyMobSegmentType::FerritharaxBody => self.repeater_body_image.clone(),
                EnemyMobSegmentType::FerritharaxRightShoulder => {
                    self.repeater_right_shoulder_image.clone()
                }
                EnemyMobSegmentType::FerritharaxLeftShoulder => {
                    self.repeater_left_shoulder_image.clone()
                }
                EnemyMobSegmentType::FerritharaxRightArm => self.repeater_right_arm_image.clone(),
                EnemyMobSegmentType::FerritharaxLeftArm => self.repeater_left_arm_image.clone(),
                EnemyMobSegmentType::FerritharaxRightClaw => self.repeater_right_claw_image.clone(),
                EnemyMobSegmentType::FerritharaxLeftClaw => self.repeater_left_claw_image.clone(),
                EnemyMobSegmentType::MechaFerritharaxBody => {
                    self.mecha_ferritharax_body_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightShoulder => {
                    self.mecha_ferritharax_right_shoulder_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftShoulder => {
                    self.mecha_ferritharax_left_shoulder_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightArm => {
                    self.mecha_ferritharax_right_arm_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftArm => {
                    self.mecha_ferritharax_left_arm_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxRightClaw => {
                    self.mecha_ferritharax_right_claw_image.clone()
                }
                EnemyMobSegmentType::MechaFerritharaxLeftClaw => {
                    self.mecha_ferritharax_left_claw_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronBody => self.mecha_saucetron_body_image.clone(),
                EnemyMobSegmentType::MechaSaucetronRightShoulder => {
                    self.mecha_saucetron_right_shoulder_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftShoulder => {
                    self.mecha_saucetron_left_shoulder_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronRightArm => {
                    self.mecha_saucetron_right_arm_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftArm => {
                    self.mecha_saucetron_left_arm_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronRightClaw => {
                    self.mecha_saucetron_right_claw_image.clone()
                }
                EnemyMobSegmentType::MechaSaucetronLeftClaw => {
                    self.mecha_saucetron_left_claw_image.clone()
                }
            },
        }
    }

    /// Use a MobType enum to access its associated thruster's texture atlas layout
    /// Returns an option due to some mobs not having an thruster
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
                EnemyMobType::Ferritharax => None,
                EnemyMobType::MechaFerritharax => None,
                EnemyMobType::MechaSaucetron => None,
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

    /// Use a MobType enum to access its associated thruster's image
    /// Returns an option due to some mobs not having an thruster
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
                EnemyMobType::Ferritharax => None,
                EnemyMobType::MechaFerritharax => None,
                EnemyMobType::MechaSaucetron => None,
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
}
