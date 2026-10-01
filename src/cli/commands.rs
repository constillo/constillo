use constillo::{plan, plan_authorized, plan_input, validate, validate_input};
use serde_json::{json, Value};

use super::file_input::{load_decision, load_input, load_workflow};

pub(super) fn dispatch(arguments: &[String]) -> Result<Value, (u8, Value)> {
    match arguments {
        [command] if command == "doctor" => Ok(doctor()),
        [command, path] if command == "validate" => {
            let workflow = load_workflow(path)?;
            result_or_validation(validate(&workflow))
        }
        [command, path] if command == "plan" => {
            let workflow = load_workflow(path)?;
            plan(&workflow).map_or_else(
                |report| Err((2, serialize(report))),
                |output| Ok(serialize(output)),
            )
        }
        [command, path] if command == "validate-input" => {
            let input = load_input(path)?;
            let report = validate_input(&input);
            result_or_input_validation(report)
        }
        [command, workflow_path, input_path] if command == "plan-input" => {
            let workflow = load_workflow(workflow_path)?;
            let input = load_input(input_path)?;
            plan_input(&workflow, &input).map_or_else(
                |report| Err((2, serialize(report))),
                |receipt| Ok(serialize(receipt)),
            )
        }
        [command, workflow_path, decision_path] if command == "plan-authorized" => {
            let workflow = load_workflow(workflow_path)?;
            let decision = load_decision(decision_path)?;
            plan_authorized(&workflow, &decision).map_or_else(
                |message| {
                    Err((
                        2,
                        json!({
                            "schema_version": "constillo.authorization-error/v1",
                            "ok": false,
                            "message": message,
                            "executable": false,
                            "external_actions": false
                        }),
                    ))
                },
                |receipt| Ok(serialize(receipt)),
            )
        }
        _ => Err((
            1,
            json!({
                "schema_version": "constillo.error/v1",
                "ok": false,
                "code": "constillo.cli.usage",
                "message": "usage: constillo doctor | validate <workflow> | plan <workflow> | validate-input <input> | plan-input <workflow> <input> | plan-authorized <workflow> <decision>"
            }),
        )),
    }
}

fn doctor() -> Value {
    json!({
        "schema_version": "constillo.doctor/v1",
        "healthy": true,
        "mode": "plan-only",
        "capabilities": {
            "validate": true,
            "deterministic_plan": true,
            "validate_workflow_input": true,
            "deterministic_input_receipt": true,
            "authorized_workflow_receipt": true,
            "external_execution": false,
            "network_access": false,
            "credential_storage": false
        }
    })
}

fn result_or_validation(report: constillo::ValidationReport) -> Result<Value, (u8, Value)> {
    let valid = report.valid;
    let value = serialize(report);
    if valid {
        Ok(value)
    } else {
        Err((2, value))
    }
}

fn result_or_input_validation(
    report: constillo::InputValidationReport,
) -> Result<Value, (u8, Value)> {
    let valid = report.valid;
    let value = serialize(report);
    if valid {
        Ok(value)
    } else {
        Err((2, value))
    }
}

fn serialize(value: impl serde::Serialize) -> Value {
    serde_json::to_value(value).expect("closed command result serializes")
}
