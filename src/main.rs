mod cli;

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match cli::run(&arguments) {
        Ok(value) => {
            cli::emit(&value);
            ExitCode::SUCCESS
        }
        Err((code, value)) => {
            cli::emit(&value);
            ExitCode::from(code)
        }
    }
}
