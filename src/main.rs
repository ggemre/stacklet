use stacklet::external::exec;
use stacklet::external::widget::Widget;
use stacklet::interface::window::{destroy, init, BreakCondition};
use stacklet::utils::args;
use std::process::exit;

/// Entry point for the program.
fn main() {
    // parse commandline arguments
    let args = args::parse_args();
    let exec_path: String;

    if args.help() {
        args::print_help();
        exit(0);
    } else if args.version() {
        args::print_version();
        exit(0);
    } else {
        match args.exec_path() {
            | Some(path) => exec_path = (&path).to_string(),
            | None => {
                println!("Error: Missing required argument -x/--exec");
                args::print_help();
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
        let (mut model, new_data) = exec::run_executable(
            &exec_path,
            &input,
            &input_content,
            &selection,
            &data,
        );

        if model.is_empty() {
            // no stdout, end the app loop
            break;
        }

        if !new_data.is_empty() {
            // app used DATA() macro
            data = new_data;
        }

        // for widget in model {
        //     match widget {
        //         | Widget::Input {
        //             y, selectable, ..
        //         } => {
        //             println!("{}", selectable);
        //         }
        //         | Widget::Text {
        //             y, selectable, ..
        //         } => {
        //             println!("{}", selectable);
        //         }
        //     }
        // }
        // exit(1);

        let (break_condition, match_id) = init(&mut model);

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
                | Widget::Input { content, .. } => Some(content.clone()),
                | _ => None,
            })
            .collect::<Vec<_>>()
            .join(":"); // TODO: pull out to configurable option
    }

    destroy();
}
