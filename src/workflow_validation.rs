use std::collections::{BTreeMap, BTreeSet};

use crate::diagnostics::{diagnostic, sort};
use crate::graph::topological_order;
use crate::identifiers::{valid_identifier, valid_semver};
use crate::step_validation::validate_step;
use crate::{ValidationReport, Workflow, WORKFLOW_SCHEMA_VERSION};

/// Validates workflow identity, control policy, dependency integrity, and DAG shape.
#[must_use]
pub fn validate(workflow: &Workflow) -> ValidationReport {
    let mut diagnostics = Vec::new();
    if workflow.schema_version != WORKFLOW_SCHEMA_VERSION {
        diagnostics.push(diagnostic(
            "constillo.schema.unsupported",
            "$.schema_version",
            format!("expected schema version {WORKFLOW_SCHEMA_VERSION}"),
        ));
    }
    if !valid_identifier(&workflow.workflow_id) {
        diagnostics.push(diagnostic(
            "constillo.workflow.invalid-id",
            "$.workflow_id",
            "workflow_id must be a bounded lowercase ASCII identifier",
        ));
    }
    if !valid_semver(&workflow.workflow_version) {
        diagnostics.push(diagnostic(
            "constillo.workflow.invalid-version",
            "$.workflow_version",
            "workflow_version must use numeric major.minor.patch form",
        ));
    }
    if workflow.steps.is_empty() || workflow.steps.len() > 1_024 {
        diagnostics.push(diagnostic(
            "constillo.workflow.invalid-step-count",
            "$.steps",
            "workflow must contain between 1 and 1024 steps",
        ));
    }
    let mut steps_by_id = BTreeMap::new();
    for (index, step) in workflow.steps.iter().enumerate() {
        validate_step(step, index, &mut diagnostics);
        if steps_by_id.insert(step.id.as_str(), index).is_some() {
            diagnostics.push(diagnostic(
                "constillo.step.duplicate-id",
                format!("$.steps[{index}].id"),
                format!("duplicate step id '{}'", step.id),
            ));
        }
    }
    missing_dependencies(workflow, &mut diagnostics);
    if diagnostics.is_empty() && topological_order(workflow).is_none() {
        diagnostics.push(diagnostic(
            "constillo.workflow.cycle",
            "$.steps",
            "dependency graph contains a cycle",
        ));
    }
    sort(&mut diagnostics);
    ValidationReport {
        schema_version: "constillo.validation/v1",
        workflow_id: workflow.workflow_id.clone(),
        valid: diagnostics.is_empty(),
        diagnostics,
    }
}

fn missing_dependencies(workflow: &Workflow, diagnostics: &mut Vec<crate::Diagnostic>) {
    let known: BTreeSet<&str> = workflow.steps.iter().map(|step| step.id.as_str()).collect();
    for (index, step) in workflow.steps.iter().enumerate() {
        for dependency in &step.depends_on {
            if !known.contains(dependency.as_str()) {
                diagnostics.push(diagnostic(
                    "constillo.step.missing-dependency",
                    format!("$.steps[{index}].depends_on"),
                    format!("dependency '{dependency}' is not defined"),
                ));
            }
        }
    }
}
