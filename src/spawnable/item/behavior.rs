use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub enum ItemBehavior {
    ApplyEffectsOnImpact,
    AttractToPlayer,
}
