use super::raw_list_public_s3_bucket;
use async_channel::{bounded, Receiver, Sender};
use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::{in_state, Condition, IntoSystemConfigs, OnEnter, Resource};
use bevy_ecs::system::{Res, ResMut};
use bevy_log::{error, info};
use derive_more::{Deref, DerefMut, From};
use thetawave_interface::assets::BackupBackgroundAssetPaths;
use thetawave_interface::states::AppStates;
use wasm_bindgen_futures::spawn_local;

/// Since Bevy doesn't handle async things so well (each system function ultimately blocks the next
/// frame), we spawn a single-element queue with these ends, then populate a resource as part of
/// the public API used by other plugins.
#[derive(Resource, Deref, DerefMut, From)]
struct FetchS3FileNamesTask(Receiver<Vec<String>>);

#[derive(Resource, Deref, DerefMut, Clone, From)]
struct DownloadedS3FileNamesChannelInput(Sender<Vec<String>>);

#[derive(Resource, Debug, Clone)]
pub struct RemoteAssetFetchOptions {
    /// The base URL of the S3-compatible bucket/service that can support ListBucketV2 requests.
    pub s3_bucket_base_url: String,
    pub timeout_ms: u32,
}
/// Populates `BackupBackgroundAssetPaths` with strings that can be sent to
/// `bevy_asset::AssetServer::load`. These will use an S3-compatible service behind some CDN+DNS
/// magic external to the program. This plugin has to assume some amount about the deployment
/// environment and how the S3+AssetServer routing works.
pub struct PublicS3AssetsPlugin;

fn start_listing_s3_files(
    remote_asset_options: Res<RemoteAssetFetchOptions>,
    got_s3_filenames_sender: Res<DownloadedS3FileNamesChannelInput>,
) {
    info!("Starting to list the s3 remote assets");
    let RemoteAssetFetchOptions {
        s3_bucket_base_url,
        timeout_ms,
        ..
    } = remote_asset_options.clone();
    let tx = (*got_s3_filenames_sender).clone();
    spawn_local(async move {
        let free_background_assets_url = s3_bucket_base_url + "?prefix=free_assets/backgrounds";
        let got_fnames =
            raw_list_public_s3_bucket(free_background_assets_url.as_ref(), timeout_ms).await;
        info!("Downloaded S3 filenames. Putting then on the queue");
        tx.try_send(got_fnames).unwrap_or_else(|_| {
            error!("Queue full. too many HTTP requests");
        });
        tx.close();
    });
}
/// We don't have an event loop right now in the game, so we have no choice but to poll.
fn poll_for_complete_s3_list_background_assets_and_populate_cache_if_compolete(
    downloaded_s3_filenames: Res<FetchS3FileNamesTask>,
    mut cached_background_asset_paths: ResMut<BackupBackgroundAssetPaths>,
) {
    if let Ok(file_names) = (*downloaded_s3_filenames).try_recv() {
        info!(
            "Retrieved {} background assets paths from S3. Found: {:?}",
            &file_names.len(),
            &file_names
        );
        **cached_background_asset_paths = file_names;
        downloaded_s3_filenames.close(); // We should only need to do this once.
    }
}
impl Plugin for PublicS3AssetsPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = bounded::<Vec<String>>(1);
        app.insert_resource(DownloadedS3FileNamesChannelInput::from(tx))
            .insert_resource(FetchS3FileNamesTask::from(rx))
            .insert_resource(RemoteAssetFetchOptions {
                s3_bucket_base_url: "https://assets.thetawave.metalmancy.tech".to_string(),
                timeout_ms: 2000,
            })
            .add_systems(OnEnter(AppStates::LoadingAssets), start_listing_s3_files)
            .add_systems(
                FixedUpdate,
                poll_for_complete_s3_list_background_assets_and_populate_cache_if_compolete.run_if(
                    in_state(AppStates::MainMenu).or_else(in_state(AppStates::LoadingAssets)),
                ),
            );
    }
}
