use bevy::prelude::*;
use serde::Deserialize;

/// Types of behaviors that can be performed by mobs
#[derive(Deserialize, Clone)]
pub enum MobSegmentBehavior {
    DealDamageToPlayerOnImpact,
    ReceiveDamageOnImpact,
    DieAtZeroHealth,
}
