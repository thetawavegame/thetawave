use bevy_ecs_macros::Resource;
use bevy_log::error;
use derive_more::{Deref, DerefMut, From};
use std::default::Default;
use std::env;
use std::path::{Path, PathBuf};
pub const BACKGROUND_ASSETS_BASE_PATH_ENV_VAR_NAME: &str = "THETAWAVE_BACKGROUND_ASSETS_PATH";
/// A collection of file names/paths for individual background image assets to be lazily loaded.
/// These are fit to be used with bevy's AssetServer::load. Many architectures will support
/// bevy::asset::AssetIO::read_directory . But wasm32 does not support it. So we have a fallback
/// that can be populated by other systems. Generally this will be written to once and never
/// updated again.
#[derive(Resource, Deref, DerefMut, Default, From)]
pub struct BackupBackgroundAssetPaths(pub Vec<String>);

impl BackupBackgroundAssetPaths {
    /// Return the string names of files at the base path, relative to that base path.
    pub fn from_local_base_path(base_path: &Path) -> Self {
        let fnames = match std::fs::read_dir(base_path) {
            Err(err) => {
                error!(
                    "Failed to read background asset base path. {} . Error: {}",
                    base_path.to_string_lossy(),
                    err
                );
                Default::default()
            }
            Ok(dir) => dir
                .into_iter()
                .filter_map(|path| match &path {
                    Ok(path_) if path_.path().exists() => {
                        Some(path_.path().into_os_string().to_string_lossy().into_owned())
                    }
                    Ok(_) => None,
                    Err(err) => {
                        error!(
                            "Failed to read path {:?} base_path={}, Err: {}",
                            &path,
                            base_path.to_string_lossy(),
                            err
                        );
                        None
                    }
                })
                .collect::<Vec<String>>(),
        };
        fnames.into()
    }
    /// Return the string names of files at the base path (input as a string), relative to that base path.
    pub fn from_raw_path(raw_path: String) -> Option<Self> {
        Some(Self::from_local_base_path(&PathBuf::from(raw_path)))
    }
    pub fn from_env_and_local_fs() -> Option<Self> {
        let raw_path = match env::var(BACKGROUND_ASSETS_BASE_PATH_ENV_VAR_NAME) {
            Err(e) => {
                error!(
                        "Failed to read environment variable {}. This is ok if we are running in the browser. {}",
                        BACKGROUND_ASSETS_BASE_PATH_ENV_VAR_NAME,
                        e
                    );
                None
            }
            Ok(val) => Some(val),
        }?;
        Self::from_raw_path(raw_path)
    }
}
