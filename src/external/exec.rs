use std::process::{Command, exit};
use crate::external::model::parse_stdout;

use crate::Widget;

pub fn run_executable(path: &str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(&path)
        .output()
        .expect("Failed to run provided executable");

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout);
        let model = parse_stdout(&result);

        for widget in &model {
            match widget {
                Widget::InputWidget { content, .. } => {
                    println!("Input: {}", content);
                }
                Widget::TextWidget { content } => {
                    println!("Text: {}", content);
                }
            }
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("{}", error);
        exit(1);
    }
}
