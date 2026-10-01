use crate::diagnostics::diagnostic;
use crate::Diagnostic;

use super::references::prefixed_sha256;
use super::{ExecutionMode, InformationClassification, WorkflowInput};

pub(crate) fn validate(input: &WorkflowInput, diagnostics: &mut Vec<Diagnostic>) {
    let customer_classification =
        input.classification == InformationClassification::CustomerConfidential;
    if input.customer_data != customer_classification {
        diagnostics.push(diagnostic(
            "constillo.input.classification.customer-data-mismatch",
            "$.customer_data",
            "customer_data and customer-confidential classification must be selected together",
        ));
    }
    if matches!(
        input.classification,
        InformationClassification::RestrictedSensitive
            | InformationClassification::CustomerConfidential
    ) && input.decision_ref.is_none()
    {
        diagnostics.push(diagnostic(
            "constillo.input.classification.decision-required",
            "$.decision_ref",
            "restricted input requires an opaque decision reference",
        ));
    }
    if !prefixed_sha256(&input.idempotency_key) {
        diagnostics.push(diagnostic(
            "constillo.input.idempotency.invalid-key",
            "$.idempotency_key",
            "idempotency_key must be sha256: followed by 64 lowercase hex characters",
        ));
    }
    if input.external_actions {
        diagnostics.push(diagnostic(
            "constillo.input.external-actions.forbidden",
            "$.external_actions",
            "workflow input must declare external_actions=false",
        ));
    }
    if input.execution_mode == ExecutionMode::Runtime {
        diagnostics.push(diagnostic(
            "constillo.input.execution.runtime-forbidden",
            "$.execution_mode",
            "runtime mode is unavailable in this plan-only build",
        ));
    }
}
