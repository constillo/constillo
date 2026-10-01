use constillo::{parse_decision_authorization, parse_workflow_json, plan_authorized};

fn workflow() -> constillo::Workflow {
    parse_workflow_json(include_str!("../examples/workflow.authorized.json")).expect("workflow")
}

fn decision() -> constillo::DecisionAuthorization {
    parse_decision_authorization(include_str!("../examples/authorized-decision.json"))
        .expect("decision")
}

#[test]
fn receipt_binds_workflow_to_decision_without_execution() {
    let receipt = plan_authorized(&workflow(), &decision()).expect("authorized plan");
    assert_eq!(
        receipt.schema,
        "constillo.workflow-authorization-receipt.v1"
    );
    assert_eq!(receipt.decision_id, decision().decision_id);
    assert!(receipt.receipt_digest_sha256.starts_with("sha256:"));
    assert!(!receipt.executable);
    assert!(!receipt.external_actions);
}

#[test]
fn one_character_decision_tamper_is_rejected() {
    let mut changed = decision();
    changed.recommended_option_id = "bounded-pilotx".to_string();
    assert!(plan_authorized(&workflow(), &changed).is_err());

    let mut changed = decision();
    changed.decision_digest_sha256.replace_range(7..8, "a");
    assert!(plan_authorized(&workflow(), &changed).is_err());
}

#[test]
fn unknown_fields_and_unbound_recommendation_are_rejected() {
    let unknown =
        include_str!("../examples/authorized-decision.json").replacen('{', "{\"unknown\":true,", 1);
    assert!(parse_decision_authorization(&unknown).is_err());
    let mut unbound = workflow();
    unbound.steps[1].action = "record-other-decision".to_string();
    assert!(plan_authorized(&unbound, &decision()).is_err());
}
