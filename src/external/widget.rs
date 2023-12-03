#[derive(Debug)]
pub enum Widget {
    InputWidget { 
        max_width: usize,
        filter: bool,
        content: String, 
    },
    TextWidget { content: String }
}

#[derive(Debug)]
pub struct Input {
    pub max_width: usize,
    pub filter: bool,
    pub content: String,
}

#[derive(Debug)]
pub struct Text {
    pub content: String,
}

