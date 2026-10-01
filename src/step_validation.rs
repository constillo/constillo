use std::collections::BTreeSet;

use crate::diagnostics::diagnostic;
use crate::identifiers::valid_identifier;
use crate::{Diagnostic, WorkflowStep};

pub(crate) fn validate_step(step: &WorkflowStep, index: usize, diagnostics: &mut Vec<Diagnostic>) {
    let base = format!("$.steps[{index}]");
    if !valid_identifier(&step.id) {
        diagnostics.push(diagnostic(
            "constillo.step.invalid-id",
            format!("{base}.id"),
            "step id must be a bounded lowercase ASCII identifier",
        ));
    }
    let action_length = step.action.chars().count();
    if action_length == 0 || action_length > 256 {
        diagnostics.push(diagnostic(
            "constillo.step.invalid-action",
            format!("{base}.action"),
            "action must contain between 1 and 256 characters",
        ));
    }
    if step.depends_on.len() > 1_024 {
        diagnostics.push(diagnostic(
            "constillo.step.too-many-dependencies",
            format!("{base}.depends_on"),
            "depends_on must contain at most 1024 step ids",
        ));
    }
    let unique: BTreeSet<_> = step.depends_on.iter().collect();
    if unique.len() != step.depends_on.len() {
        diagnostics.push(diagnostic(
            "constillo.step.duplicate-dependency",
            format!("{base}.depends_on"),
            "depends_on must not contain duplicate step ids",
        ));
    }
    if step.depends_on.iter().any(|value| value == &step.id) {
        diagnostics.push(diagnostic(
            "constillo.step.self-dependency",
            format!("{base}.depends_on"),
            "a step cannot depend on itself",
        ));
    }
    validate_controls(step, &base, diagnostics);
}

fn validate_controls(step: &WorkflowStep, base: &str, diagnostics: &mut Vec<Diagnostic>) {
    let gate = step
        .approval
        .gate_id
        .as_deref()
        .filter(|value| !value.trim().is_empty());
    match (step.approval.required, gate) {
        (true, None) => diagnostics.push(diagnostic(
            "constillo.approval.missing-gate",
            format!("{base}.approval.gate_id"),
            "gate_id is required when approval.required is true",
        )),
        (false, Some(_)) => diagnostics.push(diagnostic(
            "constillo.approval.unused-gate",
            format!("{base}.approval.gate_id"),
            "gate_id must be null when approval.required is false",
        )),
        _ => {}
    }
    let key = step
        .idempotency
        .key
        .as_deref()
        .filter(|value| !value.trim().is_empty() && value.len() <= 256);
    match (step.idempotency.required, key) {
        (true, None) => diagnostics.push(diagnostic(
            "constillo.idempotency.missing-key",
            format!("{base}.idempotency.key"),
            "a bounded key is required when idempotency.required is true",
        )),
        (false, Some(_)) => diagnostics.push(diagnostic(
            "constillo.idempotency.unused-key",
            format!("{base}.idempotency.key"),
            "key must be null when idempotency.required is false",
        )),
        _ => {}
    }
    if step.external_action && !step.approval.required {
        diagnostics.push(diagnostic(
            "constillo.external.approval-required",
            format!("{base}.approval.required"),
            "external actions require an approval gate",
        ));
    }
    if step.external_action && !step.idempotency.required {
        diagnostics.push(diagnostic(
            "constillo.external.idempotency-required",
            format!("{base}.idempotency.required"),
            "external actions require an idempotency key",
        ));
    }
}
