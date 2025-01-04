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

use serde_with::serde_as;

/// KvDelta : A single Delta containing the key, the previous value and the current value for a single round.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct KvDelta {
    /// The key, base64 encoded.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Vec<u8>>,
    /// The new value of the KV store entry, base64 encoded.
    #[serde_as(as = "Option<serde_with::base64::Base64>")]
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<u8>>,
}

impl KvDelta {
    /// A single Delta containing the key, the previous value and the current value for a single round.
    pub fn new() -> KvDelta {
        KvDelta {
            key: None,
            value: None,
        }
    }
}

