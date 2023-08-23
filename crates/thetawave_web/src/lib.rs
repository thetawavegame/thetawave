pub mod public_s3_assets;
mod s3;
use bevy_log::{error, warn};
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[wasm_bindgen(module = "/src/fetch_url_with_timeout.js")]
extern "C" {
    async fn fetchWithTimeout(url: String, timeoutMs: u32) -> JsValue;
}

fn get_filenames_from_s3_list_buckets_resp(s3_resp: s3::ListBucketV2Response) -> Vec<String> {
    s3_resp.contents.iter().map(|x| x.key.clone()).collect()
}

/// If this returns an empty collection, then something is wrong (we would have logged it). And the
/// game should just move on. S3 data should be _very_ optional.
pub async fn raw_list_public_s3_bucket(url: &str, timeout_ms: u32) -> Vec<String> {
    match fetchWithTimeout(String::from(url), timeout_ms)
        .await
        .as_string()
    {
        Some(str_result) => {
            return serde_xml_rs::from_str::<s3::ListBucketV2Response>(&str_result)
                .map(get_filenames_from_s3_list_buckets_resp)
                .unwrap_or_else(|err| {
                    error!("Failed to parse s3 list bucket response. Err: {}", err);
                    error!("Bad S3 listBuckets response; {}", str_result);
                    Default::default()
                });
        }
        None => {
            warn!("Received no data from unsigned S3 List objects request.");
        }
    };
    Default::default()
}
