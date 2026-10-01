use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use constillo::{
    parse_decision_authorization, parse_workflow_input_json, parse_workflow_json,
    DecisionAuthorization, Workflow, WorkflowInput,
};
use serde_json::{json, Value};

const MAX_DOCUMENT_BYTES: u64 = 1_048_576;

pub(super) fn load_workflow(path: &str) -> Result<Workflow, (u8, Value)> {
    let source = read_bounded_utf8(path, "workflow")?;
    parse_workflow_json(&source).map_err(|message| decode_error(&message))
}

pub(super) fn load_input(path: &str) -> Result<WorkflowInput, (u8, Value)> {
    let source = read_bounded_utf8(path, "workflow input")?;
    parse_workflow_input_json(&source).map_err(|message| decode_error(&message))
}

pub(super) fn load_decision(path: &str) -> Result<DecisionAuthorization, (u8, Value)> {
    let source = read_bounded_utf8(path, "decision authorization")?;
    parse_decision_authorization(&source).map_err(|message| decode_error(&message))
}

fn read_bounded_utf8(path: &str, entity: &str) -> Result<String, (u8, Value)> {
    let path = Path::new(path);
    let metadata = fs::symlink_metadata(path).map_err(|_| io_error(entity, "inspect"))?;
    if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
        return Err(io_error(entity, "accept only a regular non-symlink file"));
    }
    if metadata.len() > MAX_DOCUMENT_BYTES {
        return Err(io_error(entity, "respect the 1 MiB boundary"));
    }
    let file = File::open(path).map_err(|_| io_error(entity, "open"))?;
    let mut bytes = Vec::new();
    file.take(MAX_DOCUMENT_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| io_error(entity, "read"))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_DOCUMENT_BYTES {
        return Err(io_error(entity, "respect the 1 MiB boundary"));
    }
    String::from_utf8(bytes).map_err(|_| io_error(entity, "contain UTF-8"))
}

fn io_error(entity: &str, action: &str) -> (u8, Value) {
    (
        1,
        json!({
            "schema_version": "constillo.error/v1",
            "ok": false,
            "code": "constillo.io.read",
            "message": format!("{entity} file must {action}")
        }),
    )
}

fn decode_error(message: &str) -> (u8, Value) {
    (
        1,
        json!({
            "schema_version": "constillo.error/v1",
            "ok": false,
            "code": "constillo.json.decode",
            "message": message
        }),
    )
}
