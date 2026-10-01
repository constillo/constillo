use crate::diagnostics::diagnostic;
use crate::Diagnostic;

use super::references::{identifier, opaque, sha256};
use super::{ArtifactReference, DAILY_REPORT_SCHEMA, JSON_MEDIA_TYPE, MAX_ARTIFACT_BYTES};

pub(crate) fn validate(artifact: &ArtifactReference, diagnostics: &mut Vec<Diagnostic>) {
    identifier(&artifact.artifact_id, "$.artifact.artifact_id", diagnostics);
    if artifact.schema_id != DAILY_REPORT_SCHEMA {
        diagnostics.push(diagnostic(
            "constillo.input.artifact.unsupported-schema",
            "$.artifact.schema_id",
            format!("expected {DAILY_REPORT_SCHEMA}"),
        ));
    }
    if artifact.media_type != JSON_MEDIA_TYPE {
        diagnostics.push(diagnostic(
            "constillo.input.artifact.unsupported-media-type",
            "$.artifact.media_type",
            format!("expected {JSON_MEDIA_TYPE}"),
        ));
    }
    if !sha256(&artifact.digest_sha256) {
        diagnostics.push(diagnostic(
            "constillo.input.artifact.invalid-digest",
            "$.artifact.digest_sha256",
            "digest_sha256 must be exactly 64 lowercase hexadecimal characters",
        ));
    }
    if !(1..=MAX_ARTIFACT_BYTES).contains(&artifact.size_bytes) {
        diagnostics.push(diagnostic(
            "constillo.input.artifact.invalid-size",
            "$.artifact.size_bytes",
            format!("size_bytes must be between 1 and {MAX_ARTIFACT_BYTES}"),
        ));
    }
    opaque(&artifact.opaque_ref, "$.artifact.opaque_ref", diagnostics);
}
