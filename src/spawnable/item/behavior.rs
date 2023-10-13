use serde::Deserialize;

#[derive(Deserialize)]
pub enum ItemBehavior {
    ApplyEffectsOnImpact,
    AttractToPlayer,
}
