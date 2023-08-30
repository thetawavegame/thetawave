pub mod public_s3_assets;
mod s3;

use crate::public_s3_assets::PublicS3AssetsPlugin;
use bevy_app::{App, Plugin, PluginGroup, PluginGroupBuilder};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/src/fetch_url_with_timeout.js")]
extern "C" {
    async fn fetchWithTimeout(url: String, timeoutMs: u32) -> JsValue;
}

pub struct RedirectPanicsToBrowserConsoleLogPlugin;
impl Plugin for RedirectPanicsToBrowserConsoleLogPlugin {
    fn build(&self, _app: &mut App) {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
}

pub struct WebWasmSpecificPlugins;
impl PluginGroup for WebWasmSpecificPlugins {
    fn build(self) -> PluginGroupBuilder {
        self.set(RedirectPanicsToBrowserConsoleLogPlugin)
            .set(PublicS3AssetsPlugin)
    }
}
