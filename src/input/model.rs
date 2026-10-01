use serde::{Deserialize, Serialize};

/// Content-free acquisition receipt bound to a target workflow.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkflowInput {
    pub schema: String,
    pub request_id: String,
    pub correlation_id: String,
    pub workflow_id: String,
    pub workflow_version: String,
    pub source: InputSource,
    pub subject: InputSubject,
    pub artifact: ArtifactReference,
    pub classification: InformationClassification,
    pub customer_data: bool,
    pub commission_ref: String,
    pub decision_ref: Option<String>,
    pub idempotency_key: String,
    pub execution_mode: ExecutionMode,
    pub external_actions: bool,
}

/// Adapter identity and opaque acquisition receipt.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputSource {
    pub service_id: String,
    pub acquisition_mode: AcquisitionMode,
    pub receipt_ref: String,
}

/// Supported acquisition boundary; neither mode executes inside Constillo.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AcquisitionMode {
    MountedFolder,
    ProviderApi,
}

/// Organizational scope used to bind downstream policy.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputSubject {
    pub kind: SubjectKind,
    pub id: String,
    pub headquarters_id: String,
    pub department_id: String,
    pub team_id: String,
}

/// Accepted content contract family.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SubjectKind {
    DepartmentDailyReport,
}

/// Digest and opaque locator for an artifact that Constillo never reads.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactReference {
    pub artifact_id: String,
    pub schema_id: String,
    pub media_type: String,
    pub digest_sha256: String,
    pub size_bytes: u64,
    pub opaque_ref: String,
}

/// Information boundary carried into workflow policy.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum InformationClassification {
    Internal,
    InternalConfidential,
    PersonalConfidential,
    RestrictedSensitive,
    CustomerConfidential,
}

/// Planning intent; runtime remains unavailable in this package.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionMode {
    Simulation,
    DryRun,
    Runtime,
}
