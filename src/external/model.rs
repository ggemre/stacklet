use regex::Regex;
use crate::external::widget::{Widget};

pub fn parse_stdout(stdout: &str) -> (Vec<Widget>, String) {
    let mut widgets = Vec::new();
    let mut data = String::from("");

    let input_regex = Regex::new(r"INPUT\s*\((.*?)\)").unwrap();
    let param_regex = Regex::new(r#"(\w+)\s*=\s*\"?([^\",]+)\"?,?\s*"#).unwrap();
    let text_regex = Regex::new(r#"TEXT\("(.*)"\)"#).unwrap();
    let data_regex = Regex::new(r#"DATA\("(.*)"\)"#).unwrap();

    let mut level: usize = 0;

    for line in stdout.lines() {
        if let Some(captures) = input_regex.captures(line) {
            let params_str = captures.get(1).unwrap().as_str();
            let mut max_width = 32;
            let mut filter = false;
            let mut label = String::new();
            let mut placeholder = String::new();
            let mut content = String::new();

            for param_match in param_regex.captures_iter(params_str) {
                let param = param_match.get(1).unwrap().as_str();
                let value = param_match.get(2).unwrap().as_str();

                match param {
                    "max_width" => max_width = value.parse().unwrap_or(32),
                    "filter" => filter = value.parse().unwrap_or(false),
                    "label" => label = value.to_string(),
                    "placeholder" => placeholder = value.to_string(),
                    "content" => content = value.to_string(),
                    _ => {}
                }
            }

            widgets.push(Widget::Input { 
                y: level,
                max_width,
                filter,
                label,
                placeholder,
                content,
            });
        } else if let Some(captures) = text_regex.captures(line) {
            let content: String = captures[1].to_string();
            widgets.push(Widget::Text { 
                y: level, 
                content 
            });
        } else if let Some(captures) = data_regex.captures(line) {
            let content: String = captures[1].to_string();
            data = content.clone();
            level -= 1; // TODO: this is a temp fix for a later day...
        }

        level += 1;
    }

    (widgets, data)
}

