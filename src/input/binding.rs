use crate::diagnostics::{diagnostic, sort};
use crate::{plan, validate, Workflow};

use super::receipt;
use super::{validate_input, InputReceipt, InputValidationReport, WorkflowInput};

/// Binds a valid input to one local workflow and emits a deterministic receipt.
///
/// # Errors
///
/// Returns all input, workflow, binding, or external-action findings. It never
/// executes a step or resolves an artifact reference.
pub fn plan_input(
    workflow: &Workflow,
    input: &WorkflowInput,
) -> Result<InputReceipt, InputValidationReport> {
    let mut report = validate_input(input);
    let workflow_report = validate(workflow);
    report
        .diagnostics
        .extend(workflow_report.diagnostics.into_iter().map(prefix_workflow));
    if workflow.workflow_id != input.workflow_id {
        report.diagnostics.push(diagnostic(
            "constillo.input.binding.workflow-id-mismatch",
            "$.workflow_id",
            "input workflow_id does not match the selected workflow",
        ));
    }
    if workflow.workflow_version != input.workflow_version {
        report.diagnostics.push(diagnostic(
            "constillo.input.binding.workflow-version-mismatch",
            "$.workflow_version",
            "input workflow_version does not match the selected workflow",
        ));
    }
    for (index, step) in workflow.steps.iter().enumerate() {
        if step.external_action {
            report.diagnostics.push(diagnostic(
                "constillo.input.workflow.external-action-forbidden",
                format!("workflow:$.steps[{index}].external_action"),
                "plan-input accepts only workflows with external_action=false",
            ));
        }
    }
    sort(&mut report.diagnostics);
    report.valid = report.diagnostics.is_empty();
    if !report.valid {
        return Err(report);
    }
    let workflow_plan = match plan(workflow) {
        Ok(plan) => plan,
        Err(validation) => {
            report
                .diagnostics
                .extend(validation.diagnostics.into_iter().map(prefix_workflow));
            sort(&mut report.diagnostics);
            report.valid = false;
            return Err(report);
        }
    };
    match receipt::build(input, workflow_plan) {
        Ok(receipt) => Ok(receipt),
        Err(error) => {
            report.diagnostics.push(error);
            report.valid = false;
            Err(report)
        }
    }
}

fn prefix_workflow(mut diagnostic: crate::Diagnostic) -> crate::Diagnostic {
    diagnostic.path = format!("workflow:{}", diagnostic.path);
    diagnostic
}
