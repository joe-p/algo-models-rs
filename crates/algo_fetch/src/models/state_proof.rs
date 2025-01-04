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

/// StateProof : Represents a state proof and its corresponding message
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StateProof {
    #[serde(rename = "Message")]
    pub message: Box<models::StateProofMessage>,
    /// The encoded StateProof for the message.
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "StateProof")]
    pub state_proof: Vec<u8>,
}

impl StateProof {
    /// Represents a state proof and its corresponding message
    pub fn new(message: models::StateProofMessage, state_proof: Vec<u8>) -> StateProof {
        StateProof {
            message: Box::new(message),
            state_proof,
        }
    }
}

