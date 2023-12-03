mod external {
    pub mod exec;
    pub mod widget;
    pub mod model;
}
use crate::external::widget::Widget;

fn main() {
    println!("Hello, world!");
    external::exec::run_executable("./test.sh");
    // let widgets = external::model::create_test();
    // for widget in &widgets {
    //     match widget {
    //         Widget::InputWidget { content } => {
    //             println!("Input: {}", content);
    //         }
    //         Widget::TextWidget { content } => {
    //             println!("Text: {}", content);
    //         }
    //     }
    // }
}
