use crate::diagnostics::{diagnostic, sort};
use crate::identifiers::valid_semver;

use super::artifact_validation;
use super::policy_validation;
use super::references::{identifier, opaque};
use super::{InputValidationReport, WorkflowInput, WORKFLOW_INPUT_SCHEMA};

/// Validates the engine-neutral handoff without reading its referenced artifact.
#[must_use]
pub fn validate_input(input: &WorkflowInput) -> InputValidationReport {
    let mut diagnostics = Vec::new();
    if input.schema != WORKFLOW_INPUT_SCHEMA {
        diagnostics.push(diagnostic(
            "constillo.input.schema.unsupported",
            "$.schema",
            format!("expected schema {WORKFLOW_INPUT_SCHEMA}"),
        ));
    }
    for (value, path) in [
        (&input.request_id, "$.request_id"),
        (&input.correlation_id, "$.correlation_id"),
        (&input.workflow_id, "$.workflow_id"),
        (&input.source.service_id, "$.source.service_id"),
        (&input.subject.id, "$.subject.id"),
        (&input.subject.headquarters_id, "$.subject.headquarters_id"),
        (&input.subject.department_id, "$.subject.department_id"),
        (&input.subject.team_id, "$.subject.team_id"),
    ] {
        identifier(value, path, &mut diagnostics);
    }
    if !valid_semver(&input.workflow_version) {
        diagnostics.push(diagnostic(
            "constillo.input.workflow.invalid-version",
            "$.workflow_version",
            "workflow_version must use numeric major.minor.patch form",
        ));
    }
    opaque(
        &input.source.receipt_ref,
        "$.source.receipt_ref",
        &mut diagnostics,
    );
    artifact_validation::validate(&input.artifact, &mut diagnostics);
    opaque(&input.commission_ref, "$.commission_ref", &mut diagnostics);
    if let Some(reference) = &input.decision_ref {
        opaque(reference, "$.decision_ref", &mut diagnostics);
    }
    policy_validation::validate(input, &mut diagnostics);
    sort(&mut diagnostics);
    InputValidationReport::new(
        input.request_id.clone(),
        input.workflow_id.clone(),
        diagnostics,
    )
}
