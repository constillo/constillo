use constillo::{parse_workflow_json, plan, validate};

const WORKFLOW: &str = include_str!("../examples/workflow.valid.json");

#[test]
fn standalone_workflow_is_valid_and_plans_deterministically() {
    let workflow = parse_workflow_json(WORKFLOW).expect("workflow");
    assert!(validate(&workflow).valid);
    let first = plan(&workflow).expect("plan");
    let second = plan(&workflow).expect("plan");
    assert_eq!(first, second);
    assert!(!first.executable);
    assert_eq!(
        first
            .steps
            .iter()
            .map(|step| step.step_id.as_str())
            .collect::<Vec<_>>(),
        ["collect", "review", "publish"]
    );
}

#[test]
fn cycles_missing_nodes_and_duplicate_dependencies_fail_closed() {
    let mut workflow = parse_workflow_json(WORKFLOW).expect("workflow");
    workflow.steps[0].depends_on = vec!["publish".to_owned()];
    assert!(validate(&workflow)
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.workflow.cycle"));
    workflow.steps[0].depends_on = vec!["missing".to_owned(), "missing".to_owned()];
    let report = validate(&workflow);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.step.missing-dependency"));
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.step.duplicate-dependency"));
}

#[test]
fn external_actions_require_both_controls_but_remain_non_executable() {
    let mut workflow = parse_workflow_json(WORKFLOW).expect("workflow");
    workflow.steps[2].approval.required = false;
    workflow.steps[2].approval.gate_id = None;
    workflow.steps[2].idempotency.required = false;
    workflow.steps[2].idempotency.key = None;
    let report = validate(&workflow);
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.external.approval-required"));
    assert!(report
        .diagnostics
        .iter()
        .any(|item| item.code == "constillo.external.idempotency-required"));
}

#[test]
fn workflow_json_is_closed_and_bounded() {
    let unknown = WORKFLOW.replace(
        "\"workflow_version\": \"1.0.0\",",
        "\"workflow_version\": \"1.0.0\", \"credential\": \"no\",",
    );
    assert!(parse_workflow_json(&unknown).is_err());
    assert!(parse_workflow_json(&" ".repeat(1_048_577)).is_err());
}
