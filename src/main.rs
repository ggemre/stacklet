mod external {
    pub mod exec;
    pub mod model;
    pub mod widget;
}
mod utils {
    pub mod args;
    pub mod fuzzy;
    pub mod helpers;
}
mod interface {
    pub mod window;
}
use crate::external::widget::Widget;
use crate::interface::window::BreakCondition;
use std::process::exit;

/// Entry point for the program.
fn main() {
    // parse commandline arguments
    let args = utils::args::parse_args();
    let exec_path: String;

    if args.help() {
        utils::args::print_help();
        exit(0);
    } else if args.version() {
        utils::args::print_version();
        exit(0);
    } else {
        match args.exec_path() {
            Some(path) => exec_path = (&path).to_string(),
            None => {
                println!("Error: Missing required argument -x/--exec");
                utils::args::print_help();
                exit(1);
            }
        }
    }

    // initialize runtime variables
    let mut input = String::from("");
    let mut input_content = String::from("");
    let mut selection = String::from("");
    let mut data = String::from("");

    loop {
        // run provided executable and collect ui model (from stdout) and generated data
        let (mut model, new_data) =
            external::exec::run_executable(&exec_path, &input, &input_content, &selection, &data);

        if model.is_empty() {
            // no stdout, end the app loop
            break;
        }

        if !new_data.is_empty() {
            // app used DATA() macro
            data = new_data;
        }

        let (break_condition, match_id) = interface::window::init(&mut model);

        if break_condition == BreakCondition::SELECTION {
            // user clicked on a text widget, set selection runtime variable for next execution
            if let Some(Widget::Text { content, .. }) = model.get(match_id) {
                selection = content.to_string();
                input = "".to_string();
            }
        } else if break_condition == BreakCondition::INPUT {
            // user clicked on an input widget, set input runtime variable for next execution
            if let Some(Widget::Input { content, .. }) = model.get(match_id) {
                selection = "".to_string();
                input = content.to_string();
            }
        } else if break_condition == BreakCondition::QUIT {
            break;
        }

        // generate a string of all input widgets deliminated by input_content_delimiter for next execution
        input_content = model
            .iter()
            .filter_map(|widget| match widget {
                Widget::Input { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(":"); // TODO: pull out to configurable option
    }

    interface::window::destroy();
}
