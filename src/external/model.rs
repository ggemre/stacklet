use crate::external::exec;
use crate::external::widget::{Filter, Widget};
use regex::Regex;

/// Generate the model, (vector of ui widgets), and runtime data from the stdout of executable.
///
/// ## UI Model creation
///
/// The stdout is parsed to generate a ui model according to the following BNF form:
///   <input> ::= INPUT '(' <param_list> ')'
///   <param_list> ::= <param> | <param>, <param_list>
///   <param> ::= <param_name> '=' <param_value>
///   <param_name> ::= [a-zA-Z_][a-zA-Z0-9_]*
///   <param_value> ::= '"' <string_content> '"'
///   <text> ::= TEXT '(' '"' <string_content> '"' ')'
///   <data> ::= DATA '(' '"' <string_content> '"' ')'
///   <quit> ::= QUIT '(' ')' | QUIT '(' '"' <string_content> '"' ')'
///   <string_content> ::= [^"]*
///
/// ## Runtime data generation
///
/// Data is set by the DATA() widget and returned accordingly.
/// Since the program quits if no stdout is provided, QUIT() clears the model, (as if there was no stdout), and returns.
pub fn parse_stdout(stdout: &str) -> (Vec<Widget>, String) {
    let mut widgets = Vec::new();
    let mut data = String::from("");

    // regex for widgets, see BNF form above
    let input_regex = Regex::new(r"INPUT\s*\((.*?)\)").unwrap();
    let param_regex = Regex::new(r#"(\w+)\s*=\s*\"?([^\",]+)\"?,?\s*"#).unwrap();
    let text_regex = Regex::new(r#"TEXT\("(.*)"\)"#).unwrap();
    let data_regex = Regex::new(r#"DATA\("(.*)"\)"#).unwrap();
    let quit_regex = Regex::new(r#"QUIT\("([^"]*)"\)"#).unwrap();

    let mut level: i32 = 0;
    let mut unique_id: usize = 0;

    // match up each line of stdout with widget regex
    for line in stdout.lines() {
        if let Some(captures) = input_regex.captures(line) {
            // found input widget, initialize parameters to default values
            let params_str = captures.get(1).unwrap().as_str();
            let mut max_width = 32;
            let mut filter = Filter::Off;
            let mut label = String::new();
            let mut placeholder = String::new();
            let mut content = String::new();

            for param_match in param_regex.captures_iter(params_str) {
                let param = param_match.get(1).unwrap().as_str();
                let value = param_match.get(2).unwrap().as_str();

                // set parameter variable for each valid parameter
                match param {
                    "max_width" => max_width = value.parse().unwrap_or(32),
                    "filter" => {
                        filter = value.parse().unwrap_or_else(|_| {
                            eprintln!("Invalid filter value: {}", &value.to_string());
                            std::process::exit(1); // TODO: add error utils module
                        })
                    }
                    "label" => label = value.to_string(),
                    "placeholder" => placeholder = value.to_string(),
                    "content" => content = value.to_string(),
                    _ => {}
                }
            }

            // add a new input widget to the model
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
            // found text widget, identify its text content
            let content: String = captures[1].to_string();

            // add a new text widget to the model
            widgets.push(Widget::Text {
                y: level,
                content,
                show: true,
                id: unique_id,
            });
        } else if let Some(captures) = data_regex.captures(line) {
            // found data widget, set the data variable to its content
            let content: String = captures[1].to_string();
            data = content.clone();
            level -= 1; // TODO: this is a temp fix for a later day...
            unique_id -= 1;
        } else if let Some(captures) = quit_regex.captures(line) {
            // found quit widget, clear the model and stop reading stdout
            if let Some(content) = captures.get(1) {
                let command = content.as_str().to_string();
                exec::spawn_detached_child(&command);
            }
            widgets.clear();
            break;
        }

        level += 1;
        unique_id += 1; // TODO: remove id & just use index
    }

    (widgets, data)
}
