//! Exposes a plugin Bevy plugin that lists file names from an S3 service and puts those names in
//! `BackupBackgroundAssetPaths`. The `THETAWAVE_S3_ASSETS_BASE_URL` compile-time environment
//! variable controls S3 location.
use crate::s3;
use async_channel::{bounded, Receiver, Sender};
use bevy_app::{App, FixedUpdate, Plugin};
use bevy_ecs::prelude::{in_state, Condition, IntoSystemConfigs, OnEnter, Resource};
use bevy_ecs::system::{Res, ResMut};
use bevy_log::{error, info, warn};
use derive_more::{Deref, DerefMut, From};
use thetawave_interface::assets::BackupBackgroundAssetPaths;
use thetawave_interface::states::AppStates;
use wasm_bindgen_futures::spawn_local;

/// Since Bevy doesn't handle async things so well (each system function ultimately blocks the next
/// frame), we spawn a single-element queue with these ends, then populate a resource as part of
/// the public API used by other plugins. This might be reworked to not require this resource
/// based on https://github.com/bevyengine/bevy/discussions/3351 in later bevy versions
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
fn poll_for_complete_s3_list_background_assets_and_populate_cache_if_complete(
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
                poll_for_complete_s3_list_background_assets_and_populate_cache_if_complete.run_if(
                    in_state(AppStates::MainMenu).or_else(in_state(AppStates::LoadingAssets)),
                ),
            );
    }
}

fn get_filenames_from_s3_list_buckets_resp(s3_resp: s3::ListBucketV2Response) -> Vec<String> {
    s3_resp.contents.iter().map(|x| x.key.clone()).collect()
}

/// If this returns an empty collection, then something is wrong (we would have logged it). And the
/// game should just move on. S3 data should be _very_ optional.
async fn raw_list_public_s3_bucket(url: &str, timeout_ms: u32) -> Vec<String> {
    match crate::fetchWithTimeout(String::from(url), timeout_ms)
        .await
        .as_string()
    {
        Some(str_result) => {
            return serde_xml_rs::from_str::<s3::ListBucketV2Response>(&str_result)
                .map(get_filenames_from_s3_list_buckets_resp)
                .unwrap_or_else(|err| {
                    error!("Failed to parse s3 list bucket response. Err: {}", err);
                    error!("Bad S3 listBuckets response; {}", &str_result);
                    Default::default()
                });
        }
        None => {
            warn!("Received no data from unsigned S3 List objects request.");
        }
    };
    Default::default()
}
