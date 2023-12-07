use std::process::{Command, exit};
use crate::external::model::parse_stdout;
use crate::Widget;

pub fn run_executable(path: &str, input: &str, input_content: &str, selection: &str, data: &str) -> (Vec<Widget>, String) {
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
