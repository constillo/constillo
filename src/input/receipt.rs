use serde::Serialize;

use crate::diagnostics::diagnostic;
use crate::{Diagnostic, WorkflowPlan};

use super::digest::{receipt_id, sha256_hex, sha256_prefixed};
use super::{WorkflowInput, INPUT_RECEIPT_SCHEMA_VERSION};

/// Content-free acceptance receipt containing a non-executable workflow plan.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InputReceipt {
    pub schema_version: &'static str,
    pub receipt_id: String,
    pub request_id: String,
    pub input_digest: String,
    pub workflow_id: String,
    pub workflow_version: String,
    pub correlation_id: String,
    pub artifact_digest_sha256: String,
    pub accepted: bool,
    pub executable: bool,
    pub external_actions: bool,
    pub plan: WorkflowPlan,
}

pub(crate) fn build(input: &WorkflowInput, plan: WorkflowPlan) -> Result<InputReceipt, Diagnostic> {
    let canonical_input = serde_json::to_vec(input).map_err(|_| {
        diagnostic(
            "constillo.input.internal-serialization",
            "$",
            "closed workflow input could not be canonicalized",
        )
    })?;
    let canonical_plan = serde_json::to_vec(&plan).map_err(|_| {
        diagnostic(
            "constillo.input.internal-plan-serialization",
            "workflow:$",
            "workflow plan could not be canonicalized",
        )
    })?;
    let input_digest = sha256_prefixed(&canonical_input);
    let plan_digest = sha256_hex(&canonical_plan);
    let receipt_id = receipt_id(&[
        &input_digest,
        &input.workflow_id,
        &input.workflow_version,
        &input.correlation_id,
        &input.artifact.digest_sha256,
        &plan_digest,
    ]);
    Ok(InputReceipt {
        schema_version: INPUT_RECEIPT_SCHEMA_VERSION,
        receipt_id,
        request_id: input.request_id.clone(),
        input_digest,
        workflow_id: input.workflow_id.clone(),
        workflow_version: input.workflow_version.clone(),
        correlation_id: input.correlation_id.clone(),
        artifact_digest_sha256: input.artifact.digest_sha256.clone(),
        accepted: true,
        executable: false,
        external_actions: false,
        plan,
    })
}
