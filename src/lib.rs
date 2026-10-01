#![forbid(unsafe_code)]
#![doc = "Deterministic workflow validation and planning without an executor."]

mod authorization;
mod diagnostics;
mod graph;
mod identifiers;
mod json_input;
mod plan;
mod plan_model;
mod step_validation;
mod workflow_model;
mod workflow_validation;

mod input;

pub use authorization::{
    parse_decision_authorization, plan_authorized, DecisionAuthorization,
    WorkflowAuthorizationReceipt,
};
pub use diagnostics::{Diagnostic, ValidationReport};
pub use input::{
    parse_workflow_input_json, plan_input, validate_input, AcquisitionMode, ArtifactReference,
    ExecutionMode, InformationClassification, InputReceipt, InputSource, InputSubject,
    InputValidationReport, SubjectKind, WorkflowInput, INPUT_RECEIPT_SCHEMA_VERSION,
    INPUT_VALIDATION_SCHEMA_VERSION, WORKFLOW_INPUT_SCHEMA,
};
pub use json_input::parse_workflow_json;
pub use plan::plan;
pub use plan_model::{PlannedStep, WorkflowPlan};
pub use workflow_model::{ApprovalPolicy, IdempotencyPolicy, Workflow, WorkflowStep};
pub use workflow_validation::validate;

/// Accepted workflow contract.
pub const WORKFLOW_SCHEMA_VERSION: &str = "constillo.workflow/v1";
/// Deterministic, non-executable plan contract.
pub const PLAN_SCHEMA_VERSION: &str = "constillo.plan/v1";
