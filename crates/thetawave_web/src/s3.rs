//! Minimal parsing for a few S3 requests. Very much not feature complete, to keep the wasm binary small. Many keys are
//! missing from the objects, keeping things close to what we need for the game. We might eventually use
//! https://github.com/awslabs/aws-sdk-rust
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketV2Response {
    pub name: String,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    pub max_keys: Option<u32>,
    pub is_truncated: bool,
    pub contents: Vec<Object>,
    pub common_prefixes: Option<Vec<CommonPrefix>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Object {
    pub key: String,
    pub last_modified: String,
    pub size: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CommonPrefix {
    pub prefix: String,
}
