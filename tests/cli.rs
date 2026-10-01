use std::{
    path::PathBuf,
    process::{Command, Output},
};

use serde_json::Value;

fn run(arguments: &[&str]) -> Output {
    Command::new(binary_path())
        .args(arguments)
        .current_dir(std::env::current_dir().expect("package root"))
        .output()
        .expect("CLI starts")
}

// Resolve beside the running integration-test binary so a moved checkout does
// not retain a compile-time absolute path to its former Cargo target.
fn binary_path() -> PathBuf {
    std::env::current_exe()
        .expect("integration test executable")
        .parent()
        .and_then(|dependencies| dependencies.parent())
        .expect("Cargo target profile")
        .join(format!("constillo{}", std::env::consts::EXE_SUFFIX))
}

fn json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).expect("one JSON document")
}

#[test]
fn standalone_commands_are_deterministic_and_non_executing() {
    for arguments in [
        vec!["doctor"],
        vec!["validate", "examples/workflow.valid.json"],
        vec!["plan", "examples/workflow.valid.json"],
        vec!["validate-input", "examples/workflow-input.valid.json"],
        vec![
            "plan-input",
            "examples/workflow.input-plan.json",
            "examples/workflow-input.valid.json",
        ],
    ] {
        let first = run(&arguments);
        let second = run(&arguments);
        assert!(first.status.success(), "{arguments:?}");
        assert_eq!(first.stdout, second.stdout, "{arguments:?}");
    }
    let doctor = json(&run(&["doctor"]));
    assert_eq!(doctor["capabilities"]["external_execution"], false);
    assert_eq!(doctor["capabilities"]["network_access"], false);
    assert_eq!(doctor["capabilities"]["credential_storage"], false);
}

#[test]
fn unknown_command_has_a_stable_machine_readable_failure() {
    let output = run(&["unknown"]);
    assert_eq!(output.status.code(), Some(1));
    let value = json(&output);
    assert_eq!(value["code"], "constillo.cli.usage");
}
