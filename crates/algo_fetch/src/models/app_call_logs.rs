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

/// AppCallLogs : The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppCallLogs {
    /// The application from which the logs were generated
    #[serde(rename = "application-index")]
    pub application_index: i32,
    /// An array of logs
    #[serde(rename = "logs")]
    pub logs: Vec<String>,
    /// The transaction ID of the outer app call that lead to these logs
    #[serde(rename = "txId")]
    pub tx_id: String,
}

impl AppCallLogs {
    /// The logged messages from an app call along with the app ID and outer transaction ID. Logs appear in the same order that they were emitted.
    pub fn new(application_index: i32, logs: Vec<String>, tx_id: String) -> AppCallLogs {
        AppCallLogs {
            application_index,
            logs,
            tx_id,
        }
    }
}

