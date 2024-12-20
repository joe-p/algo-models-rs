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

/// Application : Application index and its parameters
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    /// \\[appidx\\] application index.
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "params")]
    pub params: Box<models::ApplicationParams>,
}

impl Application {
    /// Application index and its parameters
    pub fn new(id: i32, params: models::ApplicationParams) -> Application {
        Application {
            id,
            params: Box::new(params),
        }
    }
}

