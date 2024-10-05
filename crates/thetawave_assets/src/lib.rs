//! Exposes `AssetCollection`s with handles to files in the `assets/` directory at the base of the
//! repo. These are typically all loaded into memory when the game launches.
mod audio;
mod consumable;
mod effect;
mod item;
mod mob;
mod player;
mod projectile;
mod ui;

/*
pub(crate) use self::{
    audio::GameAudioAssets, consumable::ConsumableAssets, effect::EffectAssets, item::ItemAssets,
    mob::MobAssets, player::PlayerAssets, projectile::ProjectileAssets, ui::UiAssets,
};
*/

pub use audio::GameAudioAssets;
pub use consumable::ConsumableAssets;
pub use effect::EffectAssets;
pub use item::ItemAssets;
pub use mob::MobAssets;
pub use player::PlayerAssets;
pub use projectile::ProjectileAssets;
pub use ui::UiAssets;
