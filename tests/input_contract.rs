use constillo::{
    parse_workflow_input_json, parse_workflow_json, plan_input, validate_input, ExecutionMode,
    InformationClassification,
};

const WORKFLOW: &str = include_str!("../examples/workflow.input-plan.json");
const INPUT: &str = include_str!("../examples/workflow-input.valid.json");

#[test]
fn engine_neutral_input_emits_a_stable_non_executable_receipt() {
    let workflow = parse_workflow_json(WORKFLOW).expect("workflow");
    let input = parse_workflow_input_json(INPUT).expect("input");
    assert!(validate_input(&input).valid);
    let first = plan_input(&workflow, &input).expect("receipt");
    let second = plan_input(&workflow, &input).expect("receipt");
    assert_eq!(first, second);
    assert!(first.accepted);
    assert!(!first.executable);
    assert!(!first.external_actions);
    assert!(!first.plan.executable);
    assert_eq!(first.receipt_id.len(), 32);
    let serialized = serde_json::to_string(&first).expect("receipt JSON");
    for forbidden in ["/mounted/", "password", "credential", "provider_data"] {
        assert!(!serialized.contains(forbidden));
    }
}

#[test]
fn binding_identity_and_external_steps_fail_closed() {
    let mut workflow = parse_workflow_json(WORKFLOW).expect("workflow");
    let input = parse_workflow_input_json(INPUT).expect("input");
    workflow.workflow_id = "other".to_owned();
    workflow.workflow_version = "2.0.0".to_owned();
    workflow.steps[0].external_action = true;
    workflow.steps[0].approval.required = true;
    workflow.steps[0].approval.gate_id = Some("owner".to_owned());
    workflow.steps[0].idempotency.required = true;
    workflow.steps[0].idempotency.key = Some("stable".to_owned());
    let report = plan_input(&workflow, &input).expect_err("binding failure");
    for code in [
        "constillo.input.binding.workflow-id-mismatch",
        "constillo.input.binding.workflow-version-mismatch",
        "constillo.input.workflow.external-action-forbidden",
    ] {
        assert!(
            report.diagnostics.iter().any(|item| item.code == code),
            "{code}"
        );
    }
}

#[test]
fn paths_urls_secrets_runtime_and_external_actions_are_rejected() {
    let mut input = parse_workflow_input_json(INPUT).expect("input");
    input.source.receipt_ref = "/mounted/private/report.json".to_owned();
    input.artifact.opaque_ref = "https://example.invalid/report".to_owned();
    input.commission_ref = "secret:token=abc".to_owned();
    input.execution_mode = ExecutionMode::Runtime;
    input.external_actions = true;
    let report = validate_input(&input);
    assert!(!report.valid);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.input.invalid-opaque-ref"));
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.input.execution.runtime-forbidden"));
}

#[test]
fn customer_data_requires_matching_classification_and_decision() {
    let mut input = parse_workflow_input_json(INPUT).expect("input");
    input.customer_data = true;
    input.classification = InformationClassification::CustomerConfidential;
    input.decision_ref = Some("decision:customer-data-approved".to_owned());
    assert!(validate_input(&input).valid);
    input.classification = InformationClassification::PersonalConfidential;
    assert!(!validate_input(&input).valid);
}

#[test]
fn workflow_input_json_is_closed_and_bounded() {
    let unknown = INPUT.replace(
        "\"external_actions\": false",
        "\"external_actions\": false, \"token\": \"no\"",
    );
    assert!(parse_workflow_input_json(&unknown).is_err());
    assert!(parse_workflow_input_json(&" ".repeat(1_048_577)).is_err());
}
