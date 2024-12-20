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

/// ErrorResponse : An error response with optional data field.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    #[serde(rename = "message")]
    pub message: String,
}

impl ErrorResponse {
    /// An error response with optional data field.
    pub fn new(message: String) -> ErrorResponse {
        ErrorResponse {
            data: None,
            message,
        }
    }
}
