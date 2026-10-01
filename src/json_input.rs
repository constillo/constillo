use serde::de::DeserializeOwned;

use crate::Workflow;

const MAX_DOCUMENT_BYTES: usize = 1_048_576;

pub(crate) fn parse_bounded<T: DeserializeOwned>(source: &str, label: &str) -> Result<T, String> {
    if source.len() > MAX_DOCUMENT_BYTES {
        return Err(format!("{label} exceeds 1 MiB"));
    }
    serde_json::from_str(source).map_err(|error| {
        format!(
            "invalid {label} JSON at line {}, column {}",
            error.line(),
            error.column()
        )
    })
}

/// Parses a bounded workflow JSON document with a closed Serde shape.
///
/// # Errors
///
/// Rejects documents above 1 MiB, invalid JSON, duplicate fields, and unknown
/// fields at every closed contract level.
pub fn parse_workflow_json(source: &str) -> Result<Workflow, String> {
    parse_bounded(source, "workflow")
}
