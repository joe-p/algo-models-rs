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
pub struct RawTransaction200Response {
    /// encoding of the transaction hash.
    #[serde(rename = "txId")]
    pub tx_id: String,
}

impl RawTransaction200Response {
    pub fn new(tx_id: String) -> RawTransaction200Response {
        RawTransaction200Response {
            tx_id,
        }
    }
}

