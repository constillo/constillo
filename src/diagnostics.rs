use serde::{Deserialize, Serialize};

/// Stable machine-readable validation finding.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    pub code: String,
    pub path: String,
    pub message: String,
}

/// Complete fail-closed workflow validation result.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ValidationReport {
    pub schema_version: &'static str,
    pub workflow_id: String,
    pub valid: bool,
    pub diagnostics: Vec<Diagnostic>,
}

pub(crate) fn diagnostic(
    code: impl Into<String>,
    path: impl Into<String>,
    message: impl Into<String>,
) -> Diagnostic {
    Diagnostic {
        code: code.into(),
        path: path.into(),
        message: message.into(),
    }
}

pub(crate) fn sort(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|left, right| {
        (&left.path, &left.code, &left.message).cmp(&(&right.path, &right.code, &right.message))
    });
}
