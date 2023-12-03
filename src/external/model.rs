use regex::Regex;
use crate::external::widget::{Widget};

pub fn parse_stdout(stdout: &str) -> Vec<Widget> {
    let mut widgets = Vec::new();

    let input_regex = Regex::new(r"INPUT\((.*?)\)").unwrap();
    let param_regex = Regex::new(r"(\w+)=(\S+)").unwrap();
    let text_regex = Regex::new(r#"TEXT\("(.*)"\)"#).unwrap();

    for line in stdout.lines() {
        if let Some(captures) = input_regex.captures(line) {
            let params_str = captures.get(1).unwrap().as_str();
            let mut max_width = 32;
            let mut filter = false;
            let mut content = String::new();

            for param_match in param_regex.captures_iter(params_str) {
                let param = param_match.get(1).unwrap().as_str();
                let value = param_match.get(2).unwrap().as_str();

                match param {
                    "max_width" => max_width = value.parse().unwrap_or(32),
                    "filter" => filter = value.parse().unwrap_or(false),
                    "content" => content = value.to_string(),
                    _ => {}
                }
            }

            widgets.push(Widget::InputWidget {
                max_width,
                filter,
                content,
            });
        } else if let Some(captures) = text_regex.captures(line) {
            let content: String = captures[1].to_string();
            widgets.push(Widget::TextWidget { content });
        }
    }

    widgets
}

