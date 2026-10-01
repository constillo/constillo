use serde::Serialize;

use crate::Diagnostic;

use super::INPUT_VALIDATION_SCHEMA_VERSION;

/// Complete semantic result for one workflow input.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct InputValidationReport {
    pub schema_version: &'static str,
    pub request_id: String,
    pub workflow_id: String,
    pub valid: bool,
    pub diagnostics: Vec<Diagnostic>,
}

impl InputValidationReport {
    pub(crate) fn new(
        request_id: String,
        workflow_id: String,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            schema_version: INPUT_VALIDATION_SCHEMA_VERSION,
            request_id,
            workflow_id,
            valid: diagnostics.is_empty(),
            diagnostics,
        }
    }
}
