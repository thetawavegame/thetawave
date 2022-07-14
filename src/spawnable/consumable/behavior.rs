use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum ConsumableBehavior {
    ApplyEffectsOnImpact,
}
