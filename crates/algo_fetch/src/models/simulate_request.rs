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

/// SimulateRequest : Request type for simulation endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulateRequest {
    /// Allows transactions without signatures to be simulated as if they had correct signatures.
    #[serde(rename = "allow-empty-signatures", skip_serializing_if = "Option::is_none")]
    pub allow_empty_signatures: Option<bool>,
    /// Lifts limits on log opcode usage during simulation.
    #[serde(rename = "allow-more-logging", skip_serializing_if = "Option::is_none")]
    pub allow_more_logging: Option<bool>,
    /// Allows access to unnamed resources during simulation.
    #[serde(rename = "allow-unnamed-resources", skip_serializing_if = "Option::is_none")]
    pub allow_unnamed_resources: Option<bool>,
    #[serde(rename = "exec-trace-config", skip_serializing_if = "Option::is_none")]
    pub exec_trace_config: Option<Box<models::SimulateTraceConfig>>,
    /// Applies extra opcode budget during simulation for each transaction group.
    #[serde(rename = "extra-opcode-budget", skip_serializing_if = "Option::is_none")]
    pub extra_opcode_budget: Option<i32>,
    /// If true, signers for transactions that are missing signatures will be fixed during evaluation.
    #[serde(rename = "fix-signers", skip_serializing_if = "Option::is_none")]
    pub fix_signers: Option<bool>,
    /// If provided, specifies the round preceding the simulation. State changes through this round will be used to run this simulation. Usually only the 4 most recent rounds will be available (controlled by the node config value MaxAcctLookback). If not specified, defaults to the latest available round.
    #[serde(rename = "round", skip_serializing_if = "Option::is_none")]
    pub round: Option<i32>,
    /// The transaction groups to simulate.
    #[serde(rename = "txn-groups")]
    pub txn_groups: Vec<models::SimulateRequestTransactionGroup>,
}

impl SimulateRequest {
    /// Request type for simulation endpoint.
    pub fn new(txn_groups: Vec<models::SimulateRequestTransactionGroup>) -> SimulateRequest {
        SimulateRequest {
            allow_empty_signatures: None,
            allow_more_logging: None,
            allow_unnamed_resources: None,
            exec_trace_config: None,
            extra_opcode_budget: None,
            fix_signers: None,
            round: None,
            txn_groups,
        }
    }
}

