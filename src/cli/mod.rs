mod commands;
mod file_input;

use serde::Serialize;
use serde_json::Value;

pub(crate) fn run(arguments: &[String]) -> Result<Value, (u8, Value)> {
    commands::dispatch(arguments)
}

pub(crate) fn emit(value: &impl Serialize) {
    println!(
        "{}",
        serde_json::to_string_pretty(value).expect("command output serializes")
    );
}
