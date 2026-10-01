use serde::Serialize;

/// Deterministic topological proposal; it cannot be executed directly.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkflowPlan {
    pub schema_version: &'static str,
    pub workflow_id: String,
    pub workflow_version: String,
    pub mode: &'static str,
    pub executable: bool,
    pub steps: Vec<PlannedStep>,
}

/// One normalized step in a topological plan.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PlannedStep {
    pub sequence: usize,
    pub level: usize,
    pub step_id: String,
    pub action: String,
    pub depends_on: Vec<String>,
    pub external_action: bool,
    pub approval_gate: Option<String>,
    pub idempotency_key: Option<String>,
}
