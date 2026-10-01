//! Content-free acquisition handoff into the local workflow planner.

mod artifact_validation;
mod binding;
mod digest;
mod model;
mod policy_validation;
mod receipt;
mod references;
mod reports;
mod validation;

pub use binding::plan_input;
pub use model::{
    AcquisitionMode, ArtifactReference, ExecutionMode, InformationClassification, InputSource,
    InputSubject, SubjectKind, WorkflowInput,
};
pub use receipt::InputReceipt;
pub use reports::InputValidationReport;
pub use validation::validate_input;

use crate::json_input::parse_bounded;

/// Engine-neutral input wire schema.
pub const WORKFLOW_INPUT_SCHEMA: &str = "estate://contracts/workflow-input/v1";
/// Semantic input validation result schema.
pub const INPUT_VALIDATION_SCHEMA_VERSION: &str = "constillo.input-validation/v1";
/// Deterministic, non-executable acceptance receipt schema.
pub const INPUT_RECEIPT_SCHEMA_VERSION: &str = "constillo.input-receipt/v1";

pub(crate) const DAILY_REPORT_SCHEMA: &str = "estate://operations/department-daily-report/v1";
pub(crate) const JSON_MEDIA_TYPE: &str = "application/json";
pub(crate) const MAX_ARTIFACT_BYTES: u64 = 1_048_576;

/// Parses a bounded workflow-input JSON document with closed nested objects.
///
/// # Errors
///
/// Rejects documents above 1 MiB, invalid JSON, duplicate fields, and unknown
/// fields.
pub fn parse_workflow_input_json(source: &str) -> Result<WorkflowInput, String> {
    parse_bounded(source, "workflow input")
}
