/*
 * Algod REST API.
 *
 * API endpoint for algod operations.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: contact@algorand.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetBlockTimeStampOffset200Response {
    /// Timestamp offset in seconds.
    #[serde(rename = "offset")]
    pub offset: i32,
}

impl GetBlockTimeStampOffset200Response {
    pub fn new(offset: i32) -> GetBlockTimeStampOffset200Response {
        GetBlockTimeStampOffset200Response {
            offset,
        }
    }
}

