#![allow(non_snake_case, unused)]
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
// use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::get_call_parameters;
use cio_response_types::{AMResponse};
// use crate::serde_json::Value;

module_manifest!();

mod curl;
mod utils;
mod types;
mod query;

use types::TLRequest;
use types::TLQuery;

pub fn main() {}

#[marine]
pub fn tl_insert(gateway: String, tl_request: &TLRequest) -> AMResponse {

    query::run(
        format!("{}/record", gateway),
        &serde_json::to_string(tl_request).unwrap()
    )
}

#[marine]
pub fn tl_query(gateway: String, tl_query: &TLQuery) -> AMResponse {

    query::run(
        format!("{}/query", gateway),
        &serde_json::to_string(tl_query).unwrap()
    )
}
