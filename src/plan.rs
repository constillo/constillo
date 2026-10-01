use std::collections::BTreeMap;

use crate::diagnostics::diagnostic;
use crate::graph::topological_order;
use crate::{validate, PlannedStep, ValidationReport, Workflow, WorkflowPlan, PLAN_SCHEMA_VERSION};

/// Builds a deterministic, non-executable topological plan.
///
/// # Errors
///
/// Returns the complete validation report when the workflow is invalid.
pub fn plan(workflow: &Workflow) -> Result<WorkflowPlan, ValidationReport> {
    let validation = validate(workflow);
    if !validation.valid {
        return Err(validation);
    }
    let Some(order) = topological_order(workflow) else {
        return Err(ValidationReport {
            schema_version: "constillo.validation/v1",
            workflow_id: workflow.workflow_id.clone(),
            valid: false,
            diagnostics: vec![diagnostic(
                "constillo.workflow.cycle",
                "$.steps",
                "dependency graph contains a cycle",
            )],
        });
    };
    let steps: BTreeMap<_, _> = workflow
        .steps
        .iter()
        .map(|step| (step.id.as_str(), step))
        .collect();
    let mut levels: BTreeMap<&str, usize> = BTreeMap::new();
    let mut planned = Vec::with_capacity(order.len());
    for (sequence, step_id) in order.into_iter().enumerate() {
        let step = steps[step_id.as_str()];
        let level = step
            .depends_on
            .iter()
            .filter_map(|dependency| levels.get(dependency.as_str()))
            .max()
            .map_or(0, |value| value + 1);
        levels.insert(step.id.as_str(), level);
        let mut depends_on = step.depends_on.clone();
        depends_on.sort();
        planned.push(PlannedStep {
            sequence,
            level,
            step_id: step.id.clone(),
            action: step.action.clone(),
            depends_on,
            external_action: step.external_action,
            approval_gate: step.approval.gate_id.clone(),
            idempotency_key: step.idempotency.key.clone(),
        });
    }
    Ok(WorkflowPlan {
        schema_version: PLAN_SCHEMA_VERSION,
        workflow_id: workflow.workflow_id.clone(),
        workflow_version: workflow.workflow_version.clone(),
        mode: "plan-only",
        executable: false,
        steps: planned,
    })
}
