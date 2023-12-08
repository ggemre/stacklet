mod external {
    pub mod exec;
    pub mod widget;
    pub mod model;
}
mod utils {
    pub mod args;
}
mod interface {
    pub mod window;
}
use crate::external::widget::Widget;
use crate::interface::window::BreakCondition;
use std::process::exit;

fn main() {
    let args = utils::args::parse_args();
    let exec_path: String ;

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

    let mut input = String::from("");
    let mut input_content = String::from("");
    let mut selection = String::from("");
    let mut data = String::from("");

    loop {
        let (mut model, new_data) = external::exec::run_executable(&exec_path, &input, &input_content, &selection, &data);

        if !new_data.is_empty() {
            data = new_data;
        }

        let (break_condition, y)= interface::window::init(&mut model);

        if break_condition == BreakCondition::SELECTION {
            if let Some(Widget::Text { content, .. }) = model.get(y) {
                selection = content.to_string();
                input = "".to_string();
            }
        } else if break_condition == BreakCondition::INPUT {
            if let Some(Widget::Input { content, .. }) = model.get(y) {
                selection = "".to_string();
                input = content.to_string();
            }
        } else if break_condition == BreakCondition::QUIT {
            break;
        }

        input_content = model
            .iter()
            .filter_map(|widget| match widget {
                Widget::Input { content, .. } => Some(content.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(":");
    }

    interface::window::destroy();
}
