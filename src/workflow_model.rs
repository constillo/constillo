use serde::{Deserialize, Serialize};

/// Closed versioned workflow containing a bounded DAG.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
    pub schema_version: String,
    pub workflow_id: String,
    pub workflow_version: String,
    pub steps: Vec<WorkflowStep>,
}

/// One declarative action and its dependency/control metadata.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkflowStep {
    pub id: String,
    pub action: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    pub external_action: bool,
    pub approval: ApprovalPolicy,
    pub idempotency: IdempotencyPolicy,
}

/// Approval control required for any proposed external action.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ApprovalPolicy {
    pub required: bool,
    pub gate_id: Option<String>,
}

/// Replay control required for any proposed external action.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct IdempotencyPolicy {
    pub required: bool,
    pub key: Option<String>,
}
