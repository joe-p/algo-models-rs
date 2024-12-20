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

/// TransactionParams200Response : TransactionParams contains the parameters that help a client construct a new transaction.
#[serde_as]
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionParams200Response {
    /// ConsensusVersion indicates the consensus protocol version as of LastRound.
    #[serde(rename = "consensus-version")]
    pub consensus_version: String,
    /// Fee is the suggested transaction fee Fee is in units of micro-Algos per byte. Fee may fall to zero but transactions must still have a fee of at least MinTxnFee for the current network protocol.
    #[serde(rename = "fee")]
    pub fee: i32,
    /// GenesisHash is the hash of the genesis block.
    #[serde_as(as = "serde_with::base64::Base64")]
    #[serde(rename = "genesis-hash")]
    pub genesis_hash: Vec<u8>,
    /// GenesisID is an ID listed in the genesis block.
    #[serde(rename = "genesis-id")]
    pub genesis_id: String,
    /// LastRound indicates the last round seen
    #[serde(rename = "last-round")]
    pub last_round: i32,
    /// The minimum transaction fee (not per byte) required for the txn to validate for the current network protocol.
    #[serde(rename = "min-fee")]
    pub min_fee: i32,
}

impl TransactionParams200Response {
    /// TransactionParams contains the parameters that help a client construct a new transaction.
    pub fn new(consensus_version: String, fee: i32, genesis_hash: Vec<u8>, genesis_id: String, last_round: i32, min_fee: i32) -> TransactionParams200Response {
        TransactionParams200Response {
            consensus_version,
            fee,
            genesis_hash,
            genesis_id,
            last_round,
            min_fee,
        }
    }
}

