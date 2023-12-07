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

    let model = external::exec::run_executable(&exec_path);
    for widget in &model {
        match widget {
            Widget::Input { content, .. } => {
                println!("Input: {}", content);
            }
            Widget::Text { content, .. } => {
                println!("Text: {}", content);
            }
        }
    }

    interface::window::init(model);
}
