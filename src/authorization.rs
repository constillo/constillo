//! A closed receipt binds one workflow plan to one verified TPAR decision.

use crate::{plan, Workflow, WorkflowPlan};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const DECISION_SCHEMA: &str = "tpar.authorized-decision.v1";
pub const RECEIPT_SCHEMA: &str = "constillo.workflow-authorization-receipt.v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DecisionAuthorization {
    pub schema: String,
    pub decision_id: String,
    pub decision_digest_sha256: String,
    pub official_hat_id: String,
    pub hat_manifest_digest_sha256: String,
    pub proposal_id: String,
    pub proposal_fingerprint: String,
    pub recommended_option_id: String,
    pub execution_authorized: bool,
    pub external_actions: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WorkflowAuthorizationReceipt {
    pub schema: &'static str,
    pub receipt_id: String,
    pub receipt_digest_sha256: String,
    pub workflow_id: String,
    pub workflow_version: String,
    pub workflow_plan_digest_sha256: String,
    pub decision_id: String,
    pub decision_digest_sha256: String,
    pub official_hat_id: String,
    pub hat_manifest_digest_sha256: String,
    pub recommended_option_id: String,
    pub executable: bool,
    pub external_actions: bool,
}

#[derive(Serialize)]
struct DecisionSeed<'a> {
    official_hat_id: &'a str,
    hat_manifest_digest_sha256: &'a str,
    proposal_id: &'a str,
    proposal_fingerprint: &'a str,
    recommended_option_id: &'a str,
    execution_authorized: bool,
    external_actions: bool,
}

/// Parses a closed TPAR decision receipt.
///
/// # Errors
///
/// Returns an error for malformed, oversized, or extended JSON.
pub fn parse_decision_authorization(source: &str) -> Result<DecisionAuthorization, String> {
    crate::json_input::parse_bounded(source, "decision authorization")
}

/// Produces a receipt binding the workflow plan to the verified decision.
///
/// # Errors
///
/// Returns an error when either digest fails or the workflow is not bound to
/// the recommended option.
pub fn plan_authorized(
    workflow: &Workflow,
    decision: &DecisionAuthorization,
) -> Result<WorkflowAuthorizationReceipt, String> {
    validate_decision(decision)?;
    let expected_action = format!("record-{}-decision", decision.recommended_option_id);
    if !workflow
        .steps
        .iter()
        .any(|step| step.action == expected_action)
    {
        return Err("workflow does not bind the authorized recommendation".to_string());
    }
    let plan = plan(workflow).map_err(|_| "authorized workflow is invalid".to_string())?;
    build_receipt(&plan, decision)
}

fn validate_decision(value: &DecisionAuthorization) -> Result<(), String> {
    let seed = DecisionSeed {
        official_hat_id: &value.official_hat_id,
        hat_manifest_digest_sha256: &value.hat_manifest_digest_sha256,
        proposal_id: &value.proposal_id,
        proposal_fingerprint: &value.proposal_fingerprint,
        recommended_option_id: &value.recommended_option_id,
        execution_authorized: false,
        external_actions: false,
    };
    let expected = digest(&serde_json::to_vec(&seed).map_err(|_| "decision encoding failed")?);
    let valid = value.schema == DECISION_SCHEMA
        && value.decision_id == format!("decision-{}", &expected[7..31])
        && value.decision_digest_sha256 == expected
        && valid_digest(&value.hat_manifest_digest_sha256)
        && !value.execution_authorized
        && !value.external_actions;
    valid
        .then_some(())
        .ok_or_else(|| "decision authorization failed integrity validation".to_string())
}

fn build_receipt(
    plan: &WorkflowPlan,
    decision: &DecisionAuthorization,
) -> Result<WorkflowAuthorizationReceipt, String> {
    let plan_digest = digest(&serde_json::to_vec(plan).map_err(|_| "plan encoding failed")?);
    let material = serde_json::json!({
        "workflow_id": plan.workflow_id,
        "workflow_version": plan.workflow_version,
        "workflow_plan_digest_sha256": plan_digest,
        "decision_id": decision.decision_id,
        "decision_digest_sha256": decision.decision_digest_sha256,
        "official_hat_id": decision.official_hat_id,
        "hat_manifest_digest_sha256": decision.hat_manifest_digest_sha256,
        "recommended_option_id": decision.recommended_option_id,
        "executable": false,
        "external_actions": false
    });
    let receipt_digest = digest(&serde_json::to_vec(&material).map_err(|_| "receipt encoding")?);
    Ok(WorkflowAuthorizationReceipt {
        schema: RECEIPT_SCHEMA,
        receipt_id: format!("workflow-receipt-{}", &receipt_digest[7..31]),
        receipt_digest_sha256: receipt_digest,
        workflow_id: plan.workflow_id.clone(),
        workflow_version: plan.workflow_version.clone(),
        workflow_plan_digest_sha256: plan_digest,
        decision_id: decision.decision_id.clone(),
        decision_digest_sha256: decision.decision_digest_sha256.clone(),
        official_hat_id: decision.official_hat_id.clone(),
        hat_manifest_digest_sha256: decision.hat_manifest_digest_sha256.clone(),
        recommended_option_id: decision.recommended_option_id.clone(),
        executable: false,
        external_actions: false,
    })
}

fn valid_digest(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}
