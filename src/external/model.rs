use regex::Regex;
use crate::external::widget::{Filter, Widget};

pub fn parse_stdout(stdout: &str) -> (Vec<Widget>, String) {
    let mut widgets = Vec::new();
    let mut data = String::from("");

    let input_regex = Regex::new(r"INPUT\s*\((.*?)\)").unwrap();
    let param_regex = Regex::new(r#"(\w+)\s*=\s*\"?([^\",]+)\"?,?\s*"#).unwrap();
    let text_regex = Regex::new(r#"TEXT\("(.*)"\)"#).unwrap();
    let data_regex = Regex::new(r#"DATA\("(.*)"\)"#).unwrap();

    let mut level: i32 = 0;
    let mut unique_id: usize = 0;

    for line in stdout.lines() {
        if let Some(captures) = input_regex.captures(line) {
            let params_str = captures.get(1).unwrap().as_str();
            let mut max_width = 32;
            let mut filter = Filter::Off;
            let mut label = String::new();
            let mut placeholder = String::new();
            let mut content = String::new();

            for param_match in param_regex.captures_iter(params_str) {
                let param = param_match.get(1).unwrap().as_str();
                let value = param_match.get(2).unwrap().as_str();

                match param {
                    "max_width" => max_width = value.parse().unwrap_or(32),
                    "filter" => filter = value.parse().unwrap_or_else(|_| {
                        eprintln!("Invalid filter value: {}", &value.to_string());
                        std::process::exit(1); // TODO: add error utils module
                    }),
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
                id: unique_id,
            });
        } else if let Some(captures) = text_regex.captures(line) {
            let content: String = captures[1].to_string();
            widgets.push(Widget::Text { 
                y: level, 
                content,
                show: true,
                id: unique_id,
            });
        } else if let Some(captures) = data_regex.captures(line) {
            let content: String = captures[1].to_string();
            data = content.clone();
            level -= 1; // TODO: this is a temp fix for a later day...
            unique_id -= 1;
        } else if line == "QUIT()" {
            widgets.clear();
            break;
        }

        level += 1;
        unique_id += 1; // TODO: remove id & just use index
    }

    (widgets, data)
}

