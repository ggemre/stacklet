use crate::external::model::parse_stdout;
use crate::Widget;
use std::process::{exit, Command};

use fork::{daemon, Fork};

/// Run provided executable with the provided environment and parse the stdout.
///
/// Takes a path for the executable and runs via `sh` with the provided runtime variables.
/// Takes the stdout of the executable and passes to `external::model::parse_stdout` to generate the model
/// and new runtime variables.
pub fn run_executable(
    path: &str,
    input: &str,
    input_content: &str,
    selection: &str,
    data: &str,
) -> (Vec<Widget>, String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&path)
        .env("INPUT", &input)
        .env("INPUT_CONTENT", &input_content)
        .env("SELECTION", &selection)
        .env("DATA", &data)
        .output()
        .expect("Failed to run provided executable");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        return parse_stdout(&result);
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", error);
        exit(1);
    }
}

/// Fork a child process and run the given command on it.
pub fn spawn_detached_child(command: &str) {
    if let Ok(Fork::Child) = daemon(false, false) {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .expect("Failed to launch subprocess");
    }
}
